;;; eam-persistent.el --- Explicit persistent terminal sessions -*- lexical-binding: t; -*-
(require 'eam-worktree)
(require 'eam-native)
(defconst eam-persistent--code
  (expand-file-name "native/" eam--resource-directory))
(require 'eam-persistent-events)
(defvar eam-persistent--readers (make-hash-table :test #'equal))
(defvar-local eam-persistent--timer nil)
(defvar-local eam-persistent--display-state nil)

(defun eam-persistent--call (action request)
  "Invoke only an explicit local management operation."
  (with-temp-buffer
    (insert (json-encode request))
    (let ((status (call-process-region
                   (point-min) (point-max) (eam-native--executable)
                   t t nil "manager" action)))
      (unless (eq status 0) (error "%s" (buffer-string)))
      (goto-char (point-min))
      (json-parse-buffer :object-type 'alist :array-type 'list :null-object nil))))

(defun eam-persistent--cancel ()
  (when eam-persistent--timer
    (cancel-timer eam-persistent--timer)
    (setq eam-persistent--timer nil)))

(defun eam-persistent--poll (buffer reader)
  (if (not (buffer-live-p buffer)) nil
    (with-current-buffer buffer
      (if (not (process-live-p (eam-terminal-process eam-terminal--current)))
          (progn
            (setq eam-persistent--display-state
                  (condition-case nil
                      (let* ((info (eam-persistent--call
                                    "inspect" `((session . ,eam-terminal-persistent-directory))))
                             (state (alist-get 'status info)))
                        (if (equal (alist-get 'state state) "stopped")
                            (format "terminated%s"
                                    (let ((code (alist-get 'exit_code state)))
                                      (if (and code (not (equal code "")))
                                          (format "(%s)" code) "")))
                          "disconnected"))
                    (error "disconnected (unverified)")))
            (eam-persistent--cancel)
            (unless (string-prefix-p "terminated" eam-persistent--display-state)
              (setq eam-persistent--timer
                    (run-at-time 1 1 #'eam-persistent--poll buffer reader)))
            (setq header-line-format
                  (concat "PERSISTENT | " eam-persistent--display-state
                          " | C-c a h 기록 / C-c a d 분리"))
            (when (string-prefix-p "terminated" eam-persistent--display-state)
              (let* ((session eam-terminal--current)
                     (draft (eam-terminal-input session))
                     (directory eam-terminal-persistent-directory))
                ;; Automatic cleanup must not discard unsaved user text.
                (when (and (buffer-live-p draft) (buffer-modified-p draft))
                  (setf (eam-terminal-input session) nil)
                  (message "CLI ended; unsaved draft retained in %s" (buffer-name draft)))
                (remhash directory eam-persistent--readers)
                (eam-detach))))
        (condition-case err (eai-event-reader-poll reader)
          (error (eam-persistent--cancel)
                 (message "Persistent events stopped: %s" (error-message-string err))))))))

(defun eam-persistent--wait-attachment (session directory token)
  "Confirm this display's PID, not another client's endpoint, within four seconds."
  (let ((file (expand-file-name "editor.json" directory))
        (process (eam-terminal-process session))
        (deadline (+ (float-time) 4)) ready)
    (while (and (not ready) (process-live-p process) (< (float-time) deadline))
      (let ((attrs (file-attributes file)))
        (when (and attrs (null (file-attribute-type attrs))
                   (<= (file-attribute-size attrs) 4096))
          (condition-case nil
              (let ((endpoint (with-temp-buffer
                                (insert-file-contents file nil 0 4097)
                                (json-parse-buffer :object-type 'alist :false-object nil))))
                (setq ready (and (eq (alist-get 'connected endpoint) t)
                                 (equal (alist-get 'attachment_pid endpoint) (process-id process))
                                 (equal (alist-get 'attachment_token endpoint) token))))
            (file-error nil))))
      (unless ready (accept-process-output process .025)))
    (unless (and ready (process-live-p process))
      (user-error "Persistent display connection failed or timed out; CLI was not restarted"))))

;;;###autoload
(defun eam-attach (&optional directory)
  "Select a detached CLI to attach.  DIRECTORY is for programmatic callers.
The running CLI is never restarted."
  (interactive)
  (if (null directory)
      (progn (require 'eam-app) (eam-sessions t))
    (eam-persistent--attach directory)))

(defun eam-persistent--attach (directory)
  "Attach to the existing CLI stored in DIRECTORY."
  (setq directory (expand-file-name directory))
  (let* ((info (eam-persistent--call "inspect" `((session . ,directory))))
         (metadata (alist-get 'metadata info))
         (state (alist-get 'status info)))
    (unless (and (equal (alist-get 'state state) "running")
                 (zerop (alist-get 'attached_clients state)))
      (user-error "Session is not running or is already attached"))
    (eam-terminal--editor-command)
    (let ((configuration (current-window-configuration))
          (token (secure-hash 'sha256 (prin1-to-string (list (emacs-pid) (current-time) (random)))))
          session success)
      (unwind-protect
          (progn
            (setq session
                  (eam-terminal-start
                   (alist-get 'provider metadata) (eam-native--executable)
                   (list "attach" directory
                         (eam-terminal--emacsclient)
                         (expand-file-name server-name server-socket-dir) token)
                   (alist-get 'directory metadata)))
            ;; Redraw is display data, never another write to the canonical log.
            (set-process-filter (eam-terminal-process session) #'ghostel--filter)
            (setf (eam-terminal-file session) (alist-get 'archive metadata))
            (eam-persistent--wait-attachment session directory token)
            (let* ((buffer (eam-terminal-output session))
                   (reader (or (gethash directory eam-persistent--readers)
                               (eai-event-reader-open (alist-get 'events metadata) buffer))))
              (with-current-buffer buffer
                (setq-local eam-terminal-persistent-directory directory)
                (setq-local eam-persistent--display-state nil)
                (setq-local ghostel-notification-function nil)
                (setq-local header-line-format
                            (format "PERSISTENT %s | M-x eam-detach / eam-quit"
                                    "PTY"))
                (add-hook 'kill-buffer-hook #'eam-persistent--cancel nil t)
                (setq eam-persistent--timer
                      (run-at-time 0 .1 #'eam-persistent--poll buffer reader)))
              ;; Commit reader and notice ownership after fallible setup.
              (let ((previous (eai-event-reader-buffer reader)))
                (dolist (notice eam-notifications--entries)
                  (when (eq previous (eam-notice-buffer notice))
                    (setf (eam-notice-buffer notice) buffer))))
              (setf (eai-event-reader-buffer reader) buffer)
              (puthash directory reader eam-persistent--readers))
            (eam-persistent--set-label session (alist-get 'name metadata))
            (setq success t)
            session)
        (unless success
          (when (and session (buffer-live-p (eam-terminal-output session)))
            (with-current-buffer (eam-terminal-output session)
              (eam-persistent--cancel)
              (eam-detach)))
          (set-window-configuration configuration))))))

(defun eam-persistent--set-label (session name)
  "Apply NAME to SESSION display buffers without changing provider or identity."
  (let ((name (or name "")))
    (with-current-buffer (eam-terminal-output session)
      (rename-buffer (format "*%s terminal: %s%s*"
                             (eam-terminal-name session)
                             (if (string-empty-p name) "" (concat name " | "))
                             (abbreviate-file-name (eam-terminal-directory session))) t)
      (setq-local header-line-format
                  (concat "PERSISTENT PTY | " name " | M-x eam-detach / eam-quit")))
    (when (buffer-live-p (eam-terminal-input session))
      (with-current-buffer (eam-terminal-input session)
        (rename-buffer (eam-terminal--draft-name session) t)))))

(defun eam-persistent--start (provider directory extra-args &optional name)
  "Create a persistent CLI with EXTRA-ARGS for native conversation resume."
  (setq provider (eam--provider-name provider))
  (let* ((parent (expand-file-name "persistent" eam-directory))
         (directory (expand-file-name directory))
         (session (expand-file-name (format "%s-%s" (downcase provider)
                                           (format-time-string "%Y%m%d-%H%M%S-%N")) parent)))
    (make-directory parent t)
    (eam-persistent--call
     "start" `((session . ,session) (provider . ,provider)
               (backend . "pty") (name . ,(or name ""))
               (executable . ,(eam--executable (downcase provider)))
               (raw_recording . ,(if eam-record-terminal t :json-false))
               (args . ,(vconcat (append (eam-notifications-cli-args provider) extra-args)))
               (directory . ,directory)))
    ;; Startup is asynchronous. Wait only for recorder readiness, not AI output.
    (let ((deadline (+ (float-time) 3)))
      (while (and (not (file-exists-p (expand-file-name "events.jsonl" session)))
                  (< (float-time) deadline))
        (accept-process-output nil .05)))
    (condition-case err (eam-attach session)
      (error (message "Persistent session saved at %s" session)
             (signal (car err) (cdr err))))))

;;;###autoload
(defun eam-quit (directory)
  "Explicitly stop the selected owned server, retaining its records."
  (interactive (list (or eam-terminal-persistent-directory
                         (read-directory-name "Persistent session to stop: "))))
  (when (yes-or-no-p (format "Stop persistent CLI in %s? " directory))
    (let ((result (eam-persistent--call
                   "stop" `((session . ,(expand-file-name directory))))))
      (unless (equal (alist-get 'state result) "stopped")
        (user-error "Stop could not be verified (%s); inspect with M-x eam-status"
                    (or (alist-get 'state result) "unknown")))
      (remhash (expand-file-name directory) eam-persistent--readers)
      (message "Persistent CLI server stopped; disk records retained"))))
;;;###autoload
(defun eam-status (directory)
  "Show verified local process metadata without starting or reattaching a CLI."
  (interactive (list (or eam-terminal-persistent-directory
                         (read-directory-name "Persistent session: "))))
  (let ((info (eam-persistent--call "inspect" `((session . ,(expand-file-name directory))))))
    (with-help-window "*Persistent session status*"
      (princ (format "Session: %s\n\n%s\n" directory (pp-to-string info))))))

(defun eam-persistent--guard-worktree (directory)
  "Include detached persistent sessions in the worktree deletion check."
  (eam-persistent--call
   "guard-worktree" `((root . ,(expand-file-name "persistent" eam-directory))
                      (directory . ,directory))))
(add-hook 'eam-worktree-before-remove-hook #'eam-persistent--guard-worktree)
(provide 'eam-persistent)
;;; eam-persistent.el ends here

;;; pty-backend-test.el --- Default PTY sessions through public EAM -*- lexical-binding: t; -*-
(require 'ert)
(require 'eam-app)
(defconst eam-pty-backend-test-root
  (expand-file-name ".." (file-name-directory (or load-file-name buffer-file-name))))
(defun eam-pty-backend-test-wait (predicate)
  (let ((deadline (+ (float-time) 7)))
    (while (and (not (funcall predicate)) (< (float-time) deadline))
      (accept-process-output nil .025))
    (should (funcall predicate))))
(ert-deftest eam-pty-backend-default-new-detach-attach-quit ()
  (let* ((eam-directory (make-temp-file "eam-pty-product-" t))
         (eam-record-terminal nil)
         (eam-persistent--readers (make-hash-table :test #'equal))
         (eam-notifications--entries nil)
         (fixture (or (getenv "EAM_NATIVE_FIXTURE")
                      (expand-file-name "var/native-target/debug/examples/fixture" eam-pty-backend-test-root)))
         session path runtime draft)
    (unwind-protect
        (progn
          (cl-letf (((symbol-function 'eam--executable) (lambda (_name) fixture))
                    ((symbol-function 'eam-notifications-cli-args)
                     (lambda (_provider)
                       (list "terminal"
                             (expand-file-name "received.jsonl" eam-directory)))))
            (setq session (eam-new "Claude" eam-directory)))
          (setq path (buffer-local-value 'eam-terminal-persistent-directory (eam-terminal-output session)))
          (let* ((info (eam-persistent--call "inspect" `((session . ,path))))
                 (metadata (alist-get 'metadata info)))
            (setq runtime (alist-get 'runtime metadata))
            (should (equal (alist-get 'backend metadata) "pty"))
            (should-not (alist-get 'archive metadata))
            (should (= 1 (alist-get 'attached_clients (alist-get 'status info)))))
          (eam-pty-backend-test-wait
           (lambda () (with-current-buffer (eam-terminal-output session)
                        (string-match-p "READY" (buffer-string)))))
          (with-current-buffer (eam-terminal-output session)
            (should-not (ghostel--mouse-tracking-p ghostel--term))
            (eam-detach))
          (eam-pty-backend-test-wait
           (lambda () (zerop (alist-get 'attached_clients
                                       (alist-get 'status (eam-persistent--call "inspect" `((session . ,path))))))))
          (should (= 1 (length (alist-get 'sessions (eam-persistent--call "list-live"
                                                    `((root . ,(expand-file-name "persistent" eam-directory))))))))
          (setq session (eam-attach path))
          (eam-pty-backend-test-wait
           (lambda () (with-current-buffer (eam-terminal-output session)
                        (string-match-p "READY" (buffer-string)))))
          (with-current-buffer (eam-terminal-output session)
            (setq draft (eam-terminal--ensure-draft session))
            (with-current-buffer draft (insert "한글 PTY 입력"))
            (eam-paste))
          (eam-pty-backend-test-wait
           (lambda () (with-current-buffer (eam-terminal-output session)
                        (string-match-p "PASTED 한글 PTY 입력" (buffer-string)))))
          (with-current-buffer (eam-terminal-output session) (eam-edit-input))
          (eam-pty-backend-test-wait
           (lambda () (seq-find (lambda (b) (with-current-buffer b
                                             (and buffer-file-name (string-suffix-p ".editor.txt" buffer-file-name))))
                                (buffer-list))))
          (let ((editor (seq-find (lambda (b) (with-current-buffer b
                                               (and buffer-file-name (string-suffix-p ".editor.txt" buffer-file-name))))
                                 (buffer-list))))
            (with-current-buffer editor
              (goto-char (point-max)) (insert " 수정")
              (call-interactively (key-binding (kbd "C-c C-c")))))
          (eam-pty-backend-test-wait
           (lambda () (with-current-buffer (eam-terminal-output session)
                        (string-match-p "EDITED 한글 PTY 입력 수정" (buffer-string)))))
          (cl-letf (((symbol-function 'yes-or-no-p) (lambda (&rest _) t)))
            (eam-quit path))
          (should (equal "stopped" (alist-get 'state (alist-get 'status
                                      (eam-persistent--call "inspect" `((session . ,path))))))))
      (when path (ignore-errors (eam-persistent--call "stop" `((session . ,path)))))
      (when (and session (buffer-live-p (eam-terminal-output session)))
        (with-current-buffer (eam-terminal-output session) (eam-detach)))
      (when (buffer-live-p draft) (with-current-buffer draft (set-buffer-modified-p nil)) (kill-buffer draft))
      (when runtime (delete-directory runtime t))
      (delete-directory eam-directory t))))

(ert-deftest eam-pty-backend-detached-natural-exit ()
  (let* ((root (make-temp-file "eam-pty-exit-" t))
         (path (expand-file-name "session" root)) metadata runtime)
    (unwind-protect
        (progn
          (setq metadata (eam-persistent--call "start"
                          `((session . ,path) (provider . "Fake") (backend . "pty")
                            (directory . ,root) (executable . ,(or (getenv "EAM_NATIVE_FIXTURE")
                                              (expand-file-name "var/native-target/debug/examples/fixture" eam-pty-backend-test-root)))
                            (raw_recording . t)
                            (args . ["exit"]))))
          (setq runtime (alist-get 'runtime metadata))
          (eam-pty-backend-test-wait
           (lambda () (equal "stopped" (alist-get 'state (alist-get 'status
                                          (eam-persistent--call "inspect" `((session . ,path))))))))
          (should-not (file-exists-p (expand-file-name "socket" runtime)))
          (should (equal 0 (alist-get 'exit_code (alist-get 'status
                              (eam-persistent--call "inspect" `((session . ,path)))))))
          (with-temp-buffer
            (insert-file-contents (alist-get 'archive metadata))
            (should (string-match-p "한글 종료" (buffer-string)))))
      (when metadata (ignore-errors (eam-persistent--call "stop" `((session . ,path)))))
      (when runtime (delete-directory runtime t))
      (delete-directory root t))))

(ert-deftest eam-pty-backend-idle-window-width-propagates ()
  "Resize an idle Ghostel window through attach, daemon and recorder PTYs."
  (save-window-excursion
    (let* ((root (make-temp-file "eam-pty-size-" t))
           (eam-directory root)
           (path (expand-file-name "session" root))
           (sizes (expand-file-name "width" root))
           metadata runtime session window)
      (unwind-protect
          (progn
            (setq metadata
                  (eam-persistent--call "start"
                   `((session . ,path) (provider . "Fake") (backend . "pty")
                     (directory . ,root) (executable . ,(or (getenv "EAM_NATIVE_FIXTURE")
                                              (expand-file-name "var/native-target/debug/examples/fixture" eam-pty-backend-test-root)))
                     (args . ["size"]))))
            (setq runtime (alist-get 'runtime metadata)
                  session (eam-attach path)
                  window (get-buffer-window (eam-terminal-output session)))
            (select-window window)
            (delete-other-windows)
            (dolist (split '(t nil))
              (if split
                  (set-window-buffer (split-window-right) (get-buffer-create "*scratch*"))
                (delete-other-windows))
              (with-current-buffer (eam-terminal-output session)
                (ghostel--adjust-size window t)
                (let ((expected ghostel--term-cols))
                  (eam-pty-backend-test-wait
                   (lambda ()
                     (and (file-exists-p sizes)
                          (= expected (with-temp-buffer
                                        (insert-file-contents sizes)
                                        (string-to-number (buffer-string)))))))))))
        (when metadata (ignore-errors (eam-persistent--call "stop" `((session . ,path)))))
        (when (and session (buffer-live-p (eam-terminal-output session)))
          (with-current-buffer (eam-terminal-output session) (eam-detach)))
        (when runtime (delete-directory runtime t))
        (delete-directory root t)))))

(ert-deftest eam-pty-backend-cli-quit-cleans-attached-buffer ()
  "CLI quit closes its display, even when a helper retains the slave PTY."
  (dolist (mode '("echo" "inherited-pty"))
    (let* ((root (make-temp-file "eam-pty-cli-quit-" t))
           (eam-directory root)
           (path (expand-file-name "session" root))
           (eam-persistent--readers (make-hash-table :test #'equal))
           session runtime draft)
      (unwind-protect
          (cl-letf (((symbol-function 'yes-or-no-p)
                     (lambda (&rest _) (ert-fail "Automatic CLI exit prompted"))))
            (let ((v (eam-persistent--call "start"
                      `((session . ,path) (provider . "Fake") (backend . "pty")
                        (directory . ,root) (executable . ,(getenv "EAM_NATIVE_FIXTURE"))
                        (args . [,mode])))))
              (setq runtime (alist-get 'runtime v)))
            (setq session (eam-attach path))
            (with-current-buffer (eam-terminal-output session)
              (setq draft (eam-terminal--ensure-draft session)))
            (with-current-buffer draft (insert "보존할 미전송 입력"))
            (process-send-string (eam-terminal-process session) "q")
            (eam-pty-backend-test-wait
             (lambda () (not (buffer-live-p (eam-terminal-output session)))))
            (should (equal "stopped" (alist-get 'state (alist-get 'status
                                     (eam-persistent--call "inspect" `((session . ,path)))))))
            (should-not (alist-get 'sessions (eam-persistent--call "list-live" `((root . ,root)))))
            (should (buffer-live-p draft))
            (with-current-buffer draft
              (should (equal "보존할 미전송 입력" (buffer-string)))))
        (ignore-errors (eam-persistent--call "stop" `((session . ,path))))
        (when (and session (buffer-live-p (eam-terminal-output session)))
          (with-current-buffer (eam-terminal-output session) (eam-detach)))
        (when (buffer-live-p draft)
          (with-current-buffer draft (set-buffer-modified-p nil)) (kill-buffer draft))
        (when runtime (delete-directory runtime t))
        (server-force-delete)
        (delete-directory root t)))))

;;; eam-caffeine.el --- Explicit macOS idle sleep prevention -*- lexical-binding: t; -*-
(defvar eam-caffeine--process nil)
(defvar eam-caffeine-mode nil)

(defun eam-caffeine--stop ()
  "Release only the assertion process started by this Emacs."
  (when (process-live-p eam-caffeine--process)
    (delete-process eam-caffeine--process))
  (setq eam-caffeine--process nil))

;;;###autoload
(define-minor-mode eam-caffeine-mode
  "Manually prevent macOS idle system sleep while this Emacs lives.
Display sleep remains allowed. Disabled by default; no CLI/model state polling."
  :global t :init-value nil :lighter " Caffeine"
  (if eam-caffeine-mode
      (condition-case err
          (progn
            (unless (and (eq system-type 'darwin) (file-executable-p "/usr/bin/caffeinate"))
              (user-error "Caffeine requires macOS /usr/bin/caffeinate"))
            (unless (process-live-p eam-caffeine--process)
              (setq eam-caffeine--process
                    (make-process
                     :name "eam-caffeine" :buffer nil :noquery t
                     :command (list "/usr/bin/caffeinate" "-i" "-w" (number-to-string (emacs-pid)))
                     :sentinel
                     (lambda (process _event)
                       (when (and (eq process eam-caffeine--process)
                                  (not (process-live-p process)))
                         (setq eam-caffeine--process nil eam-caffeine-mode nil)
                         (force-mode-line-update t))))))
            (add-hook 'kill-emacs-hook #'eam-caffeine--stop))
        (error
         (setq eam-caffeine-mode nil)
         (eam-caffeine--stop)
         (signal (car err) (cdr err))))
    (remove-hook 'kill-emacs-hook #'eam-caffeine--stop)
    (eam-caffeine--stop))
  (force-mode-line-update t))

(provide 'eam-caffeine)

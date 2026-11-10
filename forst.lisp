#!/usr/bin/env -S sbcl --script

(require 'uiop)

(defun tree (dir)
  (labels ((walk (d prefix)
             (dolist (p (uiop:subdirectories d))
               (format t "~a|-- ~a~%" prefix
                       (car (last (pathname-directory p))))
               (walk p (concatenate 'string prefix "|   ")))
             (dolist (p (uiop:directory-files d))
               (format t "~a|-- ~a~%" prefix (file-namestring p)))))
    (format t "~a~%" (car (last (pathname-directory dir))))
    (walk dir "")))

(tree ".")

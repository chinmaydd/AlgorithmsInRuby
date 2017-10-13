(define (a-plus-abs-b a b)
  ((if (> b 0) + -) a b))
; a + |b|
; The 'if' expression returns a function rather than a value

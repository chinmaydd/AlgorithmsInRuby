;; Problem 10
(ns clj.p10)

;; http://mishadoff.com/blog/clojure-euler-problem-010/
;; Implementation of sieve of erathosnes in Clojure
(defn sieve 
  [lim]
  (loop [nums (set (cons 2 (range 3 lim 2))) n 3]
    ;; Optimization is to start the sieve at n^2 rather than n. 
    (if (> (* n n) lim) 
        (reduce + nums)
        (recur (clojure.set/difference nums (range (* n n) lim n)) (+ n 2)))))


(defn soln
  "The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million."
  [n]
  (sieve n))

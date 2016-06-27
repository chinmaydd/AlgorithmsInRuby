;; Problem 9
(ns clj.p9
  (:refer-clojure :exclude [==])
  (:require [clojure.core.logic :refer :all]
             [clojure.core.logic.fd :as fd]))

(defn soln 
  "A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
  
  a2 + b2 = c2
  For example, 32 + 42 = 9 + 16 = 25 = 52.
  
  There exists exactly one Pythagorean triplet for which a + b + c = 1000.
  Find the product abc."
  [sum]
  ;; We need the quantity a*b*c
  (reduce * (first
        ;; Solving it with core.logic. Symbolic Execution ftw!
        (run* [q]
        ;; Declare 3 fresh symbolic lvars, or variables
        (fresh [a b c]
        ;; Resulting solution should be of the form [a b c]
        (== q [a b c])
        ;; All the numbers should lie between 1 and sum/2. This is a rather broad approximation 
        (fd/in a b c (fd/interval 1 (/ sum 2)))
        ;; All the three numbers should be distinct, otherwise the pythagorean triplet rule does not hold
        (fd/distinct [a b c])
        ;; We now specify the constraints on those 3 variables
        (fd/eq
          ;; sum = a + b + c
          (= sum (+ a b c))
          ;; c^2 = a^2 + b^2
          (= (* c c) (+ (* a a) (* b b)))))))))

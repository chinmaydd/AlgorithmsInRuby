;; Problem 6
(ns clj.p6
  [:require [clojure.math.numeric-tower :as math]])

;; Square of sums
(defn square-of-sums
  [n]
  (math/expt (/ (* n (+ n 1)) 2) 2))

;; Sum of squares
(defn sum-of-squares
  [n]
  (/ (* n (+ n 1) (+ (* 2 n) 1)) 6))

(defn soln
  "The sum of the squares of the first ten natural numbers is,
  
  12 + 22 + ... + 102 = 385
  The square of the sum of the first ten natural numbers is,
  
  (1 + 2 + ... + 10)2 = 552 = 3025
  Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
  
  Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum."
  [n]
  (- (square-of-sums n) (sum-of-squares n)))

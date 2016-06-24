;; Problem 5
(ns clj.p5
  [:require [clojure.math.numeric-tower :as math]])

(defn soln
  "2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
  
  What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?"
  [n]
  (reduce math/lcm (range 1 n))) 

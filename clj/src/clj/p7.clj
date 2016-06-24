;; Problem 7
(ns clj.p7
  [:require [clojure.math.numeric-tower :as math]])

;; Check if a number is prime
(defn is-prime?
  [n]
  (or (= 2 n)
      (not-any? #(= 0 (mod n %)) (range 3 (inc (math/sqrt n)) 2))))

;; Building up an array of primes so that we only check against those. Useful for bigger computations 
(defn is-prime-with-sieve?
  [n]
  (let [root (-> n
                 (math/sqrt)
                 inc
                 int)]
    (loop [i 3 prime-array [2]]
      (if (>= i root)
        (not-any? #(= 0 (mod n %)) prime-array)
        (recur (+ i 2) (if (not-any? #(= 0 (mod i %)) prime-array)
                           (conj prime-array i)
                           prime-array))))))

;; Can be further optimized by inclusion of the sieve of erathosnes
(defn soln
  "By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
  
  What is the 10 001st prime number?"
  [n]
  (last (take n (filter #(is-prime? %) (cons 2 (iterate (partial + 2) 3))))))

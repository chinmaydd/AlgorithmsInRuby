;; Problem 4
(ns clj.p4)

;; Initial approach
(defn explode-to-digits [number]
  (into [] (map #(Character/getNumericValue %) (str number))))

(defn palindrome? [number]
  (= (reverse (explode-to-digits number)) (explode-to-digits number)))

;; Optimized approach: StackOverflow http://stackoverflow.com/a/9148336
(defn is-palindrome? [num]
  (= (str num) (clojure.string/reverse (str num))))

(defn soln
    "A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

    Find the largest palindrome made from the product of two 3-digit numbers."
    (reduce max (filter is-palindrome? (for [x (reverse (range 100 1000)) y (reverse (range 100 x))] (* x y)))))
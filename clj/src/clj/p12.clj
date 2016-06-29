;; Problem 12
(ns clj.p12
  (:require [clojure.math.numeric-tower :as math])
  )

(defn find-factor
  [n]
  (->>
    (range 2 (inc n))
    (drop-while #(not= 0 (rem n %)))
    (first)
    ))

(defn factors
  [n]
  (loop [n n
        factors #{}]
    (if (nil? n)
      factors
      (let [next (find-factor n)
            q    (if (nil? next) nil (/ n next))]
      (recur q (conj factors next))))))

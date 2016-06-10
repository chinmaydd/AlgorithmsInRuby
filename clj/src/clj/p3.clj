;; Problem 3
(ns clj.p3)

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

(defn soln [n]
  "The prime factors of 13195 are 5, 7, 13 and 29.

    What is the largest prime factor of the number 600851475143 ?"
  (last (sort (clojure.set/difference (factors n) #{nil}))))

(soln 600851475143)
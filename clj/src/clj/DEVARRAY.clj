(require '[clojure.string :as str])
(defn read-into-list [] (map read-string (str/split (read-line) #" ")))
(defn read-into-int  [] (read-string (read-line)))

(def params (read-into-list))
(def input (read-into-list))

(def minimum (apply min input))
(def maximum (apply max input))

(for [x (range (nth params 1))]
  (if (<= minimum (read-into-int) maximum) (println "Yes") (println "No")))
;; An implmementation of the problem of solving dimensional analysis functionally 
;; as described in Peter Henderson's Functional Programming and applications...
;; .. wait for it.. in Clojure!
(ns clj.dimensional-analysis
  (:require [schema.core :as s]
            [instaparse.core :as insta]))

;; The solution to the problem has been defined from a programming and a testing point of view,
;; where the program is broken down into functions hence making testing a lot more easier since
;; we only have to test and check for their core logic. This is a concept which is taught to the 
;; novice programmer(myself, LUL) through the text. We can try and understand it as a conveyor belt
;; on which the data is operated upon by various "functions" and then the final output is produced.
;; It then becomes intuitive to test for and design in cases where you know what "tools" you already
;; have to operate on your data.

;; Now, the author starts with the definition of dimensions, quantities and dimensional expressions.
;; Definitons:
;; Dimension: Quantities in the real world are associated with dimensions. They are usually expressed in
;;            terms of fundamental units -> M(Mass), Length(L) and Time(T).
;; Quantity: Can be a real life quantity which has a value. It's dimensions can be descibed in terms of M, L, T.
;; Dimensional expressions: Expressions on which dimensional analysis can be applied on. They contain quantities
;;                          with operations on them.
;;

;; Reference: http://plumatic.github.io/schema-for-clojurescript-data-shape-declaration-and-validation/
;; Let us define Schemas for all the quantities we will be dealing with.
;; Why are schemas useful in functional programming?
;; Pure functions: FREE OF SIDE EFFECTS. In layman terms, they do what they are supposed to, and no mo.
;; Hence it is pivotal to understand what the function inputs and corresponding outputs to understand its behaviour.
;; It would have been useful if we had types instead, but .. you know, whatever.
;; Schema also helps in runtime data validation and checking if the function "contract" is not broken.

;; Defining schemas!
(def Dimension
  {:M s/Int 
   :L s/Int
   :T s/Int
   })

(def Atom
  {:Type      (s/enum :variable :constant)
   :Symbol    s/Str
   :Dimension Dimension})

(def Constant
  {:M 0
   :L 0
   :T 0})

(def Assoc-List
  {:v {:Type :variable
       :Symbol "v"
       :Dimension {:M 0
                   :L 1
                   :T -1}}
   :a {:Type :variable
       :Symbol "a"
       :Dimension {:M 0
                   :L 1
                   :T -2}}
   :u {:Type :variable
       :Symbol "u"
       :Dimension {:M 0
                   :L 1
                   :T -1}}
   :t {:Type :variable
       :Symbol "b"
       :Dimension {:M 0
                   :L 0
                   :T 1}}})

(def Operator
  (s/enum :add :subtract :divide :multiply))

(def Expression
  {:LHS (s/conditional #(= nil (:Type %)) (s/recursive #'Expression) :else Atom)
   :RHS (s/conditional #(= nil (:Type %)) (s/recursive #'Expression) :else Atom)
   :Operator Operator})

(def UNDEF
  "BAD VALUE")

(defn product
  [dim1 dim2]
  (cond
    (= UNDEF dim1) UNDEF
    (= UNDEF dim2) UNDEF
    :else {:M (+ (:M dim1) (:M dim2))
           :L (+ (:L dim1) (:L dim2))
           :T (+ (:T dim1) (:T dim2))}))

(defn ratio
  [dim1 dim2]
  (cond
    (= UNDEF dim1) UNDEF
    (= UNDEF dim2) UNDEF
    :else {:M (- (:M dim1) (:M dim2))
           :L (- (:L dim1) (:L dim2))
           :T (- (:T dim1) (:T dim2))}))

(defn dim
  [expr]
  (let [lhs           (:LHS expr)
        rhs           (:RHS expr)
        operator      (:Operator expr)
        expr-type     (:Type expr)]
    (cond
      (= expr-type :variable)  (:Dimension expr)
      (= expr-type :constant)  Constant
      (= operator :add)        (if (= (dim lhs) (dim rhs))
                                 (dim lhs)
                                 UNDEF)
      (= operator :subtract)   (if (= (dim lhs) (dim rhs))
                                 (dim lhs)
                                 UNDEF)
      (= operator :multiply)   (product (dim lhs) (dim rhs))
      (= operator :divide)     (ratio   (dim lhs) (dim rhs)))))

(def arithmetic2
  (insta/parser
    "expr       = add-sub
    <add-sub>   = mul-div | add | sub 
    add         = add-sub <'+'> mul-div
    sub         = add-sub <'-'> mul-div
    <mul-div>   = pterm | term | mul | div
    mul         = mul-div <'*'> term
    div         = mul-div <'/'> term
    term        = number | <'('> add-sub <')'>
    number      = #'[0-9]+'
    coefficient = #'[0-9]*'
    <symbol>    = #'[a-zA-Z]'
    sym         = symbol+
    pterm       = coefficient sym"
    :output-format :enlive))

;; The transformation function
(defn transform-map [m fm]
  (into {} (map (fn [[k v]]
                  [k (apply (first v)
                            ((apply juxt (rest v)) m))]) fm)))

(defn replace-coefficient
  [value]
  (let [parsed-value (read-string value)]
    {:Type :constant
     :Symbol parsed-value
     :Dimension Constant}))

(defn find-sym
  [sym]
  (let [sym-key (keyword sym)
        sym-value (sym-key Assoc-List)]
    (if (not-empty sym-value)
      sym-value
      "UNDEF")))  

(defn replace-sym
  [& sym]
  (case (count sym)
  1 (find-sym (first sym)) 
  2 {:Operator :multiply
      :LHS      (find-sym (first sym))
      :RHS      (find-sym (last sym))
     }
  {:Operator :multiply
   :LHS      (first sym)
   :RHS      (replace-sym (rest sym))}))

(defn replace-pterm
  [coeff-term symbol-term]
  {:Operator :multiply
   :LHS coeff-term
   :RHS symbol-term})

(defn replace-add
  [term1 term2]
  {:Operator :add
   :LHS term1
   :RHS term2})

(defn replace-sub
  [term1 term2]
  {:Operator :subtract
   :LHS term1
   :RHS term2})

(defn replace-mul
  [term1 term2]
  {:Operator :multiply
   :LHS term1
   :RHS term2})

(defn replace-div
  [term1 term2]
  {:Operator :divide
   :LHS term1
   :RHS term2})

(defn replace-term
  [& terms]
  Constant)

(def parse-tree->schema-def
  {:sym         replace-sym
   :coefficient replace-coefficient
   :add         replace-add
   :sub         replace-sub
   :div         replace-div
   :term        replace-term
   :pterm       replace-pterm 
   :expr        identity})

;(defn validate-equation
  ;[]
  ;(let [raw-input      (read-line)
        ;parsed-result  (parse-equation raw-input)
        ;is-valid?      (not-empty parsed-result)]
    ;(if is-valid?
      ;; (create-expression parsed-result)
      ;parsed-result
      ;(println "Please enter a valid expression."))))

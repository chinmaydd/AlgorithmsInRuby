;; An implmementation of the the problem of solving dimensional analysis functionally 
;; as described in Peter Henderson's Functional Programming and applications...
;; .. wait for it.. in Clojure!
(ns clj.dimensional-analysis
  (:require [schema.core :as s]))

;; The solution to the problem has been defined from a programming and a testing point of view,
;; where the program is broken down into functions and testing them becomes a lot more easier since
;; we only have to test and check for their core logic. This is a concept which is taught to the 
;; novice (myself, LUL) through the text. We can try and understand the concept as a conveyor belt
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
(s/Dimension
  {:M s/Int 
   :L s/Int
   :T s/Int
   })

(s/defschema Atom
  {Type (s/enum :variable :constant)
   Dimension})

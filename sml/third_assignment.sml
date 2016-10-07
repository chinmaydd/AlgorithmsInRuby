(* Coursera Programming Languages, Homework 3, Provided Code *)

exception NoAnswer

(**** you can put all your code here ****)

(* Problem 1 *)
(* Using concepts mentioned ahead in the course (val binding) *)
val only_capitals = List.filter (fn x => Char.isUpper(String.sub(x, 0)))

(* Problem 2 *)
fun longest_string1 xs = foldl (fn (x, acc) => if String.size x > String.size acc
                                               then x else acc) "" xs

(* Problem 3 *)
fun longest_string2 xs = foldl (fn (x, acc) => if String.size x >= String.size acc 
                                               then x else acc) "" xs

(* Helper function for problem 4 *)
fun longest_string_helper g = fn y => foldl (fn (x, acc) => if g (String.size x,
                                          String.size acc) then x else acc) "" y

(* Problrm 4a *)
fun longest_string3 xs = longest_string_helper (fn (a, b) => a > b) xs

(* Problem 4b *)
fun longest_string4 xs = longest_string_helper (fn (a,b) => a >= b) xs

(* Problem 5 *)
val longest_capitalized = longest_string1 o only_capitals

(* Problem 6 *)
val rev_string = String.implode o List.rev o String.explode

(* Problem 7 *)
(* This can be done better, I think. *)
fun first_answer g = fn x =>
let val res = List.map g x
    val ans = List.find (fn z => isSome z) res
in
  if isSome ans then valOf(valOf(ans)) else raise NoAnswer
end

(* Problem 8 *)
fun all_answers g = fn x =>
let val res = List.map g x
    val none_exists = List.exists (fn z => not(isSome z)) res
in
  if none_exists
  then NONE
  else SOME(foldr (fn (x, acc) => valOf(x) @ acc) [] res)
end

(** Given code for problems 9- **)

datatype pattern = Wildcard
     | Variable of string
     | UnitP
     | ConstP of int
     | TupleP of pattern list
     | ConstructorP of string * pattern

datatype valu = Const of int
        | Unit
        | Tuple of valu list
        | Constructor of string * valu

fun g f1 f2 p =
  let val r = g f1 f2 
  in
  case p of
      Wildcard          => f1 ()
    | Variable x        => f2 x
    | TupleP ps         => List.foldl (fn (p,i) => (r p) + i) 0 ps
    | ConstructorP(_,p) => r p
    | _                 => 0
    end

(* Problem 9a *)
fun count_wildcards patt = g (fn x => 1) (fn y => 0) patt

(* Problem 9b *)
fun count_wild_and_variable_lengths patt = g (fn x => 1) (fn y => String.size y)
  patt

(* Problem 9c *)
fun count_some_var (str, patt) = g (fn x => 0) (fn y => if y = str then 1 else 0) 
  patt

(* Helper function to get all variables in a pattern *)
fun get_all_variables patt = 
case patt of 
     Variable x         => x :: []
   | TupleP ps          => List.foldl (fn (p,i) => get_all_variables p @ i) [] ps
   | ConstructorP(_, p) => get_all_variables p
   | _                  => []

(* Helper function to check if the list contains duplicates. Inspired from SO *)
fun check_if_duplicates [] = false
  | check_if_duplicates (x::xs) = (List.exists (fn y => x = y) xs) orelse
  (check_if_duplicates xs)

(* Problem 10 *)
val check_pat  = not o check_if_duplicates o get_all_variables

(*datatype pattern = Wildcard
     | Variable of string
     | UnitP
     | ConstP of int
     | TupleP of pattern list
     | ConstructorP of string * pattern

datatype valu = Const of int
        | Unit
        | Tuple of valu list
        | Constructor of string * valu
        *)

(* Problem 11 *)
fun match inp = 
  case inp of
       (_, Wildcard)         => SOME([])
     | (v, Variable s)       => SOME((s, v) :: [])
     | (Unit, UnitP)         => SOME([])
     | (Const x, ConstP y)   => if x = y then SOME([]) else NONE
     | (Tuple vs, TupleP ps) => if length vs = length ps
                                then all_answers match (ListPair.zip (vs, ps))
                                else NONE
     | (Constructor(s2, v), ConstructorP(s1, p)) => if s1 = s2
                                                    then match (v, p)
                                                    else NONE
     | (_, _) => NONE

fun first_match v plist =
  SOME (first_answer (fn p => match(v, p)) plist) handle NoAnswer => NONE

(**** for the challenge problem only ****)
datatype typ = Anything
       | UnitT
       | IntT
       | TupleT of typ list
       | Datatype of string

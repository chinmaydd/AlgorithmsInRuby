(* Dan Grossman, Coursera PL, HW2 Provided Code *)

(* if you use this function to compare two strings (returns true if the same
   string), then you avoid several of the functions in problem 1 having
   polymorphic types that may be confusing *)
fun same_string(s1 : string, s2 : string) =
    s1 = s2

(* put your solutions for problem 1 here *)

(* Solution to problem (a) *)
fun all_except_option tuple = 
  case tuple of
       (_, []) => NONE
     | (str, hd::[])   => if same_string (str, hd) then SOME [] else NONE
     | (str, hd::rest) => if same_string (str, hd)
                          then SOME rest
                          else 
                            let val rem_result = all_except_option (str, rest) 
                            in case rem_result of
                                 NONE => NONE
                               | SOME remaining => SOME (hd::remaining)
                            end

(* Solution to problem (b) *)
fun get_substitutions1 x =
  case x of
       ([], _) => []
    | (hd::rest, str) => let val result = all_except_option (str, hd)
                         in case result of
                             NONE => get_substitutions1 (rest, str)
                           | SOME remaining => remaining @ 
                               get_substitutions1 (rest, str)
                         end

(* Solution to problem (c) *)
fun get_substitutions2 (string_list_list, str) = 
(* Let us assume it takes (string_list, str, acc) *)
let fun helper_substitution x =
  case x of
       ([], _, acc) => acc
     | (hd::rest, str, acc) => let val result = all_except_option (str, hd)
                               in case result of
                                   NONE => helper_substitution (rest, str, acc)
                                 | SOME remaining => 
                                     helper_substitution (rest, str, acc @
                                     remaining)
                               end
  in
    helper_substitution(string_list_list, str, [])
end

(* Solution to problem (d) *)
fun similar_names (string_list_list, {first=f, last=l, middle=mid}) =
let val subs = get_substitutions1 (string_list_list, f)
    fun helper_similar x =
      case x of
           ([], acc) => acc
         | (hd::rest, acc) => 
             helper_similar(rest, acc @ [{first=hd, middle=mid, last=l}])
in
  helper_similar(subs, [{first=f, last=l, middle=mid}])
end

(* you may assume that Num is always used with values 2, 3, ..., 10
   though it will not really come up *)
datatype suit = Clubs | Diamonds | Hearts | Spades
datatype rank = Jack | Queen | King | Ace | Num of int 
type card = suit * rank

datatype color = Red | Black
datatype move = Discard of card | Draw 

exception IllegalMove

(* put your solutions for problem 2 here *)

(* Solution to problem (a) *)
fun card_color (suit, rank) = 
  case suit of
       Hearts => Red
     | Diamonds => Red
     | _ => Black

(* Solution to problem (b) *)
fun card_value (suit, rank) =
  case rank of 
       Ace => 11
     | Num num_rank => num_rank
     | _ => 10

(* Generic helper function to check if the given first argument is in a list *)
(* Exactly similar to the all_except_given function except the function used for
* comparison of the head with the given member *)
fun all_except_given tuple = 
  case tuple of
       (_, []) => NONE
     | (c, hd::[])   => if c = hd then SOME [] else NONE
     | (c, hd::rest) => if c = hd
                          then SOME rest
                          else 
                            let val rem_result = all_except_given (c, rest) 
                            in case rem_result of
                                 NONE => NONE
                               | SOME remaining => SOME (hd::remaining)
                            end

(* Solution to problem (c) *)
fun remove_card (cs, c, e) =
let val given_result = all_except_given (c, cs)
in
  case given_result of
       NONE => raise e
     | SOME rest => rest
end

(* Solution to problem (d) *)
fun all_same_color lst =
  case lst of
       [] => true
     | (hd::[]) => true
     | (hd::neck::rest) => if card_color (hd) = card_color (neck)
                           then all_same_color (neck::rest)
                           else false

(* Solution to problem (e) *)
fun sum_cards card_list =
let fun helper_sum_cards x =
  case x of
       ([], acc) => acc
     | (hd::rest, acc) => helper_sum_cards (rest, acc + card_value (hd))
in
  helper_sum_cards (card_list, 0)
end

(* Solution to problem (f) *)
fun score (card_list, goal) = 
let val card_sum = sum_cards (card_list)
    val prelim_score = if card_sum > goal
                       then 3 * (card_sum - goal)
                       else (goal - card_sum)
in
   if all_same_color card_list
   then prelim_score div 2
   else prelim_score
end

(* Solution to problem (g) *)
fun officiate (card_list, move_list, goal) = 
let val held_cards = []
    val current_score = 0
    (* ntuple = ((rem)moves, (rem)cards, held_cards, goal *)  
    fun execute_move ntuple = 
      case ntuple of
           (* No more moves *)
           ([], _, held_cards, goal) => score (held_cards, goal)
           (* Draw but no more cards *)
         | (Draw::rem_moves, [], held_cards, goal) => score (held_cards, goal)
           (* Card discard *)
         | (Discard c::rem_moves, rem_cards, held_cards, goal) => 
             execute_move (rem_moves, rem_cards, 
                           remove_card (held_cards, c, IllegalMove), goal)
           (* Card draw *)
         | (Draw::rem_moves, hd::rem_cards, held_cards, goal) =>  
             if sum_cards (hd::held_cards) > goal 
             then score (hd::held_cards, goal)
             else execute_move (rem_moves, rem_cards, hd::held_cards, goal)
in
  execute_move (move_list, card_list, [], goal)
end

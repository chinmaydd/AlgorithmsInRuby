(* Solutions for the first programming homework assignment *)

(* Problem 1 *)
(* (YYYY, MM, DD) *)
fun is_older(x: int*int*int, y: int*int*int): bool = 
  (* Check is year is greater. *)
  if #1 x > #1 y
  then false
  (* Chceck is year is smaller. *)
  else if #1 x < #1 y
  then true
  (* If year is neither greater nor smaller, we can conclude that they are the
  * same *)
  (* Check is the month if greater. *)
  else if #2 x > #2 y
  then false
  (* Check if the month is smaller. *)
  else if #2 x < #2 y
  then true
  (* If the month and the year are same, the dates differ in the day. *)
  (* Check if the day is larger. *)
  else if #3 x > #3 y
  then false
  (* Check if the day is smaller. *)
  else if #3 x < #3 y
  then true
  (* Since all the conditions have failed, we can conclude that the dates are
  * the same and hence return false as per the problem statement. *)
  else false

(* Helper function for problem 2 *)
(* Checks if the date lies in the given month. If it does, function returns 1,
* else 0. *)
fun check_for_month(x: int*int*int, y: int): int = 
  if #2 x = y
  then 1
  else 0

(* Problem 2 *)
fun number_in_month(x: (int*int*int) list, y: int): int = 
(* Defining a recursive function. *)
let fun count_dates(x: (int*int*int) list, y: int, count: int) : int = 
      (* If there are no more elements left in the dates list, we return
      * the current count *)
      if null x
      then count
      (* Else, we recurse on the rest of the list incrementing the count if the
      * date is present. *)
      else count_dates(tl x, y, count + check_for_month(hd x, y))
in
  count_dates(x, y, 0)
end

(* Problem 3 *)
fun number_in_months(x: (int*int*int) list, y: int list): int =
(* Defining a recursive function for the same as well. *)
let fun count_dates(x: (int*int*int) list, y: int list, count: int) : int = 
      (* If there are no more elements left in the 'months' list, return the
      * current count. *)
      if null y
      then count
      (* Else, we will recurse on the rest of the 'month' list incrementing the
      * count based on the number of dates present in the month pointed by the
      * head. *)
      else count_dates(x, tl y, count + number_in_month(x, hd y))
in
  count_dates(x, y, 0)
end

(* Problem 4 *)
fun dates_in_month(x: (int*int*int) list, y: int): (int*int*int) list = 
(* Defining a recursive function for the same. *)
let fun list_dates(x: (int*int*int) list, y: int, date_list: (int*int*int) list) = 
      (* If the dates list is empty, we return the current date_list which
      * contains all the dates which belong to the given month. *)
      if null x
      then date_list
      (* Let us check if the head of the date list is present in the given
      * month. If it is present, we add it to the date_list. *)
      else if check_for_month(hd x, y) = 1
      then list_dates(tl x, y, date_list @ [hd x])
      (* Else we just recurse on the rest of the list. *)
      else list_dates(tl x, y, date_list)
in
  list_dates(x, y, [])
end

(* Problem 5 *)
fun dates_in_months(x: (int*int*int) list, y: int list): (int*int*int)
  list = 
(* Defining a recursive function. *)
let fun list_dates(x: (int*int*int) list, y: int list, date_list: (int*int*int)
list) =
      (* If all the months in the list are empty, we return the current
      * date_list. *)
      if null y
      then date_list
      (* Else, we recurse on the rest of the month list calling the earlier
      * dates_in_month function on the head and appending it to the current
      * date_list. *)
      else list_dates(x, tl y, date_list @ dates_in_month(x, hd y))
in
  list_dates(x, y, [])
end

(* Problem 6 *)
fun get_nth(x: string list, n: int): string = 
(* Defining a recursive function *)
let fun find_nth(x: string list, n: int, current: int) : string = 
      (* If the current head element has the index of the requested element, we
      * return it *)
      if current = n
      then hd x
      (* Else, we recurse on the rest of the list. *)
      else find_nth(tl x, n, current+1)
in
  find_nth(x, n, 1)
end

(* Problem 7 *)
fun date_to_string(x: (int*int*int)): string = 
let val month_list = ["January ", "February ", "March ", "April ", "May ", 
"June ", "July ", "August ", "September ", "October ", "November ", "December "]
in
  get_nth(month_list, #2 x) ^ Int.toString(#3 x) ^ ", " ^ Int.toString(#1 x)
end

(* Problem 8 *)
fun number_before_reaching_sum(x: int, y: int list): int =
let fun reach_sum(x: int, y: int list, current_sum: int, index: int) = 
      if current_sum + hd y >= x
      then index
      else reach_sum(x, tl y, current_sum + hd y, index + 1)
in
  reach_sum(x, y, 0, 0)
end

(* Problem 9 *)
fun what_month(x: int): int = 
let val month_count = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
in
  number_before_reaching_sum(x, month_count) + 1
end

(* Problem 10 *)
fun month_range(x: int, y: int): int list = 
  if x > y
  then []
  else let val month_list = []
           fun generate_month_list(x: int, y: int, month_list: int list): int
             list = 
             if x = y
             then month_list @ [what_month y]
             else generate_month_list(x + 1, y, month_list @ [what_month x])
       in
         generate_month_list(x, y, [])
       end

(* Problem 11 *)
fun oldest(x: (int*int*int) list): (int*int*int) option =
  if null x
  then NONE
  else let fun current_oldest(x: (int*int*int) list, old: (int*int*int)
  option): (int*int*int) option =
              if null x
              then old
              else if isSome old
              then if is_older(hd x, valOf old)
                   then current_oldest(tl x, SOME (hd x))
                   else current_oldest(tl x, old)
              (* First case when old is NONE *)
              else current_oldest(tl x, SOME (hd x))
       in
         current_oldest(x, NONE)
       end

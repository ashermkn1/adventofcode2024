
(* Anything helper functions that would be imported for each module *)
let split xs size =
  let (_, r, rs) =
    (* fold over the list, keeping track of how many elements are still
       missing in the current list (csize), the current list (ys) and
       the result list (zss) *) 
    List.fold_left (fun (csize, ys, zss) elt ->
      (* if target size is 0, add the current list to the target list and
         start a new empty current list of target-size size *)
      if csize = 0 then (size - 1, [elt], zss @ [ys])
      (* otherwise decrement the target size and append the current element
         elt to the current list ys *)
      else (csize - 1, ys @ [elt], zss))
      (* start the accumulator with target-size=size, an empty current list and
         an empty target-list *)
        (size, [], []) xs
  in
  (* add the "left-overs" to the back of the target-list *)
  rs @ [r]
let print_endline_int i = print_endline (Int.to_string i)

let time f =
  let before = Unix.gettimeofday () in
  let result = f () in
  let after = Unix.gettimeofday () in
  print_endline (Printf.sprintf "%f" (after -. before)) ;
  result

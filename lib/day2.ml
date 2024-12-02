open! Imports
open Core

module M = struct
  (* Type to parse the input into *)
  type t = int list list

  (* Parse the input to type t, invoked for both parts *)
  let parse inputs =
    inputs |> String.split_lines
    |> List.map ~f:(fun line ->
           line
           |> String.split_on_chars ~on:[' ']
           |> List.map ~f:Int.of_string )

  let sliding_window ~window_size lst =
    if window_size <= 0 then invalid_arg "Window size must be positive"
    else if window_size > List.length lst then []
    else
      lst
      |> List.foldi ~init:[] ~f:(fun i acc _ ->
             if i <= List.length lst - window_size then
               List.take (List.drop lst i) window_size :: acc
             else acc )
      |> List.rev

  let diff_check a b =
    let dist = Int.abs (a - b) in
    dist >= 1 && dist <= 3

  let xor a b = (a || b) && not (a && b)

  
  let safe levels =
    let up, down, range =
      List.fold (sliding_window ~window_size:2 levels)
        ~init:(true, true, true) ~f:(fun (up, down, range) window ->
          match window with
          | [a; b] -> (up && a < b, down && a > b, range && diff_check a b)
          | _ -> failwith "unreachable" )
    in
    xor up down && range
  (* Run part 1 with parsed inputs *)
  let part1 reports =
    let res = List.count ~f:safe reports in
    printf "Part 1: %d\n" res ; ()

  (* Run part 2 with parsed inputs *)
  let part2 reports =
    let remove l i = List.filteri l ~f:(fun i' _ -> i' <> i) in
    let res =
      List.count
        ~f:(fun levels ->
          List.existsi levels ~f:(fun i _ -> safe (remove levels i)) )
        reports
    in
    printf "Part 2: %d\n" res ; ()
end

include M
include Day.Make (M)

(* Example input *)
let example = ""

(* Expect test for example input *)
let%expect_test _ = run example ; [%expect {| |}]

open! Imports
open Core

module M = struct
  (* Type to parse the input into *)
  type t = int list list

  (* Parse the input to type t, invoked for both parts *)
  let parse inputs =
    let lines = String.split_on_chars ~on:['\n'] inputs in
    List.map
      ~f:(fun line ->
        let split = String.split_on_chars ~on:[' '] line in
        List.map split ~f:Int.of_string )
      lines

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

  (* Run part 1 with parsed inputs *)
  let safe levels =
    let res, _ =
      List.fold (sliding_window ~window_size:2 levels) ~init:(true, 0)
        ~f:(fun (res, ord) -> function
        | [a; b] ->
            if not res then (res, ord)
            else
              let ord_check, ord2 =
                match (ord, Int.compare a b) with
                | _, 0 -> (false, 0)
                | 1, -1 -> (false, 0)
                | -1, 1 -> (false, 0)
                | 0, new_ord -> (res, new_ord)
                | x, y when Int.equal x y -> (res, ord)
                | _ -> (res, ord)
              in
              (ord_check && diff_check a b, ord2)
        | _ -> failwith "unreachable" )
    in
    res

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

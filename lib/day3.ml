open! Imports
open Core

module M = struct
  (* Type to parse the input into *)
  type t = string

  (* Parse the input to type t, invoked for both parts *)
  let parse inputs = inputs

  (* Run part 1 with parsed inputs *)
  let part1 memory =
    let re = Re2.create_exn "mul\\((\\d+,\\d+)\\)" in
    let matches = Re2.find_all_exn ~sub:(`Index 1) re memory in
    let res =
      List.fold matches ~init:0 ~f:(fun acc mul ->
          let nums = String.split mul ~on:',' |> List.map ~f:Int.of_string in
          match nums with
          | [a; b] -> acc + (a * b)
          | _ -> failwith "unreachable" )
    in
    printf "Part 1: %d\n" res ; ()

  type instr = Do | Dont | Mul of int * int

  (* Run part 2 with parsed inputs *)
  let part2 memory =
    let re = Re2.create_exn "(do(n't)?\\(\\))|mul\\(\\d+,\\d+\\)" in
    let cmds =
      Re2.find_all_exn re memory
      |> List.map ~f:(fun cmd ->
             if String.is_prefix cmd ~prefix:"don't" then Dont
             else if String.is_prefix cmd ~prefix:"do" then Do
             else
               let num_str = String.slice cmd 4 (String.length cmd - 1) in
               let nums =
                 String.split num_str ~on:',' |> List.map ~f:Int.of_string
               in
               Mul (List.hd_exn nums, List.hd_exn (List.tl_exn nums)) )
    in
    let res, _ =
      List.fold cmds ~init:(0, true) ~f:(fun (acc, enabled) -> function
        | Do -> (acc, true)
        | Dont -> (acc, false)
        | Mul (a, b) ->
            if not enabled then (acc, enabled) else (acc + (a * b), enabled) )
    in
    printf "Part 2: %d\n" res ; ()
end

include M
include Day.Make (M)

(* Example input *)
let example = ""

(* Expect test for example input *)
let%expect_test _ = run example ; [%expect {| |}]

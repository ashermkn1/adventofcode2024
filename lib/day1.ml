open! Imports
open Core
module M = struct
  (* Type to parse the input into *)
  type t = int list * int list

  (* Parse the input to type t, invoked for both parts *)
  let parse input = 
    let lines = String.split_on_chars ~on:['\n'] input in
    let nums = List.map lines ~f:(String.split_on_chars ~on:[' '; ' '; ' ']) in
    let nums = List.map nums ~f:(fun s -> List.filter s ~f:(fun l -> String.is_empty l |> not)) in
    let (left, right) = List.fold nums ~init:([], []) 
    ~f:(fun (left, right) single -> 
      let (first, second) = (List.hd_exn single, List.hd_exn (List.tl_exn single)) in
      ((int_of_string first) :: left, (int_of_string second) :: right)
    ) in
    Tuple2.map (left, right) ~f:List.rev
    

  (* Run part 1 with parsed inputs *)
  let part1 (left, right) = 
    let (left, right) = Tuple2.map (left, right) ~f:(fun l -> List.sort l ~compare:Int.compare) in
    let dists = List.mapi left ~f:(fun i x -> Int.abs (List.nth_exn right i - x)) in
    print_endline_int (Utils.sum dists)

  (* Run part 2 with parsed inputs *)
  let part2 (left, right) =
    let counter = Hashtbl.create (module Int) in
    List.iter right ~f:(fun x -> 
      Hashtbl.update counter x ~f:(
        function None -> 1
          | Some c -> c + 1
      )
      );
    let res = List.fold left ~init:0 ~f:(fun acc x -> 
      let count = Option.value (Hashtbl.find counter x) ~default:0 in
      acc + (x * count)) in
    print_endline_int res;
end

include M
include Day.Make (M)

(* Example input *)
let example = ""

(* Expect test for example input *)
let%expect_test _ = run example ; [%expect {| |}]

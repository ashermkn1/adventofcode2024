open! Imports
open Core

module M = struct
  (* Type to parse the input into *)
  type t = char array array

  (* Parse the input to type t, invoked for both parts *)
  let parse input =
    let lines = String.split_lines input in
    let rows = List.length lines in
    let cols = String.length (List.hd_exn lines) in
    let grid = Array.make_matrix ~dimx:rows ~dimy:cols '.' in
    List.iteri lines ~f:(fun row line ->
        List.iteri (String.to_list line) ~f:(fun col c ->
            grid.(row).(col) <- c ) ) ;
    grid

  let in_bounds grid r c =
    let rows = Array.length grid in
    let cols = Array.length grid.(0) in
    r >= 0 && c >= 0 && r < rows && c < cols

  let next = function
    | 'X' -> 'M'
    | 'M' -> 'A'
    | 'A' -> 'S'
    | 'S' -> 'S'
    | _ -> failwith "unreachable"

  let dirs = List.cartesian_product [-1; 0; 1] [-1; 0; 1]

  let search grid r c =
    let along_dir (dr, dc) =
      let good = ref true in
      let curr = ref 'X' in
      for i = 0 to 3 do
        let r' = r + (i * dr) in
        let c' = c + (i * dc) in
        good :=
          !good && in_bounds grid r' c' && Char.equal grid.(r').(c') !curr ;
        curr := next !curr
      done ;
      !good
    in
    List.count dirs ~f:along_dir

  (* Run part 1 with parsed inputs *)
  let part1 grid =
    let res =
      Array.foldi grid ~init:0 ~f:(fun r acc row ->
          Array.foldi row ~init:acc ~f:(fun c acc _ ->
              acc + search grid r c ) )
    in
    printf "Part 1: %d\n" res ; ()

  let search2 grid r c =
    let inbound = in_bounds grid in
    let cross_in_bounds =
      inbound r c
      && inbound (r + 1) (c + 1)
      && inbound (r - 1) (c - 1)
      && inbound (r + 1) (c - 1)
      && inbound (r - 1) (c - 1)
    in
    cross_in_bounds
    && Char.equal grid.(r).(c) 'A'
    && ( Char.equal grid.(r + 1).(c + 1) 'S'
         && Char.equal grid.(r - 1).(c - 1) 'M'
       || Char.equal grid.(r + 1).(c + 1) 'M'
          && Char.equal grid.(r - 1).(c - 1) 'S' )
    && ( Char.equal grid.(r + 1).(c - 1) 'S'
         && Char.equal grid.(r - 1).(c + 1) 'M'
       || Char.equal grid.(r + 1).(c - 1) 'M'
          && Char.equal grid.(r - 1).(c + 1) 'S' )

  (* Run part 2 with parsed inputs *)
  let part2 grid =
    let res =
      Array.foldi grid ~init:0 ~f:(fun r acc row ->
          Array.foldi row ~init:acc ~f:(fun c acc _ ->
              if search2 grid r c then succ acc else acc ) )
    in
    printf "Part 2: %d\n" res ; ()
end

include M
include Day.Make (M)

(* Example input *)
let example =
  "MMMSXXMASM\n\
   MSAMXMSMSA\n\
   AMXSXMAAMM\n\
   MSAMASMSMX\n\
   XMASAMXAMM\n\
   XXAMMXXAMA\n\
   SMSMSASXSS\n\
   SAXAMASAAA\n\
   MAMMMXMMMM\n\
   MXMXAXMASX"

(* Expect test for example input *)
let%expect_test _ = run example ; [%expect {| Part 1: 18
Part 2: 9|}]

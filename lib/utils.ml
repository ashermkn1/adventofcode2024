open Core

let sum t = List.fold t ~f:(+) ~init:0
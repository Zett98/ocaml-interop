// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use znfe::{
    ocaml_alloc, ocaml_export, Intnat, IntoRust, OCaml, OCamlBytes, OCamlInt32, OCamlInt64,
    OCamlList, ToOCaml,
};

ocaml_export! {
    fn rust_twice(_gc, num: OCaml<Intnat>) -> OCaml<Intnat> {
        let num: i64 = num.into_rust();
        OCaml::of_int(num * 2)
    }

    fn rust_twice_boxed_i64(gc, num: OCaml<OCamlInt64>) -> OCaml<OCamlInt64> {
        let num: i64 = num.into_rust();
        let result = num * 2;
        ocaml_alloc!(result.to_ocaml(gc))
    }

    fn rust_twice_boxed_i32(gc, num: OCaml<OCamlInt32>) -> OCaml<OCamlInt32> {
        let num: i32 = num.into_rust();
        let result = num * 2;
        ocaml_alloc!(result.to_ocaml(gc))
    }

    fn rust_add_unboxed_floats_noalloc(_gc nokeep, num: f64, num2: f64) -> f64 {
        num * num2
    }

    fn rust_twice_boxed_float(gc, num: OCaml<f64>) -> OCaml<f64> {
        let num: f64 = num.into_rust();
        let result = num * 2.0;
        ocaml_alloc!(result.to_ocaml(gc))
    }

    fn rust_twice_unboxed_float(_gc nokeep, num: f64) -> f64 {
        num * 2.0
    }

    fn rust_increment_bytes(gc, bytes: OCaml<OCamlBytes>, first_n: OCaml<Intnat>) -> OCaml<OCamlBytes> {
        let first_n: i64 = first_n.into_rust();
        let first_n = first_n as usize;
        let mut vec: Vec<u8> = bytes.into_rust();

        for i in 0..first_n {
            vec[i] += 1;
        }

        ocaml_alloc!(vec.to_ocaml(gc))
    }

    fn rust_increment_ints_list(gc, ints: OCaml<OCamlList<Intnat>>) -> OCaml<OCamlList<Intnat>> {
        let mut vec: Vec<i64> = ints.into_rust();

        for i in 0..vec.len() {
            vec[i] += 1;
        }

        ocaml_alloc!(vec.to_ocaml(gc))
    }

    fn rust_make_tuple(gc, fst: OCaml<String>, snd: OCaml<Intnat>) -> OCaml<(String, Intnat)> {
        let fst: String = fst.into_rust();
        let snd: i64 = snd.into_rust();
        let tuple = (fst, snd);
        ocaml_alloc!(tuple.to_ocaml(gc))
    }

    fn rust_make_some(gc, value: OCaml<String>) -> OCaml<Option<String>> {
        let value: String = value.into_rust();
        let some_value = Some(value);
        ocaml_alloc!(some_value.to_ocaml(gc))
    }
}

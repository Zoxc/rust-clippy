// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;

#[allow(clippy::trivially_copy_pass_by_ref)]
fn x(y: &i32) -> i32 {
    *y
}

#[warn(clippy::all, clippy::needless_borrow)]
#[allow(unused_variables)]
fn main() {
    let a = 5;
    let b = x(&a);
    let c = x(&&a);
    let s = &String::from("hi");
    let s_ident = f(&s); // should not error, because `&String` implements Copy, but `String` does not
    let g_val = g(&Vec::new()); // should not error, because `&Vec<T>` derefs to `&[T]`
    let vec = Vec::new();
    let vec_val = g(&vec); // should not error, because `&Vec<T>` derefs to `&[T]`
    h(&"foo"); // should not error, because the `&&str` is required, due to `&Trait`
    if let Some(ref cake) = Some(&5) {}
    let garbl = match 42 {
        44 => &a,
        45 => {
            println!("foo");
            &&a // FIXME: this should lint, too
        },
        46 => &&a,
        _ => panic!(),
    };
}

fn f<T: Copy>(y: &T) -> T {
    *y
}

fn g(y: &[u8]) -> u8 {
    y[0]
}

trait Trait {}

impl<'a> Trait for &'a str {}

fn h(_: &Trait) {}
#[warn(clippy::needless_borrow)]
#[allow(dead_code)]
fn issue_1432() {
    let mut v = Vec::<String>::new();
    let _ = v.iter_mut().filter(|&ref a| a.is_empty());
    let _ = v.iter().filter(|&ref a| a.is_empty());

    let _ = v.iter().filter(|&a| a.is_empty());
}

#[allow(dead_code)]
#[warn(clippy::needless_borrow)]
#[derive(Debug)]
enum Foo<'a> {
    Str(&'a str),
}

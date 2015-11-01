#![feature(test)]

extern crate test;
extern crate tinysegmenter;

use std::io::prelude::*;
use std::fs::File;
use test::Bencher;

#[bench]
fn run(b: &mut Bencher) {
  // http://www.genpaku.org/timemachine/timemachineu8j.txt
  let mut f =
    File::open("benchmark/timemachineu8j.txt")
    .expect("Failed to read a benchmark text.");
  let mut s = String::new();
  let _ = f.read_to_string(&mut s);

  b.iter(|| tinysegmenter::tokenize(&s));
}

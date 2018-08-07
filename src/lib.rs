#[macro_use] extern crate lazy_static;
extern crate fnv;

use std::char;
use std::hash::Hash;

include!("constants.rs");

fn get_score<T: Eq + Hash>(d: &FnvHashMap<T, i32>, s: &T) -> i32 {
  d.get(s).cloned().unwrap_or(0)
}

fn get_ctype(c: char) -> char {
  match c as u32 {
    0x4E00|0x4E8C|0x4E09|0x56DB|0x4E94|0x516D|0x4E03|0x516B|0x4E5D|0x5341 => 'M',
    0x767E|0x5343|0x4E07|0x5104|0x5146 => 'M',
    0x4E00...0x9FA0|0x3005|0x3006|0x30F5|0x30F6 => 'H',
    0x3041...0x3093 => 'I',
    0x30A1...0x30F4|0x30FC|0xFF71...0xFF9D|0xFF9E|0xFF70 => 'K',
    0x61...0x7A|0x41...0x5A|0xFF41...0xFF5A|0xFF21...0xFF3A => 'A',
    0x30...0x3a|0xFF10...0xFF19 => 'N',
    _ => 'O',
  }
}

pub fn tokenize(s: &str) -> Vec<String> {
  if s.is_empty() {
    return Vec::new();
  }

  let mut result = Vec::with_capacity(s.chars().count());

  let segments =
    vec!(*B3, *B2, *B1).into_iter()
    .chain(s.chars())
    .chain(vec!(*E1, *E2, *E3).into_iter())
    .collect::<Vec<_>>();

  let ctypes =
    vec!('O'; 3).into_iter()
    .chain(s.chars().map(get_ctype))
    .chain(vec!('O'; 3).into_iter())
    .collect::<Vec<_>>();

  let mut word = segments[3].to_string();
  let mut p = vec!('U'; 3);

  for index in 4 .. segments.len() - 3 {
    let mut score = BIAS;
    let w = &segments[index - 3 .. index + 3];
    let c = &ctypes[index - 3 .. index + 3];

    score = score + get_score(&*UP1, &p[0]);
    score = score + get_score(&*UP2, &p[1]);
    score = score + get_score(&*UP3, &p[2]);
    score = score + get_score(&*BP1, &(p[0], p[1]));
    score = score + get_score(&*BP2, &(p[1], p[2]));
    score = score + get_score(&*UW1, &w[0]);
    score = score + get_score(&*UW2, &w[1]);
    score = score + get_score(&*UW3, &w[2]);
    score = score + get_score(&*UW4, &w[3]);
    score = score + get_score(&*UW5, &w[4]);
    score = score + get_score(&*UW6, &w[5]);
    score = score + get_score(&*BW1, &(w[1], w[2]));
    score = score + get_score(&*BW2, &(w[2], w[3]));
    score = score + get_score(&*BW3, &(w[3], w[4]));
    score = score + get_score(&*TW1, &(w[0], w[1], w[2]));
    score = score + get_score(&*TW2, &(w[1], w[2], w[3]));
    score = score + get_score(&*TW3, &(w[2], w[3], w[4]));
    score = score + get_score(&*TW4, &(w[3], w[4], w[5]));
    score = score + get_score(&*UC1, &c[0]);
    score = score + get_score(&*UC2, &c[1]);
    score = score + get_score(&*UC3, &c[2]);
    score = score + get_score(&*UC4, &c[3]);
    score = score + get_score(&*UC5, &c[4]);
    score = score + get_score(&*UC6, &c[5]);
    score = score + get_score(&*BC1, &(c[1], c[2]));
    score = score + get_score(&*BC2, &(c[2], c[3]));
    score = score + get_score(&*BC3, &(c[3], c[4]));
    score = score + get_score(&*TC1, &(c[0], c[1], c[2]));
    score = score + get_score(&*TC2, &(c[1], c[2], c[3]));
    score = score + get_score(&*TC3, &(c[2], c[3], c[4]));
    score = score + get_score(&*TC4, &(c[3], c[4], c[5]));
    score = score + get_score(&*UQ1, &(p[0], c[0]));
    score = score + get_score(&*UQ2, &(p[1], c[1]));
    score = score + get_score(&*UQ3, &(p[2], c[2]));
    score = score + get_score(&*BQ1, &(p[1], c[1], c[2]));
    score = score + get_score(&*BQ2, &(p[1], c[2], c[3]));
    score = score + get_score(&*BQ3, &(p[2], c[1], c[2]));
    score = score + get_score(&*BQ4, &(p[2], c[2], c[3]));
    score = score + get_score(&*TQ1, &(p[1], c[0], c[1], c[2]));
    score = score + get_score(&*TQ2, &(p[1], c[1], c[2], c[3]));
    score = score + get_score(&*TQ3, &(p[2], c[0], c[1], c[2]));
    score = score + get_score(&*TQ4, &(p[2], c[1], c[2], c[3]));

    p.remove(0);
    p.push(if score < 0 { 'O' } else { 'B' });

    if 0 < score {
      result.push(word.clone());
      word.clear();
    }
    word.push(segments[index]);
  }

  result.push(word.clone());
  result
}

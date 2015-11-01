extern crate tinysegmenter;

#[test]
fn tokenize() {
  assert_eq!(
    tinysegmenter::tokenize("私の名前は中野です"),
    ["私", "の", "名前", "は", "中野", "です"]);

  assert_eq!(
    tinysegmenter::tokenize("TinySegmenterは25kBで書かれています。"),
    ["TinySegmenter", "は", "2", "5", "kB", "で", "書か", "れ", "て", "い", "ます", "。"]);

  assert_eq!(tinysegmenter::tokenize(""), [] as [&str; 0]);
}

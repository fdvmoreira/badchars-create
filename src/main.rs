extern crate argparse;
fn main() {
  let chars_to_skip: Vec<u8> = Vec::new();
  let mut badchars: String = String::new();

  for x in 0..=255{
    if chars_to_skip.contains(&x){continue;}
    badchars = format!("{}\\x{x:02x}",badchars);
  }

  println!("{}", badchars);
}

extern crate argparse;
use argparse::Store;

fn main() {
  let mut chars_to_skip: String = String::new();
  let mut output_format: String = String::new();
  let mut output_language: String = String::new();
  let mut _badchars: String = String::new();

{  let mut parser = argparse::ArgumentParser::new();
  parser.refer(&mut chars_to_skip).add_option(&["--exclude", "--skip", "-x"], Store, "Characters to exclude from the output");
  parser.refer(&mut output_format).add_option(&["--output-format", "-o", "-f"], Store, "Output format (string or Array)");
  parser.refer(&mut output_language).add_option(&["--language", "-l"], Store, "Output language ( python | c/c++ | js | rust )");
  match parser.parse_args(){
    Ok(())=>(),
    Err(_errno) => println!("Something went wrong"),
  }
}

  println!("skip: {} output-format: {} language: {}", &mut chars_to_skip, &mut output_format, &mut output_language);

  // for x in 0..=255{
  //   if chars_to_skip.contains(&x){continue;}
  //   badchars = format!("{}\\x{x:02x}",badchars);
  // }

  // println!("{}", badchars);
}
// TODO: add support for skipping characters
// TODO: add support for choosing the output format
// TODO: add support for choosing the output type
// TODO: restrict the type of input to accept
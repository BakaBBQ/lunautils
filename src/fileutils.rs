extern crate serialize;
use std::old_io::File;
use serialize::{json, Encodable, Encoder};
use collections::str;
pub fn save_file(filepath: &str, contents: &str) -> bool{
  println!("Saved file: {}", &filepath);
  let path = Path::new(&filepath);
  let display = path.display();

  let mut file = match File::create(&path) {
    Ok(file) => file,
    Err(err) => panic!("Error in creating file! {}", err),
  };

  let r = match file.write_str(&contents) {
    Err(err) => panic!("Error in writing file! {}", err),
    Ok(_) => true,
  };

  return r;
}


pub fn save_file_with_json(filepath: &str, contents: &str) -> bool{
  let r = match json::encode(&contents) {
    Ok(s) => s,
    Err(err) => panic!("Json Encoding Error when building generic vecs: {}", err),
  };
  return save_file(&filepath, &r);
}

pub fn get_json_contents(filename: &String) -> String {
  let path = Path::new(&filename);
  let display = path.display();
  let mut file = match File::open(&path) {
    Ok(f) => f,
    Err(err) => panic!("File Error! {}", err)
  };

  let content = file.read_to_end();

  let r = match content {
    Ok(s) => s,
    Err(err) => panic!("Cannot read contents: {}", err)
  };

  let s = match str::from_utf8(&r) {
    Ok(e) => e,
    Err(err) => panic!("Invalid UTF-8 sequence: {}", err),
  };

  let result = String::from_str(&s);
  return result;
}

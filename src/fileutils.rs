extern crate serialize;
use std::old_io::File;
use serialize::{json, Encodable, Encoder};
pub fn save_file(filepath: &str, contents: &str) -> bool{
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

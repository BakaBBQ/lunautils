extern crate collections;
extern crate serialize;

mod assemblier;
mod texture_assemblier;
mod fileutils;

use std::os;
use std::old_io::File;
use collections::str;
use std::collections::HashMap;
use serialize::{json, Encodable, Encoder};
use std::path::Path;

#[derive(Encodable)]
struct FrameData {
  texture: String,
  boxes: HashMap<String, Vec<HashMap<String, i32>>>,
}

fn main() {
  let args = os::args();
  match args.len(){
    2 => do_job(&args[1]),
    _ => print_help(),
  }
}

fn print_help() {
  let help_msg: &str =
"LunaUtils
converts frames.json file into clojure-friendly frames_vector.json and frames_vm.json
---------------------------
Usage: lunautils framesjson";
  println!("{}", help_msg);
}

fn do_job(filename: &String) {
  let contents = parse_json_contents(&get_json_contents(&filename));
  let path = Path::new(&filename);
  let parent_paths = match str::from_utf8(path.dirname()) {
    Ok(s) => s,
    Err(err) => panic!("cannot decipher parent_paths: {}", err),
  };
  generate_frames_vector(&contents, parent_paths);
  generate_textures_vec(&contents, parent_paths);
}

fn generate_textures_vec(contents: &HashMap<String, HashMap<String, Vec<HashMap<String, i32>>>>, parents_path: &str) {
  let f = format!("{p}/{n}", p = parents_path, n = "textures.vec.json");
  texture_assemblier::inscribe_texture_list(&f, &contents);
}

fn get_json_contents(filename: &String) -> String {
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

fn parse_json_contents(contents: &String) -> HashMap<String, HashMap<String, Vec<HashMap<String, i32>>>>{
  let decode_results = match json::decode(&contents) {
    Ok(s) => s,
    Err(err) => panic!("Json Decoding Error! {}", err),
  };
  let map: HashMap<String, HashMap<String, Vec<HashMap<String, i32>>>> = decode_results;
  return map;
}

fn buildup_frames_vector(data: &HashMap<String, HashMap<String, Vec<HashMap<String, i32>>>>) -> Vec<FrameData>{
  let mut final_vector: Vec<FrameData> = Vec::new();
  for (t, v) in data.iter() {
    final_vector.push(FrameData {texture: (*t).clone(), boxes: (*v).clone()});
  }
  final_vector.sort_by(|a,b| a.texture.cmp(&b.texture));
  return final_vector;
}

fn encode_frames_vector(v: Vec<FrameData>)-> String{
  let r = match json::encode(&v) {
    Ok(s) => s,
    Err(err) => panic!("Json Encoding Error when building frames vector: {}", err),
  };
  return r;
}

fn generate_frames_vector(data: &HashMap<String, HashMap<String, Vec<HashMap<String, i32>>>>, parents_path: &str) {
  let c = encode_frames_vector(buildup_frames_vector(data));
  let f = format!("{p}/{n}", p = parents_path, n = "frames.vec.json");
  fileutils::save_file(&f, &c);
}

extern crate collections;
extern crate serialize;
use std::os;
use std::old_io::File;
use collections::str;
use std::collections::HashMap;
use serialize::{json, Encodable, Encoder};

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
  let help_msg: &str = "LunaUtils
converts frames.json file into clojure-friendly frames_vector.json and frames_vm.json
---------------------------
Usage: lunautils framesjson";
  println!("{}", help_msg);
}

fn do_job(filename: &String) {
  //println!("{}", get_json_contents(&filename));
  let contents = parse_json_contents(&get_json_contents(&filename));
  generate_frames_vector(contents);
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
  //for (k, v) in map.iter() {
  //  println!("{}: \"{}\"", *k, "V");
  //}
  return map;
}

fn buildup_frames_vector(data: HashMap<String, HashMap<String, Vec<HashMap<String, i32>>>>) -> Vec<FrameData>{
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

fn generate_frames_vector(data: HashMap<String, HashMap<String, Vec<HashMap<String, i32>>>>) {
  println!("{}",encode_frames_vector(buildup_frames_vector(data)))
}

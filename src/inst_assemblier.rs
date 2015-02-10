

/*
inst_assemblier.rs

assemble the complete bytecode
*/


use serialize::json;
use texture_assemblier;
use std::collections::HashMap;
use fileutils;
use regex::Regex;

enum Inst {
  NOP = 0x00,
  PUSH = 0x01,
  DEL = 0x02,

  // math operations
  PLUS = 0x10,
  SUB = 0x11,
  MULT = 0x12,
  DIV = 0x13,


  END = 0xff, //end this frame
}

#[derive(RustcDecodable, RustcEncodable)]
struct Flag  {
    key: String,
    value: String,
}


// The following representations are not the actual models for the datas.
// They are only modeled on the purpose of generating bytecodes
#[derive(Encodable)]
struct Move {
  frames: Vec<Frame>,
}

#[derive(Encodable)]
struct Frame {
  texture_id: i32,
  flags: HashMap<String, i32>,
}

impl Frame {

}



pub fn assemble(filename: &str, data: &HashMap<String, HashMap<String, Vec<HashMap<String, i32>>>>, flags: &HashMap<String, HashMap<String, i32>>) {
  save_with_json(filename, assemble_moves(group_file_vec(texture_assemblier::build_texture_vec(data)), data, flags));
}

fn group_file_vec(filenames: Vec<String>) -> HashMap<String, Vec<String>>{
  let re = match Regex::new(r"^(?P<file_pre>\D+)\d+\.png$"){
    Ok(re) => re,
    Err(err) => panic!("{}", err),
  };
  let mut groups: HashMap<String, Vec<String>> = HashMap::new();

  let mut currentMax = 0;
  for n in filenames.iter() {
    let caps = match re.captures(n) {
      Some(s) => s,
      None => panic!("nothing matched to the regex of assemblier"),
    };

    let file_pre: &str = match caps.name("file_pre") {
      Some(s) => s,
      None => panic!("It matched and it does not return a desired value"),
    };
    if (groups.contains_key(file_pre)) {
      let vec = match groups.get_mut(file_pre) {
        Some(v) => v,
        None => panic!("the group file vector passed the contains key check yet contains no value"),
      };


      vec.push(n.clone());
    } else {
      groups.insert(String::from_str(file_pre), Vec::new());
    }
  }
  return groups;
}

fn assemble_moves(groups: HashMap<String, Vec<String>>, data: &HashMap<String, HashMap<String, Vec<HashMap<String, i32>>>>, flags: &HashMap<String, HashMap<String, i32>>) -> Vec<Move>{
  let texture_vec: Vec<String> = texture_assemblier::build_texture_vec(data);

  let mut moves: Vec<Move> = Vec::new();
  for (group_name,group_members) in groups.iter() {
    let mut iter_move = Move{frames : Vec::new()};
    for texture_name in group_members.iter() {
      let id: i32 = lookup_texture_id(&texture_vec, &texture_name);
      let flags = match flags.get(texture_name){
        Some(s) => s,
        None => panic!("Data got desynchronized! texture_name exists in one but not in the other"),
      };
      iter_move.frames.push(Frame{texture_id: id, flags : flags.clone()});
    }
    iter_move.frames.sort_by(|a,b| a.texture_id.cmp(&b.texture_id));
    moves.push(iter_move);
  }
  return moves;
}

pub fn save_with_json(filepath: &str, contents: Vec<Move>) -> bool{
  let r = match json::encode(&contents) {
    Ok(s) => s,
    Err(err) => panic!("Json Encoding Error when building generic vecs: {}", err),
  };
  return fileutils::save_file(&filepath, &r);
}

fn lookup_texture_id(texture_vec: &Vec<String>, texture_name: &str) -> i32{
  // probably isn't the best way...
  let mut r: i32 = 0;
  let mut p: i32 = 0;
  for i in 0..texture_vec.len() {
    if(texture_vec[i] == texture_name.to_string()){
      r = p;
    }
    p+=1;
  }
  return r;
}

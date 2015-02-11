

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
  LIT = 0x01,
  DEL = 0x02,
  JMP = 0x03,

  // math operations
  PLUS = 0x10,
  SUB = 0x11,
  MULT = 0x12,
  DIV = 0x13,

  //hooks
  TEX = 0x30, // change current texture index, also changes the character's hitbox data


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
  fn assemble_contents(&self, idle_frame_address: i32) -> Vec<i32>{
    let mut assembling_inst: Vec<i32> = vec![Inst::LIT as i32, self.texture_id, Inst::TEX as i32];
    for i in 0..self.duration() {
      assembling_inst.push(Inst::END as i32);
    }
    return assembling_inst;
  }

  fn duration(&self) -> i32 {
    let r = match self.flags.get("duration") {
      Some(s) => s.clone(),
      None => panic!("this frame does not have a duration flag"),
    };
    return r;
  }
}

impl Move {
  fn idle(&self) -> bool {
    return self.first_frame_flag_val("idle") == 1;
  }

  fn first_frame(&self) -> &Frame {
    return &self.frames[0];
  }

  fn first_frame_flag_val(&self, key: &str) -> i32 {
    let f: &Frame = self.first_frame();
    let r: &i32 = match f.flags.get(key) {
      Some(v) => v,
      None => panic!("Could not find key :{} in the flags of first frame", key),
    };
    return r.clone();
  }

  fn assemble_my_frames(&self, idle_frame_address: i32) -> Vec<i32>{
    let mut r: Vec<i32> = Vec::new();
    for f in self.frames.iter() {
      r.push_all(&f.assemble_contents(idle_frame_address));
    }
    r.push_all(&[Inst::LIT as i32,idle_frame_address,Inst::JMP as i32]);
    return r;
  }
}



pub fn assemble(filename: &str, data: &HashMap<String, HashMap<String, Vec<HashMap<String, i32>>>>, flags: &HashMap<String, HashMap<String, i32>>) {
  //assemble_inst
  save_with_json(filename, assemble_inst(assemble_moves(group_file_vec(texture_assemblier::build_texture_vec(data)), data, flags)));
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

    } else {
      groups.insert(String::from_str(file_pre), Vec::new());
      //vec.push(n.clone());
    }
    let vec = match groups.get_mut(file_pre) {
      Some(v) => v,
      None => panic!("the group file vector passed the contains key check yet contains no value"),
    };
    vec.push(n.clone());
    println!("Grouping: {}", n);
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
    moves.sort_by(|a,b| b.idle().cmp(&a.idle()));
  }
  return moves;
}

fn assemble_inst(all_moves: Vec<Move>) -> Vec<i32>{
  let mut r: Vec<i32> = Vec::new();
  for m in all_moves.iter() {
    r.push_all(&m.assemble_my_frames(0));
  }
  return r;
}



pub fn save_with_json(filepath: &str, contents: Vec<i32>) -> bool{
  let r = match json::encode(&contents) {
    Ok(s) => s,
    Err(err) => panic!("Json Encoding Error when building generic vecs: {}", err),
  };
  return fileutils::save_file(&filepath, &r);
}

fn lookup_texture_id(texture_vec: &Vec<String>, texture_name: &str) -> i32{
  // isn't the best way...
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

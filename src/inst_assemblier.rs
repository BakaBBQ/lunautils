

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

  COND = 0x04, // conditional jump: performs as jmp if the top two stacks are equal, otherwise continues

  // math operations
  ADD = 0x10,
  SUB = 0x11,
  MULT = 0x12,
  DIV = 0x13,

  //hooks
  TEX = 0x30, // change current texture index, also changes the character's hitbox data

  //physics stuff. getters, pops the value. Velocities has nothing to do with current frames
  GPX = 0x40,
  GPY = 0x41,

  GVX = 0x42, // vx
  GVY = 0x43, // vy

  GAX = 0x44,
  GAY = 0x45,

  //setters
  SPX = 0x46,
  SPY = 0x47,

  SVX = 0x48,
  SVY = 0x49,

  SAX = 0x50,
  SAY = 0x51,

  //input
  GIP = 0x60,



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

#[derive(Encodable)]
struct InstFile {
  moves: Vec<Move>,

  // a lookup hashmap for looking up move_ids -> move_bytecode-place
  dictionary: HashMap<i32, i32>
}

impl Frame {


  fn get_velocity_x_change_inst(&self, vx_change: i32) -> Vec<i32>{
    return vec![Inst::GVX as i32, Inst::LIT as i32, vx_change, Inst::ADD as i32, Inst::SVX as i32];
  }

  fn get_velocity_y_change_inst(&self, vy_change: i32) -> Vec<i32>{
    return vec![Inst::GVY as i32, Inst::LIT as i32, vy_change, Inst::ADD as i32, Inst::SVY as i32];
  }

  fn get_set_velocity_x_inst(&self, vx_set: i32) -> Vec<i32>{
    return vec![Inst::LIT as i32, vx_set, Inst::SVX as i32];
  }

  fn get_set_velocity_y_inst(&self, vy_set: i32) -> Vec<i32>{
    return vec![Inst::LIT as i32, vy_set, Inst::SVY as i32];
  }


  fn duration(&self) -> i32 {
    let r = match self.flags.get("duration") {
      Some(s) => s.clone(),
      None => panic!("this frame does not have a duration flag"),
    };
    return r;
  }

  fn vx_set(&self) -> i32 {
    return self.retrieve_flag_with_default("vx_set", 0)
  }

  fn vy_set(&self) -> i32 {
    return self.retrieve_flag_with_default("vy_set", 0)
  }

  fn retrieve_flag(&self, key: &str) -> i32 {
    let r = match self.flags.get(key) {
      Some(s) => s.clone(),
      None => panic!("this frame does not have a flag of {}", key),
    };
    return r;
  }

  fn retrieve_flag_with_default(&self, key: &str, default: i32) -> i32 {
    let r = match self.flags.get(key) {
      Some(s) => s.clone(),
      None => default,
    };
    return r;
  }

  fn assemble_contents(&self, idle_frame_address: i32, allocated_insts: i32, inst_file: &InstFile, all_possible_cancel_codes: Vec<i32>) -> Vec<i32>{
    let mut assembling_inst: Vec<i32> = vec![Inst::LIT as i32, self.texture_id, Inst::TEX as i32];
    assembling_inst.push_all(&self.get_set_velocity_x_inst(self.vx_set()));
    assembling_inst.push_all(&self.get_set_velocity_y_inst(self.vy_set()));

    for i in 0..self.duration() {
      for possible_cancel_move_ids in all_possible_cancel_codes.iter() {
        assembling_inst.push_all(&self.get_intermission_insts_of_canceling(possible_cancel_move_ids.clone(), inst_file.get_move_location(possible_cancel_move_ids.clone())));
      }
//      assembling_inst.push_all(&self.get_intermission_insts_of_canceling());
      assembling_inst.push(Inst::END as i32);
    }

    let rest_space: i32 = allocated_insts - (assembling_inst.len() as i32);
    for i in 0..rest_space {
      assembling_inst.push(0x00);
    }
    return assembling_inst;
  }

  // very low level helper function
  fn get_intermission_insts_of_canceling(&self, move_id: i32, jmp_loc: i32) -> Vec<i32>{
    return vec![Inst::GIP as i32, Inst::LIT as i32, move_id, Inst::COND as i32, jmp_loc as i32];
  }
}

impl Move {
  fn idle(&self) -> bool {
    return self.first_frame_flag_val("idle") == 1;
  }

  fn move_id(&self) -> i32 {
    return self.first_frame_flag_val("move_id");
  }

  fn cancel_level(&self) -> i32 {
    return self.first_frame_flag_val_with_default("cancel_level", 0);
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

  fn first_frame_flag_val_with_default(&self, key: &str, default: i32) -> i32 {
    let f: &Frame = self.first_frame();
    let r: &i32 = match f.flags.get(key) {
      Some(v) => v,
      None => &default,
    };
    return r.clone();
  }

  fn assemble_my_frames(&self, idle_frame_address: i32, allocated_insts: i32, possible_cancels_for_one_file: Vec<i32>, inst_file: &InstFile) -> Vec<i32>{
    let mut r: Vec<i32> = Vec::new();
    for f in self.frames.iter() {
      r.push_all(&f.assemble_contents(idle_frame_address, allocated_insts, inst_file, possible_cancels_for_one_file.clone()));
    }
    r.push_all(&[Inst::JMP as i32, idle_frame_address]);
    return r;
  }
}

// I guess I have to statically allocate instructions for each move
impl InstFile {
  fn assemble_final(&self) -> Vec<i32> {
    let possible_cancels = self.assemble_possible_cancels();
    let mut r: Vec<i32> = Vec::new();
    for m in self.moves.iter() {
      let c: i32 = m.move_id();
      let possible_cancels_for_one_file = match possible_cancels.get(&c) {
        Some(s) => s,
        None => panic!("possible_cancels not detected"),
      };
      r.push_all(&m.assemble_my_frames(0, 50, possible_cancels_for_one_file.clone() , self));
    }
    return r;
  }

  fn assemble_possible_cancels(&self) -> HashMap<i32, Vec<i32>> {
    let mut r: HashMap<i32, Vec<i32>> = HashMap::new();
    for key_move in self.moves.iter() {
      let mut possible_cancels: Vec<i32> = Vec::new();
      for comparing_move in self.moves.iter() {
        if (comparing_move.cancel_level() > key_move.cancel_level()) {
          possible_cancels.push(comparing_move.move_id());
        }
      }
      r.insert(key_move.move_id(), possible_cancels);
    }
    return r;
  }

// move code such as: 0x623b and etc
  fn get_move_location(&self, move_code: i32) -> i32 {
    let r: &i32 = match self.dictionary.get(&move_code) {
      Some(s) => s,
      None => panic!("cannot retrieve anything from the move_code: {}", move_code),
    };
    return r.clone();
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
    //println!("Grouping: {}", n);
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
  let mut register_map: HashMap<i32, i32> = HashMap::new();
  // at least this is clear
  let mut i = 0;
  for m in all_moves.iter() {
    register_map.insert(m.move_id(), i * 50);
    i += 1;
  }
  let file_model = InstFile{moves: all_moves, dictionary: register_map};
  return file_model.assemble_final();
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

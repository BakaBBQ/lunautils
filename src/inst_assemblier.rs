

/*
inst_assemblier.rs

assemble the complete bytecode
*/
static ALLOCATED_BYTECODES: i32 = 50;

use serialize::json;
use texture_assemblier;
use canceling_assemblier;
use std::collections::HashMap;
use fileutils;
use regex::Regex;

use insts::{NOP,LIT,DEL,JMP,COND,JAS,GNP,GCP,ADD,SUB,MULT,DIV,TEX,GPX,GPY,GVX,GVY,SPX,SPY,SVX,SVY,SAX,SAY,GIP,END};
use canceling_assemblier::CancelArea;


#[derive(RustcDecodable, RustcEncodable)]
struct Flag  {
    key: String,
    value: String,
}


// The following representations are not the actual models for the datas.
// They are only modeled on the purpose of generating bytecodes
#[derive(Encodable)]
pub struct Move {
  frames: Vec<Frame>,
}

#[derive(Encodable)]
pub struct Frame {
  texture_id: i32,
  flags: HashMap<String, i32>,
}

#[derive(Encodable)]
pub struct InstFile {
  moves: Vec<Move>,

  // a lookup hashmap for looking up move_ids -> move_bytecode-place
  dictionary: HashMap<i32, i32>,
}

impl Frame {
  fn get_velocity_x_change_inst(&self, vx_change: i32) -> Vec<i32>{
    return vec![GVX, LIT, vx_change, ADD, SVX];
  }

  fn get_velocity_y_change_inst(&self, vy_change: i32) -> Vec<i32>{
    return vec![GVY, LIT, vy_change, ADD, SVY];
  }

  fn get_set_velocity_x_inst(&self, vx_set: i32) -> Vec<i32>{
    return vec![LIT, vx_set, SVX];
  }

  fn get_set_velocity_y_inst(&self, vy_set: i32) -> Vec<i32>{
    return vec![LIT, vy_set, SVY];
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

  fn assemble_contents(&self, idle_frame_address: i32, allocated_insts: i32, inst_file: &InstFile, all_possible_cancel_codes: Vec<i32>, cancel_area: &CancelArea, m: &Move) -> Vec<i32>{
    let mut assembling_inst: Vec<i32> = vec![LIT, self.texture_id, TEX];

    if (self.vx_set() != 0) {
      assembling_inst.push_all(&self.get_set_velocity_x_inst(self.vx_set()));
    }

    if (self.vy_set() != 0) {
      assembling_inst.push_all(&self.get_set_velocity_y_inst(self.vy_set()));
    }



    for i in 0..self.duration() {
      for possible_cancel_move_ids in all_possible_cancel_codes.iter() {
        //assembling_inst.push_all(&self.get_intermission_insts_of_canceling(possible_cancel_move_ids.clone(), inst_file.get_move_location(possible_cancel_move_ids.clone())));
      }
      assembling_inst.push_all(&self.get_move_to_canceling_area_inst(m, &cancel_area.dictionary_by_level, inst_file.estimated_length()));
      assembling_inst.push(END);
    }

    let rest_space: i32 = allocated_insts - (assembling_inst.len() as i32);
    if rest_space <= 1 {
      panic!("Space insufficient for additional bytes, will lead to possible jump error!");
    }
    for i in 0..rest_space {
      assembling_inst.push(0x00);
    }
    return assembling_inst;
  }

  // very low level helper function
  fn get_intermission_insts_of_canceling(&self, move_id: i32, jmp_loc: i32) -> Vec<i32>{
    return vec![GIP, LIT, move_id, COND, jmp_loc];
  }

  fn get_move_to_canceling_area_inst(&self, my_move: &Move, cancel_dictionary: &HashMap<i32, i32>, offset: i32) -> Vec<i32>{
    // gnp lit 5 add jmp my_cancel_level_pos
    return vec![GNP, LIT, 5, ADD, JMP, self.get_jmp_pos_for_cancel_level(my_move.cancel_level(), cancel_dictionary, offset)];
  }

  fn get_jmp_pos_for_cancel_level(&self, x: i32, cancel_dictionary: &HashMap<i32, i32>, offset: i32) -> i32{
    let r = match cancel_dictionary.get(&x) {
      Some(s) => s,
      None => panic!("Cannot find cancel address for cancel level: {}", x),
    };
    return r.clone() + offset;
  }
}

impl Move {
  pub fn idle(&self) -> bool {
    return self.first_frame_flag_val("idle") == 1;
  }

  pub fn move_id(&self) -> i32 {
    return self.first_frame_flag_val("move_id");
  }

  pub fn cancel_level(&self) -> i32 {
    return self.first_frame_flag_val_with_default("cancel_level", 0);
  }

  fn first_frame(&self) -> &Frame {
    return &self.frames[0];
  }

  fn estimated_length(&self) -> i32 {
    return (self.frames.len() as i32) * ALLOCATED_BYTECODES + 2;
  }

  fn first_frame_flag_val(&self, key: &str) -> i32 {
    let f: &Frame = self.first_frame();
    let r: &i32 = match f.flags.get(key) {
      Some(v) => v,
      None => panic!("Could not find key __{}__ in the flags of first frame", key),
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

  fn assemble_my_frames(&self, idle_frame_address: i32, allocated_insts: i32, possible_cancels_for_one_file: Vec<i32>, inst_file: &InstFile, cancel_area: &CancelArea) -> Vec<i32>{
    let mut r: Vec<i32> = Vec::new();
    for f in self.frames.iter() {
      r.push_all(&f.assemble_contents(idle_frame_address, allocated_insts, inst_file, possible_cancels_for_one_file.clone(), cancel_area, self));
    }
    r.push_all(&[JMP, idle_frame_address]);
    return r;
  }
}

// I guess I have to statically allocate instructions for each move
impl InstFile {
  fn assemble_final(&self) -> Vec<i32> {
    let possible_cancels = self.assemble_possible_cancels();
    let mut r: Vec<i32> = Vec::new();
    self.output_cancel_tree(&possible_cancels);

    let cancel_area: CancelArea = canceling_assemblier::get_canceling_area(&self.moves, &possible_cancels, self);
    for m in self.moves.iter() {
      let c: i32 = m.move_id();
      let possible_cancels_for_one_file = match possible_cancels.get(&c) {
        Some(s) => s,
        None => panic!("possible_cancels not detected"),
      };
      r.push_all(&m.assemble_my_frames(0, ALLOCATED_BYTECODES, possible_cancels_for_one_file.clone() , self, &cancel_area));
    }
    r.push_all(&cancel_area.inst);
    return r;
  }

  fn estimated_length(&self) -> i32{
    let mut c = 0;
    for m in self.moves.iter() {
      c = c + m.estimated_length();
    }
    return c;
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

  fn output_cancel_tree(&self, possible_cancels: &HashMap<i32, Vec<i32>>) {
    println!("");
    println!("Cancel Tree: ");
    for (key_move, all_cancels) in possible_cancels.iter() {
      println!("{}", format!("Printing cancel tree for {:X}", key_move).as_slice());
      for single_cancel in all_cancels.iter() {
        println!("{}", format!(" ---> {:X}", single_cancel).as_slice());
      }
    }
    println!("");
  }

// move code such as: 0x623b and etc
  pub fn get_move_location(&self, move_code: i32) -> i32 {
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
    moves.sort_by(|a,b| a.cancel_level().cmp(&b.cancel_level()));
  }
  return moves;
}

fn assemble_inst(all_moves: Vec<Move>) -> Vec<i32>{
  let mut register_map: HashMap<i32, i32> = HashMap::new();
  // at least this is clear
  let mut i = 0;
  let mut j = 0;
  for m in all_moves.iter() {
    register_map.insert(m.move_id(), j);
    i += 1;
    j += (m.frames.len() as i32) * ALLOCATED_BYTECODES + 2;
    //println!("{}", format!("{:X} has a length of {}", m.move_id(), m.frames.len()).as_slice());
  }

  output_move_map(&register_map);
  let file_model = InstFile{moves: all_moves, dictionary: register_map};
  return file_model.assemble_final();
}

fn output_move_map(move_map: &HashMap<i32, i32>) {
  println!("");
  println!("MoveMap: ");
  for (k,v) in move_map.iter() {
    println!("{}", format!(" - {:X} -> {}", k, v));
  }
  println!("");

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

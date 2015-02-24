/*
lit move_id gip cond move_pos lit move_id2 gip cond move_pos2
*/

use std::collections::HashMap;
use insts::{NOP,LIT,DEL,JMP,COND,JAS,GNP,GCP,ADD,SUB,MULT,DIV,TEX,GPX,GPY,GVX,GVY,SPX,SPY,SVX,SVY,SAX,SAY,GIP,END};
use inst_assemblier::Move;
use inst_assemblier::InstFile;

pub struct CancelArea {
  pub inst: Vec<i32>,
  pub dictionary_by_level: HashMap<i32, i32>, // this dictionary starts on 0, move_level -> loc
}

pub fn get_canceling_area(moves: &Vec<Move>, cancel_tree: &HashMap<i32, Vec<i32>>, inst_file: &InstFile) -> CancelArea{
  let mut canceling_area_inst: Vec<i32> = vec![];
  let mut dictionary_by_level: HashMap<i32, i32> = HashMap::new();
  dictionary_by_level.insert(0, 0);
  let sorted_moves: &Vec<Move> = sort_moves_according_to_cancel_level(moves);

  let mut current_cancel_level = 0;
  for m in sorted_moves.iter() {
    canceling_area_inst.push_all(&get_cancel_commponent_for_single_move(m, inst_file));
    // those hefty comparisons
    if (m.cancel_level() >= current_cancel_level) {
      current_cancel_level = m.cancel_level();
      dictionary_by_level.insert(current_cancel_level, canceling_area_inst.len() as i32);
    }
  }
  canceling_area_inst.push_all(&get_end_inst(inst_file));
  output_dictionary_results(&dictionary_by_level);
  return CancelArea{inst: canceling_area_inst, dictionary_by_level: dictionary_by_level};
}

fn output_dictionary_results(dictionary: &HashMap<i32, i32>) {
  println!("Canceling Dictionary: ");
  for (lv, target) in dictionary.iter() {
    println!("{}", format!("{:X} -> {}", lv, target));
  }
}

fn sort_moves_according_to_cancel_level(moves: &Vec<Move>) -> &Vec<Move>{
  let mut c_moves = moves.clone();
  return c_moves;
}

fn get_cancel_commponent_for_single_move(m: &Move, inst_file: &InstFile) -> Vec<i32>{
  return vec![LIT, m.move_id(), GIP, COND, get_move_pos(m, inst_file)];
}

fn get_move_pos(m: &Move, inst_file: &InstFile) -> i32 {
  return inst_file.get_move_location(m.move_id());
}

fn get_end_inst(inst_file: &InstFile) -> Vec<i32> {
  return vec![JAS];
}

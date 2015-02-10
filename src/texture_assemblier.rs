extern crate serialize;
use std::collections::HashMap;
use serialize::{json, Encodable, Encoder};
use fileutils;

pub fn inscribe_texture_list(filename: &str, data: &HashMap<String, HashMap<String, Vec<HashMap<String, i32>>>>) {
  save2file_texture_list(texture_vec2json(build_texture_vec(data)), filename);
}

fn build_texture_vec(data: &HashMap<String, HashMap<String, Vec<HashMap<String, i32>>>>) -> Vec<String> {
  // I am not going to care much about performance here
  let mut result: Vec<String> = Vec::new();
  for key in data.keys() {
    result.push(key.clone());
  }
  return result;
}

fn texture_vec2json(textures: Vec<String>) -> String{
  let encoded_json: String = match json::encode(&textures) {
    Ok(s) => s,
    Err(err) => panic!("Json Encoding Error when building textures vector: {}", err),
  };

  let r = encoded_json;
  return r;
}

fn save2file_texture_list(json: String, filename: &str) {
  fileutils::save_file(&filename, &json);
}

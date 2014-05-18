extern crate core;

use core::mem;
use std::io;
use std::io::{BufReader, SeekSet};
use std::iter::range_step;

static hash_size:uint = 64*1024;

fn volnitsky(data: &[u8], substring: &[u8]) -> Option<uint> {
  let mut dataReader = BufReader::new(data);
  let mut substringReader = BufReader::new(substring);

  let w_size =  mem::size_of::<u16>();
  let step = substring.len() - w_size + 1;

  let mut hash = [0u8, ..hash_size];

  for i in range(0, substring.len() - w_size).rev() {
    substringReader.seek(i as i64, SeekSet).unwrap();
    let mut hash_index = (substringReader.read_le_u16().unwrap() as uint) % hash_size;
    while hash[hash_index] != 0 {
      hash_index = (hash_index + 1) % hash_size;
    }
    hash[hash_index] = i as u8 + 1;
  }

  for offset in range_step(substring.len() - w_size, data.len() - substring.len() + 1, step) {
    dataReader.seek(offset as i64, SeekSet).unwrap();
    let mut hash_index = (dataReader.read_le_u16().unwrap() as uint) % hash_size;
    'hash_check: loop {
      if hash[hash_index] == 0 {
        break;
      }
      let subOffset = offset - (hash[hash_index] as uint - 1);
      for i in range(0,substring.len()) {
        if data[subOffset + i] != substring[i] {
          hash_index = (hash_index + 1) % hash_size;
          continue 'hash_check;
        }
      }
      return Some(subOffset);
    }
  }

  return None;
  // should have:
  //return  std::search(P-step+1,Se,SS,SSe);
}


fn main() {
  let mut stdin = io::stdin();
  let dataVec = stdin.read_to_end().unwrap();
  let data = dataVec.slice(0,dataVec.len());
  let substring = bytes!("Anything whatsoever related to the Rust programming language: an open-source systems programming language from Mozilla, emphasizing safety, concurrency, and speed.");
  // let substring = bytes!("This eBook is for the use ");
  println!("voln: {}", volnitsky(data, substring));
}

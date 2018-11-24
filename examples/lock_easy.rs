
extern crate cluLockFile;

use cluLockFile::LockPathConst;
use cluLockFile::Locker;
use std::path::Path;

pub fn main() {
     let light_lock = Path::new("/tmp/light.lock").path_lock().unwrap();

     let light2_lock = "/tmp/light2.lock".path_lock().unwrap();

     let light3_buf_lock = {
          let mut path = Path::new("/tmp").to_path_buf();
          path.push(format!("light{}.lock", 3));

          path
     }.path_lock().unwrap();

     println!("#1 {:?}, is_lock: {}", light_lock, light_lock.is_lock());
     println!("#2 {:?}, is_lock: {}", light2_lock, light2_lock.is_lock());
     println!("#3 {:?}, is_lock: {}", light3_buf_lock, light3_buf_lock.is_lock());

     drop(light_lock);
     drop(light2_lock);
     drop(light3_buf_lock);
}
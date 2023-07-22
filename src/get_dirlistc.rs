extern crate exif;
use std::io::BufReader;
use std::fs::File;
use std::fs;
use exif::{Reader, In, Tag};
use crate::dump_file::dump_file;

use std::path::{PathBuf};

pub fn get_dirlistc (current_dir: PathBuf) -> (u32, String, String) {
    let errcode: u32;
    let errstring: String;
    let mut new_dirlist: String = " ".to_string();
    let mut orient;
    let mut listitems: Vec<String> = Vec::new();
    let mut numentry = 0;
    for entry1 in fs::read_dir(&current_dir).unwrap() {
         let entry = entry1.unwrap();
         if let Ok(metadata) = entry.metadata() {
             if let Ok(file_name) = entry.file_name().into_string() {
                 if metadata.is_file() {
                     let file_path = entry.path();
                     if let Err(e) = dump_file(&file_path) {
                         orient = format!("Meta error : {}", e);
                     } else {
                         let file = File::open(file_path).unwrap();
                         let reader = Reader::new().read_from_container(&mut BufReader::new(&file)).unwrap();
                         if let Some(field) = reader.get_field(Tag::Orientation, In::PRIMARY) {
                             if let Some(width) = field.value.get_uint(0) {
                                 orient = format!("{}", width);
                             } else {
                                 orient = format!("-");
                             }
                         } else {
                             orient = format!("x");
                         }
                     }
                     let listival = file_name + " | " + "orientation: " + &orient;
                     listitems.push(listival);
                     numentry = numentry + 1;
                 }
             }
         }
    }
    if numentry > 0 {
        listitems.sort();
        let listitemlen = listitems.len();
        let newtoi = listitemlen as i32 ;
        for indexi in 0..newtoi {
             let namelist = &listitems[indexi as usize];
             new_dirlist = new_dirlist + namelist + "\n ";
        }
        errstring = format!("{} files in directory ", numentry);
        errcode = 0;
    } else {
        errstring = "********* Directory 1: directory has no images **********".to_string();
        errcode = 1;
    }
    (errcode, errstring, new_dirlist)
}


//extern crate exif;
//use std::io::BufReader;
//use std::fs::File;
use std::fs;
//use exif::{Reader, In, Tag};
//use crate::dump_file::dump_file;

use std::path::{PathBuf};

// function called by Organize directory 1 & 2  buttons and Convert directory button
//  Use to get list of sorted files in the directory list in model format
// input is the directory and output is error number, error string and model
pub fn get_dirlist (current_dir: PathBuf) -> (u32, String, String) {
    let errcode: u32;
    let errstring: String;
    let mut new_dirlist: String = " ".to_string();
    let mut listitems: Vec<String> = Vec::new();
    let mut numentry = 0;
    if !current_dir.exists() {
         errstring = "directory does not exist".to_string();
         errcode = 1;
    } else {
        for entry1 in fs::read_dir(&current_dir).unwrap() {
             let entry = entry1.unwrap();
             if let Ok(metadata) = entry.metadata() {
                 if let Ok(file_name) = entry.file_name().into_string() {
                     if metadata.is_file() {
                         if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                            file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") |
                            file_name.ends_with(".png") |file_name.ends_with(".PNG") {
                             let listival = file_name;
                             listitems.push(listival);
                             numentry = numentry + 1;
                         }
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
            errstring = "********* No images in directory **********".to_string();
            errcode = 2;
        }
     }
    (errcode, errstring, new_dirlist)
}


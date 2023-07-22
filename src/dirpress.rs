use native_dialog::FileDialog;
use std::path::{Path};
use crate::get_dirlist;
pub fn dirpress(dirval: String) -> (u32, String, String, String) {
     let errcode: u32;
     let errstring: String;
     let mut new_dirlist: String = " ".to_string();
     let mut new_dir: String;
     if Path::new(&dirval).exists() {
         new_dir = dirval.to_string();
     } else {
         new_dir = "/".to_string();
     }
     let folder = FileDialog::new()
        .set_location(&new_dir)
        .show_open_single_dir()
        .unwrap();
     if folder == None {
         errstring = "error getting directory -- possible cancel key hit".to_string();
         errcode = 1;
     } else {
         new_dir = folder.as_ref().expect("REASON").display().to_string();
         let current_dir = folder;
         let (errcd, errstr, newliststr) = get_dirlist(current_dir.unwrap());
         if errcd == 0 {
             new_dirlist = newliststr;
             errstring = "got directory".to_string();
             errcode = 0;
         } else {
             errstring = errstr.to_string();
             errcode = 2;
         }
     } 
    (errcode, errstring, new_dir, new_dirlist)
}


mod dirpress;
mod get_dirlist;
mod get_dirlistr;
mod get_dirlistc;
mod rc_rotatepress;
mod c8_diroutpress;
mod c8_copypress;
mod dump_file;
mod get_winsize;

use get_dirlist::get_dirlist;
use get_dirlistr::get_dirlistr;
use get_dirlistc::get_dirlistc;
use dirpress::dirpress;
use get_winsize::get_winsize;
use c8_diroutpress::c8_diroutpress;

// use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, text, column, button, row, Radio, progress_bar, horizontal_space, container, Scrollable, Text};
use iced::{Element, Task, Length, Alignment, Color};

use iced::theme::Theme;

use std::path::Path;
use std::process::Command as stdCommand;
use std::time::{Duration, Instant};
use std::thread::sleep;
use iced::futures;
use futures::channel::mpsc;
use crate::rc_rotatepress::rc_rotatepress;
use c8_copypress::c8_copypress;

fn main() -> iced::Result {
     let mut widthxx: f32 = 1350.0;
     let mut heightxx: f32 = 750.0;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho as f32 - 20.0;
         heightxx = heighto as f32 - 75.0;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }
     iced::application(PhotoRot1080::title, PhotoRot1080::update, PhotoRot1080::view)
        .window_size((widthxx, heightxx))
        .theme(PhotoRot1080::theme)
//        .subscription(ImageList::subscription)
        .run_with(PhotoRot1080::new)

}

struct PhotoRot1080 {
    dir_value: String,
    mess_color: Color,
    msg_value: String,
    do_progress: bool,
    progval: f32,
    tx_send: mpsc::UnboundedSender<String>,
    rx_receive: mpsc::UnboundedReceiver<String>,
     pagechoice_value: PageChoice,
    c8scrol_value: String,
    rcscrol_value: String,
    outdir_value: String,

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PageChoice {
    ROT,
    CON,
    IND,
 }

impl Default for PageChoice {
    fn default() -> Self {
        PageChoice::ROT
    }
}

#[derive(Clone, Debug)]
enum Message {
    DirPressed,
    ProgressPressed,
    ProgRtn(Result<Progstart, Error>),
    RotatexFound(Result<Rotatex, Error>),
    CopyxFound(Result<Copyx, Error>),
    PageRadioSelected(PageChoice),
    RCListPressed,
    RCRotallPressed,
    C8OutDirPressed,
    C8ListPressed,
    C8CopyPressed,
    INStartButton,
}

impl PhotoRot1080 {
    fn new() -> (Self, iced::Task<Message>) {
        let (tx_send, rx_receive) = mpsc::unbounded();
        (  PhotoRot1080 {
                dir_value: "no directory".to_string(),
                mess_color: Color::from([0.0, 0.0, 1.0]),
                msg_value: "no message".to_string(),
                do_progress: false,
                pagechoice_value: PageChoice::ROT,
                progval: 0.0,
                tx_send,
                rx_receive,
                c8scrol_value: " nothing to process ".to_string(),
                rcscrol_value: " nothing to process ".to_string(),
                outdir_value: String::new(),
           },
            Task::none(),
        )

    }

    fn title(&self) -> String {
        String::from("Photo Rotate Convert 1080")
    }

    fn update(&mut self, message: Message) -> Task<Message>  {
        match message {

            Message::PageRadioSelected(xchoice) => {
                let strx = match xchoice {
                PageChoice::ROT => "page choice rotate correction selected",
                PageChoice::CON => "page choice convert to 1080 selected",
                PageChoice::IND => "page choice individual rotate selected",};
                self.pagechoice_value = xchoice;
                self.mess_color = Color::from([0.0, 1.0, 0.0]);
                self.msg_value = strx.to_string();
               Task::none()
            }
            Message::RCListPressed => {
                self.rcscrol_value = " nothing to process ".to_string();
                if !Path::new(&self.dir_value).exists() {
                    self.msg_value = format!("directory does not exist: {}", self.dir_value);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else {
                    let dir_path = Path::new(&self.dir_value);
                    let (errcd, errstr, newliststr) = get_dirlistr(dir_path.to_path_buf());
                    if errcd == 0 {
                        self.rcscrol_value  = newliststr.to_string();
                        self.msg_value = format!("directory entries for: {}", self.dir_value);
                        self.mess_color = Color::from([0.0, 1.0, 0.0]);
                    } else {
                        self.msg_value = errstr.to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    }
                }
                Task::none()
            }
            Message::C8ListPressed => {
                self.c8scrol_value = " nothing to process ".to_string();
                if !Path::new(&self.dir_value).exists() {
                    self.msg_value = format!("directory does not exist: {}", self.dir_value);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else {
                    let dir_path = Path::new(&self.dir_value);
                    let (errcd, errstr, newliststr) = get_dirlistc(dir_path.to_path_buf());
                    if errcd == 0 {
                        self.c8scrol_value  = newliststr.to_string();
                        self.msg_value = format!("directory entries for: {}", self.dir_value);
                        self.mess_color = Color::from([0.0, 1.0, 0.0]);
                    } else {
                        self.msg_value = errstr.to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    }
                }
                Task::none()
            }
            Message::C8OutDirPressed => {
                let mut a_dir: String = self.dir_value.clone().to_string();
                if !Path::new(&a_dir).exists() {
                    a_dir = self.outdir_value.clone().to_string();
                }
                let (errcode, errstr, newdir) = c8_diroutpress(a_dir);
                self.msg_value = errstr.to_string();
                if errcode == 0 {
                    self.outdir_value = newdir.to_string();
                    self.mess_color = Color::from([0.0, 1.0, 0.0]);
                } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                }
                Task::none()
             }
             Message::C8CopyPressed => {
                let (errcode, errstr) = c8_copypress(self.dir_value.clone(), self.outdir_value.clone(), self.c8scrol_value.clone());
                self.msg_value = errstr.to_string();
                if errcode == 0 {
                    self.mess_color = Color::from([0.0, 1.0, 0.0]);
                    Task::perform(Copyx::copyit(self.dir_value.clone(), self.outdir_value.clone(), self.c8scrol_value.clone(), self.tx_send.clone()), Message::CopyxFound)
                } else {
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    Task::none()
                }
             }
 
             
            Message::RCRotallPressed => {
                if !Path::new(&self.dir_value).exists() {
                    self.msg_value = format!("directory does not exist: {}", self.dir_value);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    Task::none()
                } else {
                    let dir_path = Path::new(&self.dir_value);
                    let (errcd, errstr, newliststr) = get_dirlistr(dir_path.to_path_buf());
                    if errcd == 0 {
                        let (errrc, errstrrc) = rc_rotatepress(self.dir_value.clone(), newliststr.clone());
                        self.msg_value = errstrrc.to_string();
                        if errrc == 0 {
                            self.mess_color = Color::from([0.0, 1.0, 0.0]);
                            Task::perform(Rotatex::rotateit(self.dir_value.clone(), newliststr.clone(), self.tx_send.clone()), Message::RotatexFound)
                        } else {
                            self.mess_color = Color::from([1.0, 0.0, 0.0]);
                            Task::none()
                        }
                    } else {
                        self.msg_value = errstr.to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                        Task::none()
                    }
                }
            }

            Message::INStartButton => {
                if Path::new(&self.dir_value).exists() {
                    stdCommand::new("indivrotate1310")
                             .arg(&self.dir_value)
                             .spawn()
                             .expect("failed to execute process");
                    self.msg_value = "started indivrotate01310 program".to_string();
                    self.mess_color = Color::from([0.0, 1.0, 0.0]);
                } else {
                    self.msg_value = "The directory does not exist".to_string();
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                }   
                Task::none()
            }

            Message::DirPressed => {
               let (errcode, errstr, newdir, _newliststr) = dirpress(self.dir_value.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.dir_value = newdir.to_string();
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Task::none()
            }
            Message::ProgressPressed => {
                   self.do_progress = true;
                   Task::perform(Progstart::pstart(), Message::ProgRtn)
            }
            Message::CopyxFound(Ok(copyx)) => {
                self.msg_value = copyx.errval.clone();
                self.mess_color = copyx.errcolor.clone();
                self.do_progress = false;
                self.progval = 0.0;
                Task::none()
            }
            Message::CopyxFound(Err(_error)) => {
                self.msg_value = "error in copyx copyit routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
                Task::none()
            }
            Message::RotatexFound(Ok(copyx)) => {
                self.msg_value = copyx.errval.clone();
                self.mess_color = copyx.errcolor.clone();
                self.do_progress = false;
                self.progval = 0.0;
               Task::none()
            }
            Message::RotatexFound(Err(_error)) => {
                self.msg_value = "error in copyx copyit routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Task::none()
            }
            Message::ProgRtn(Ok(_prx)) => {
              if self.do_progress {
                let mut inputval  = " ".to_string();
                let mut bgotmesg = false;
                while let Ok(Some(input)) = self.rx_receive.try_next() {
                   inputval = input;
                   bgotmesg = true;
                }
                if bgotmesg {
                    let progvec: Vec<&str> = inputval[0..].split("|").collect();
                    let lenpg1 = progvec.len();
                    if lenpg1 == 3 {
                        let prog1 = progvec[0].to_string();
                        if prog1 == "Progress" {
                            let num_int: i32 = progvec[1].parse().unwrap_or(-9999);
                            if num_int == -9999 {
                                println!("progress numeric not numeric: {}", inputval);
                            } else {
                                let dem_int: i32 = progvec[2].parse().unwrap_or(-9999);
                                if dem_int == -9999 {
                                    println!("progress numeric not numeric: {}", inputval);
                                } else {
                                    self.progval = 100.0 * (num_int as f32 / dem_int as f32);
                                    self.msg_value = format!("Convert progress: {} of {}", num_int, dem_int);
                                    self.mess_color = Color::from([0.0, 0.0, 1.0]);
                                }
                            }
                        } else {
                            println!("message not progress: {}", inputval);
                        }
                    } else {
                        println!("message not progress: {}", inputval);
                    }
                }             
                Task::perform(Progstart::pstart(), Message::ProgRtn)
              } else {
                Task::none()
              }
            }
            Message::ProgRtn(Err(_error)) => {
                self.msg_value = "error in Progstart::pstart routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
            let selected_pagechoice = Some(self.pagechoice_value);
            let ua = Radio::new(
                     "Rotate Correction",
                     PageChoice::ROT,
                     selected_pagechoice,
                     Message::PageRadioSelected,
            ).size(15);
            let ub = Radio::new(
                     "Convert to 1080",
                     PageChoice::CON,
                     selected_pagechoice,
                     Message::PageRadioSelected,
            ).size(15);
       
            let uc = Radio::new(
                     "Individual Rotations",
                     PageChoice::IND,
                     selected_pagechoice,
                     Message::PageRadioSelected,
            ).size(15);

            let mut topshow = Column::new().spacing(10);
            topshow = topshow.push(container(row![text("Message:").size(20),
                                              text(&self.msg_value).size(20).color(*&self.mess_color),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
            ));
            topshow = topshow.push(container(row![button("Directory Button").on_press(Message::DirPressed),
                                              text(&self.dir_value).size(20),
                                              ].align_y(Alignment::Center).spacing(10).padding(5),
            ));
            topshow = topshow.push(container(row![button("Start Progress Button").on_press(Message::ProgressPressed),
                                              progress_bar(0.0..=100.0,self.progval),
                                              text(format!("{:.1}%", &self.progval)).size(30),
                                              ].align_y(Alignment::Center).spacing(5).padding(10),
            ));
            topshow = topshow.push(container(row![horizontal_space(), ua, horizontal_space(),
                                              ub, horizontal_space(), uc, horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(5).padding(10),
            ));

            let mut subshow = Column::new().spacing(10).align_x(Alignment::Center);

            match self.pagechoice_value  {
                PageChoice::CON => {
                    subshow = subshow.push(container(row![horizontal_space(),
                                                                         button("Out Directory Button").on_press(Message::C8OutDirPressed),
                                                                         text(&self.outdir_value).size(20), 
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                        ));
                    subshow = subshow.push(container(row![horizontal_space(),
                                                                         button("List Directory Button").on_press(Message::C8ListPressed),
                                                                         horizontal_space(),
                                                                         button("Copy Button").on_press(Message::C8CopyPressed), 
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                         ));
                    subshow = subshow.push(container(Scrollable::new(
                        Column::new()
                           .width(Length::Fill)
                           .align_x(Alignment::Center)
                           .push(
                              Text::new(format!("{}",&self.c8scrol_value)),
                           )
                           ).height(Length::Fill),
                        ),
                           );

                },
                PageChoice::ROT => {
                    subshow = subshow.push(container(row![horizontal_space(),
                                                                         button("List Orientation Button").on_press(Message::RCListPressed),
                                                                         horizontal_space(),
                                                                         button("Rotate All Button").on_press(Message::RCRotallPressed), 
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                         ));
                    subshow = subshow.push(container(Scrollable::new(
                        Column::new()
                           .width(Length::Fill)
                           .align_x(Alignment::Center)
                           .push(
                              Text::new(format!("{}",&self.rcscrol_value)),
                           )
                           ).height(Length::Fill),
                        ),
                           );

                },
                PageChoice::IND => {
                    subshow = subshow.push(container(row![horizontal_space(),
                                                                         button("Individual rotate start button").on_press(Message::INStartButton),
                                                                         horizontal_space(),
                                              ].align_y(Alignment::Center).spacing(10).padding(10),
                        ));

                },
           }
            
        column![

         topshow,
         subshow,
         ]
         .padding(1)
        .into()

    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
 
}

#[derive(Debug, Clone)]
pub struct Rotatex {
    errcolor: Color,
    errval: String,
}

impl Rotatex {

    pub async fn rotateit(dir_value: String, mergescrol_value: String, tx_send: mpsc::UnboundedSender<String>,) -> Result<Rotatex, Error> {
     let mut errstring  = " ".to_string();
     let mut colorx = Color::from([0.0, 1.0, 0.0]);
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
     let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
     let mut lenmg1 = mergelistvec.len();
     lenmg1 = lenmg1 -1;
     let start_time = Instant::now();
     for indl in 0..lenmg1 {
          let str_cur_dirfrom = dir_value.clone();
          let linestr = mergelistvec[indl];
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let filefromx = lineparse[0].to_string();
          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx[1..];
          if !Path::new(&fullfrom).exists() {
              errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
          let strval = lineparse[1].to_string();
          let locind = strval.find("orientation");
          if locind != None {
              let start = locind.unwrap();
              let start = start + 13;
              let end = start + 1;
              let getorient = strval.get(start..end);
              let orient_int: i32 = getorient.unwrap().parse().unwrap_or(-99);
              if orient_int > 0 {
                  if (orient_int == 3) | 
                     (orient_int == 6) |
                     (orient_int == 8) {
                      numrow = numrow + 1;
                      if numprocess < 4 {
                          stdCommand::new("/home/jp/gimp.sh")
                             .arg(&fullfrom)
                             .spawn()
                             .expect("failed to execute process");
                          numprocess = numprocess + 1;
                      } else {
                          let _output = stdCommand::new("/home/jp/gimp.sh")
                               .arg(&fullfrom)
                               .output()
                               .expect("failed to execute process");
                          numprocess = 0;
                          let msgx = format!("Progress|{}|{}", numrow, lenmg1);
                          tx_send.unbounded_send(msgx).unwrap();
                      }
                  }
              }
          }
     }
     if bolok {
         let diffx = start_time.elapsed();     
         errstring = format!("rotated {} files in {} seconds", lenmg1, diffx.as_secs());
         colorx = Color::from([0.0, 1.0, 0.0]);
     }
     Ok(Rotatex {
            errcolor: colorx,
            errval: errstring,
        })
    }
}

#[derive(Debug, Clone)]
struct Copyx {
    errcolor: Color,
    errval: String,
}

impl Copyx {

    async fn copyit(dir_value: String, outdir_value: String, mergescrol_value: String, tx_send: mpsc::UnboundedSender<String>,) -> Result<Copyx, Error> {
     let mut errstring  = " ".to_string();
     let mut colorx = Color::from([0.0, 1.0, 0.0]);
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
     let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
     let mut lenmg1 = mergelistvec.len();
     lenmg1 = lenmg1 -1;
     let start_time = Instant::now();
     for indl in 0..lenmg1 {
          let str_cur_dirfrom = dir_value.clone();
          let linestr = mergelistvec[indl];
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let filefromx = lineparse[0].to_string();
          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx[1..];
          if !Path::new(&fullfrom).exists() {
              errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
          let str_cur_dirout = outdir_value.clone();
          let fullto = str_cur_dirout.clone() + "/" + &filefromx;
          if Path::new(&fullto).exists() {
              errstring = format!("********* convert Copy: ERROR {} already exists **********", fullto);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
          if numprocess < 4 {
              stdCommand::new("magick")
                           .arg(&fullfrom)
                           .arg("-resize")
                           .arg("1920x1080")
                           .arg("-background")
                           .arg("black")
                           .arg("-gravity")
                           .arg("center")
                           .arg("-extent")
                           .arg("1920x1080")
                           .arg(&fullto)
                           .spawn()
                           .expect("failed to execute process");
              numprocess = numprocess + 1;
          } else {
              let _output = stdCommand::new("magick")
                           .arg(&fullfrom)
                           .arg("-resize")
                           .arg("1920x1080")
                           .arg("-background")
                           .arg("black")
                           .arg("-gravity")
                           .arg("center")
                           .arg("-extent")
                           .arg("1920x1080")
                           .arg(&fullto)
                           .output()
                           .expect("failed to execute process");
              numprocess = 0;
              let msgx = format!("Progress|{}|{}", numrow, lenmg1);
              tx_send.unbounded_send(msgx).unwrap();

          }

          numrow = numrow + 1;
     }
     if bolok {
         let diffx = start_time.elapsed();     
         errstring = format!("converted copied {} files in {} seconds", lenmg1, diffx.as_secs());
         colorx = Color::from([0.0, 1.0, 0.0]);
     }
     Ok(Copyx {
            errcolor: colorx,
            errval: errstring,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Error {
//    APIError,
}
// loop thru by sleeping for 5 seconds
#[derive(Debug, Clone)]
pub struct Progstart {
//    errcolor: Color,
//    errval: String,
}

impl Progstart {

    pub async fn pstart() -> Result<Progstart, Error> {
//     let errstring  = " ".to_string();
//     let colorx = Color::from([0.0, 0.0, 0.0]);
     sleep(Duration::from_secs(5));
     Ok(Progstart {
//            errcolor: colorx,
//            errval: errstring,
        })
    }
}

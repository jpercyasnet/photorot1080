mod rc_mod;
mod c8_mod;
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

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container, Text, text, column, button, row, progress_bar};
use iced::{Element, Command, Length, Application, Settings, Alignment, executor, Color, window};
use iced_aw::{TabLabel, Tabs};

use rc_mod::{RcMessage, RcTab};
use c8_mod::{C8Message, C8Tab};

use iced::theme::{Theme};

mod counter;
use counter::{CounterMessage, CounterTab};

mod settings;
use settings::{SettingsMessage, SettingsTab, TabBarPosition};

use std::path::{Path};
use std::process::Command as stdCommand;
use std::time::{Duration, Instant};
use std::thread::sleep;
use iced_futures::futures;
use futures::channel::mpsc;
use crate::rc_rotatepress::rc_rotatepress;
use c8_copypress::c8_copypress;

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;

fn main() -> iced::Result {
     let mut widthxx: u32 = 1350;
     let mut heightxx: u32 = 750;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho - 20;
         heightxx = heighto - 75;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }

     TabBarExample::run(Settings {
        window: window::Settings {
            size: (widthxx, heightxx),
            ..window::Settings::default()
        },
        ..Settings::default()
     })
//    TabBarExample::run(Settings::default())
}

struct TabBarExample {
    dir_value: String,
    mess_color: Color,
    msg_value: String,
    do_progress: bool,
    progval: f32,
    tx_send: mpsc::UnboundedSender<String>,
    rx_receive: mpsc::UnboundedReceiver<String>,
    active_tab: usize,
    rc_tab: RcTab,
    c8_tab: C8Tab,
    counter_tab: CounterTab,
    settings_tab: SettingsTab,
}

#[derive(Clone, Debug)]
enum Message {
    DirPressed,
    TabSelected(usize),
    RC(RcMessage),
    C8(C8Message),
    Counter(CounterMessage),
    Settings(SettingsMessage),
    ProgressPressed,
    ProgRtn(Result<Progstart, Error>),
    RotatexFound(Result<Rotatex, Error>),
    CopyxFound(Result<Copyx, Error>),
}

impl Application for TabBarExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let (tx_send, rx_receive) = mpsc::unbounded();
        (  TabBarExample {
                dir_value: "no directory".to_string(),
                mess_color: Color::from([0.0, 0.0, 0.0]),
                msg_value: "no message".to_string(),
                do_progress: false,
                progval: 0.0,
                tx_send,
                rx_receive,
            active_tab: 0,
            rc_tab: RcTab::new(),
            c8_tab: C8Tab::new(),
            counter_tab: CounterTab::new(),
            settings_tab: SettingsTab::new(),
           },
            Command::none(),
        )

    }

    fn title(&self) -> String {
        String::from("TabBar Example")
    }

    fn update(&mut self, message: Message) -> Command<Message>  {
        match message {
            Message::TabSelected(selected) => {self.active_tab = selected;Command::none()}
            Message::C8(C8Message::CopyPressed(outdir, scrol)) => {
               let (errcode, errstr) = c8_copypress(self.dir_value.clone(), outdir.clone(), scrol.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   Command::perform(Copyx::copyit(self.dir_value.clone(), outdir.clone(), scrol.clone(), self.tx_send.clone()), Message::CopyxFound)
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   Command::none()
               }
            }
            Message::RC(RcMessage::RotallPressed) => {
                if !Path::new(&self.dir_value).exists() {
                    self.msg_value = format!("directory does not exist: {}", self.dir_value);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    Command::none()
                } else {
                    let dir_path = Path::new(&self.dir_value);
                    let (errcd, errstr, newliststr) = get_dirlistr(dir_path.to_path_buf());
                    if errcd == 0 {
                        let (errrc, errstrrc) = rc_rotatepress(self.dir_value.clone(), newliststr.clone());
                        self.msg_value = errstrrc.to_string();
                        if errrc == 0 {
                            self.mess_color = Color::from([0.0, 1.0, 0.0]);
                            Command::perform(Rotatex::rotateit(self.dir_value.clone(), newliststr.clone(), self.tx_send.clone()), Message::RotatexFound)
                        } else {
                            self.mess_color = Color::from([1.0, 0.0, 0.0]);
                            Command::none()
                        }
                    } else {
                        self.msg_value = errstr.to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                        Command::none()
                    }
                }
            }
            Message::RC(message) => {self.rc_tab.update(message);Command::none()}
            Message::C8(message) => {self.c8_tab.update(message);Command::none()}
            Message::Counter(message) => {self.counter_tab.update(message);Command::none()}
            Message::Settings(message) => {self.settings_tab.update(message);Command::none()}
            Message::DirPressed => {
               let (errcode, errstr, newdir, _newliststr) = dirpress();
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.dir_value = newdir.to_string();
                   self.rc_tab.update(RcMessage::Dirupdate(self.dir_value.clone()));
                   self.c8_tab.update(C8Message::Dirupdate(self.dir_value.clone()));
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Command::none()
            }
            Message::ProgressPressed => {
                   self.do_progress = true;
                   Command::perform(Progstart::pstart(), Message::ProgRtn)
            }
            Message::CopyxFound(Ok(copyx)) => {
                self.msg_value = copyx.errval.clone();
                self.mess_color = copyx.errcolor.clone();
                self.do_progress = false;
                self.progval = 0.0;
                Command::none()
            }
            Message::CopyxFound(Err(_error)) => {
                self.msg_value = "error in copyx copyit routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
                Command::none()
            }
            Message::RotatexFound(Ok(copyx)) => {
                self.msg_value = copyx.errval.clone();
                self.mess_color = copyx.errcolor.clone();
                self.do_progress = false;
                self.progval = 0.0;
               Command::none()
            }
            Message::RotatexFound(Err(_error)) => {
                self.msg_value = "error in copyx copyit routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Command::none()
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
                        let prog1 = progvec[0].clone().to_string();
                        if prog1 == "Progress" {
                            let num_int: i32 = progvec[1].clone().parse().unwrap_or(-9999);
                            if num_int == -9999 {
                                println!("progress numeric not numeric: {}", inputval);
                            } else {
                                let dem_int: i32 = progvec[2].clone().parse().unwrap_or(-9999);
                                if dem_int == -9999 {
                                    println!("progress numeric not numeric: {}", inputval);
                                } else {
                                    self.progval = 100.0 * (num_int as f32 / dem_int as f32);
                                    self.msg_value = format!("Convert progress: {}", self.progval);
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
                Command::perform(Progstart::pstart(), Message::ProgRtn)
              } else {
                Command::none()
              }
            }
            Message::ProgRtn(Err(_error)) => {
                self.msg_value = "error in Progstart::pstart routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let position = self
            .settings_tab
            .settings()
            .tab_bar_position
            .unwrap_or_default();
        let theme = self
            .settings_tab
            .settings()
            .tab_bar_theme
            .unwrap_or_default();
        column![
            row![text("Message:").size(20),
                 text(&self.msg_value).size(20).style(*&self.mess_color),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("Directory Button").on_press(Message::DirPressed),
                 text(&self.dir_value).size(20),
            ].align_items(Alignment::Center).spacing(10).padding(5),
            row![button("Start Progress Button").on_press(Message::ProgressPressed),
                 progress_bar(0.0..=100.0,self.progval),
                 text(format!("{}%", &self.progval)).size(30),

            ].align_items(Alignment::Center).spacing(5).padding(10),

        Tabs::new(self.active_tab, Message::TabSelected)
            .push(self.rc_tab.tab_label(), self.rc_tab.view())
            .push(self.c8_tab.tab_label(), self.c8_tab.view())
            .push(self.counter_tab.tab_label(), self.counter_tab.view())
            .push(self.settings_tab.tab_label(), self.settings_tab.view())
            .tab_bar_style(theme)
//            .icon_font(ICON_FONT)
            .tab_bar_position(match position {
                TabBarPosition::Top => iced_aw::TabBarPosition::Top,
                TabBarPosition::Bottom => iced_aw::TabBarPosition::Bottom,
            }).height(iced::Length::Fixed(600.0))
         ]
         .padding(10)
//        .align_items(Alignment::Start)
        .into()
//            .into()

    }
}

trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}
#[derive(Debug, Clone)]
pub struct Rotatex {
    errcolor: Color,
    errval: String,
}

impl Rotatex {

    pub async fn rotateit(dir_value: String, mergescrol_value: String, tx_send: mpsc::UnboundedSender<String>,) -> Result<Rotatex, Error> {
     let mut errstring  = " ".to_string();
     let mut colorx = Color::from([0.0, 0.0, 0.0]);
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
     let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
     let mut lenmg1 = mergelistvec.len();
     lenmg1 = lenmg1 -1;
     let start_time = Instant::now();
     for indl in 0..lenmg1 {
          let str_cur_dirfrom = dir_value.clone();
          let linestr = mergelistvec[indl].clone();
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let filefromx = lineparse[0].clone().to_string();
          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx[1..];
          if !Path::new(&fullfrom).exists() {
              errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
          let strval = lineparse[1].clone().to_string();
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
         colorx = Color::from([0.0, 0.0, 0.0]);
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
     let mut colorx = Color::from([0.0, 0.0, 0.0]);
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
     let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
     let mut lenmg1 = mergelistvec.len();
     lenmg1 = lenmg1 -1;
     let start_time = Instant::now();
     for indl in 0..lenmg1 {
          let str_cur_dirfrom = dir_value.clone();
          let linestr = mergelistvec[indl].clone();
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let filefromx = lineparse[0].clone().to_string();
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
              stdCommand::new("convert")
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
              let _output = stdCommand::new("convert")
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
         colorx = Color::from([0.0, 0.0, 0.0]);
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

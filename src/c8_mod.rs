use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Button, Column, Container, Row, Text, Scrollable},
    Alignment, Element, Length, Color
};
use iced_aw::tab_bar::TabLabel;
use std::path::{Path};
use crate::{Message, Tab};
use crate::c8_diroutpress::c8_diroutpress;
use crate::get_dirlistc::get_dirlistc;

#[derive(Debug, Clone)]
pub enum C8Message {
    Dirupdate(String),
    ListPressed,
    OutDirPressed,
    CopyPressed(String, String),
}

pub struct C8Tab {
    c8dirval: String,
    mess_color: Color,
    msg_value: String,
    outdir_value: String,
    scrol_value: String,
}

impl C8Tab {
    pub fn new() -> Self {
        C8Tab {
            c8dirval: String::new(),
            outdir_value: " No directory selected".to_string(),
            mess_color: Color::from([0.0, 0.0, 0.0]),
            msg_value: "no message".to_string(),
            scrol_value: " No directory selected \n ".to_string(),
        }
    }

    pub fn update(&mut self, message: C8Message){
        match message {
            C8Message::Dirupdate(value) => {self.c8dirval = value;},
            C8Message::CopyPressed(_outdir, _scrol) => {self.msg_value = "Copy Pressed".to_string();},
            C8Message::OutDirPressed => {
                let mut a_dir: String = self.c8dirval.clone().to_string();
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
            }
            C8Message::ListPressed => {
                if !Path::new(&self.c8dirval).exists() {
                    self.msg_value = format!("directory does not exist: {}", self.c8dirval);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else {
                    let dir_path = Path::new(&self.c8dirval);
                    let (errcd, errstr, newliststr) = get_dirlistc(dir_path.to_path_buf());
                    if errcd == 0 {
                        self.scrol_value  = newliststr.to_string();
                        self.msg_value = format!("directory entries for: {}", self.c8dirval);
                        self.mess_color = Color::from([0.0, 1.0, 0.0]);
                    } else {
                        self.msg_value = errstr.to_string();
                        self.mess_color = Color::from([1.0, 0.0, 0.0]);
                    }
                }
            }
        }
    }
}

impl Tab for C8Tab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Convert to 1080")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text("Convert 1080".to_string())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, C8Message> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(1300)
                .padding(20)
                .spacing(16)
                .push(
                    Row::new()
                        .spacing(10)
                        .padding(10)
                        .push(
                            Text::new(" C8 Message:").size(20),
                        )
                        .push(
                            Text::new(&self.msg_value).size(20).style(*&self.mess_color),
                        ),
                )
                .push(
                    Row::new()
                        .spacing(10)
                        .padding(10)
                        .push(
                            Button::new(
                                Text::new("Out Directory Button").horizontal_alignment(Horizontal::Center),
                            )
                            .width(Length::Fixed(200.0))
                            .on_press(C8Message::OutDirPressed),
                        )
                        .push(
                            Text::new(&self.outdir_value).size(20),
                        ),
                )
                .push(
                    Row::new()
                        .spacing(100)
                        .push(
                            Button::new(
                                Text::new("List Directory Button").horizontal_alignment(Horizontal::Center),
                            )
                            .width(Length::Fixed(200.0))
                            .on_press(C8Message::ListPressed),
                        )
                        .push(
                            Button::new(
                                Text::new("Copy Button").horizontal_alignment(Horizontal::Center),
                            )
                            .width(Length::Fixed(200.0))
                            .on_press(C8Message::CopyPressed(self.outdir_value.clone(), self.scrol_value.clone())),
                        )
                )
                .push(
                    Scrollable::new(
                           Column::new()
                              .width(Length::Fill)
                              .push(
                                 Text::new(format!("{}",&self.scrol_value)),
                              )
                    ).height(Length::Fixed(700.0)),
                )
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into();

        content.map(Message::C8)
    }
}


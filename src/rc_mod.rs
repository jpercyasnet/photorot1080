use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Button, Column, Container, Row, Text, Scrollable},
    Alignment, Element, Length, Color
};
use iced_aw::tab_bar::TabLabel;
use std::path::{Path};
use crate::{Message, Tab};
use crate::get_dirlistr::get_dirlistr;

#[derive(Debug, Clone)]
pub enum RcMessage {
    Dirupdate(String),
    ListPressed,
    RotallPressed,
}

pub struct RcTab {
    rcdirval: String,
    mess_color: Color,
    msg_value: String,
    scrol_value: String,
}

impl RcTab {
    pub fn new() -> Self {
        RcTab {
            rcdirval: String::new(),
            mess_color: Color::from([0.0, 0.0, 0.0]),
            msg_value: "no message".to_string(),
            scrol_value: " nothing to process ".to_string(),
        }
    }

    pub fn update(&mut self, message: RcMessage){
        match message {
            RcMessage::Dirupdate(value) => {self.rcdirval = value;},
            RcMessage::RotallPressed => {self.msg_value = "Rotate All Pressed".to_string();},
            RcMessage::ListPressed => {
                self.scrol_value = " nothing to process ".to_string();
                if !Path::new(&self.rcdirval).exists() {
                    self.msg_value = format!("directory does not exist: {}", self.rcdirval);
                    self.mess_color = Color::from([1.0, 0.0, 0.0]);
                } else {
                    let dir_path = Path::new(&self.rcdirval);
                    let (errcd, errstr, newliststr) = get_dirlistr(dir_path.to_path_buf());
                    if errcd == 0 {
                        self.scrol_value  = newliststr.to_string();
                        self.msg_value = format!("directory entries for: {}", self.rcdirval);
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

impl Tab for RcTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Rotate Correction")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text("rotate correction".to_string())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, RcMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(1300)
                .padding(5)
                .spacing(5)
                .push(
                    Row::new()
                        .spacing(10)
                        .padding(10)
                        .push(
                            Text::new(" RC Message:").size(20),
                        )
                        .push(
                            Text::new(&self.msg_value).size(20).style(*&self.mess_color),
                        ),
                )
                .push(
                    Row::new()
                        .spacing(100)
                        .push(
                            Button::new(
                                Text::new("List Orientation Button").horizontal_alignment(Horizontal::Center),
                            )
                            .width(Length::Fill)
                            .on_press(RcMessage::ListPressed),
                        )
                        .push(
                            Button::new(
                                Text::new("Rotate All Button").horizontal_alignment(Horizontal::Center),
                            )
                            .width(Length::Fill)
                            .on_press(RcMessage::RotallPressed),
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

        content.map(Message::RC)
    }
}


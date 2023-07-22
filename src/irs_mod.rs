use iced::{
    widget::{Button, Column, Container, Text},
    Alignment, Element,
};
use iced_aw::tab_bar::TabLabel;

//use crate::{Icon, Message, Tab};
use crate::{Message, Tab};

#[derive(Debug, Clone)]
pub enum IrsMessage {
    StartButton,
}

pub struct IrsTab {
    value: String,
}

impl IrsTab {
    pub fn new() -> Self {
        IrsTab { value: "Individual rotate start".to_string()}
    }
/*
    pub fn update(&mut self, message: IrsMessage) {
        match message {
            IrsMessage::StartButton => {
              println!("indivdual rotate button pressed {}", self.value);
            },
        }
    } */
}

impl Tab for IrsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from(self.value.clone())
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text("individual rotation start".to_string())
//        TabLabel::IconText(Icon::Calc.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, IrsMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(1300)
                .padding(20)
                .spacing(16)
                .push(Button::new(Text::new("Individual rotate start button")).on_press(IrsMessage::StartButton)),
//                ),
        )
        .into();

        content.map(Message::IRS)
    }
}


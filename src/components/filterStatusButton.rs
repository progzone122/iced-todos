use crate::Message;
use iced::widget::{button, row, Container};
use iced::{Element, Length};
use iced::alignment::Horizontal;

pub struct FilterStatusButtons {
    pub active_state: u8,
    all: button::State,
    active: button::State,
    done: button::State,
}
#[derive(Clone, Debug)]
pub enum FilterStatusButtonsMessage {
    AllClicked,
    ActiveClicked,
    DoneClicked,
}

impl FilterStatusButtons {
    pub fn new() -> Self {
        Self {
            active_state: 0,
            all: button::State::new(),
            active: button::State::new(),
            done: button::State::new(),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        Container::new(
            row![
                button("All").on_press(Message::FilterStatusButtonsMessage(
                    FilterStatusButtonsMessage::AllClicked
                )),
                button("Active").on_press(Message::FilterStatusButtonsMessage(
                    FilterStatusButtonsMessage::ActiveClicked
                )),
                button("Done").on_press(Message::FilterStatusButtonsMessage(
                    FilterStatusButtonsMessage::DoneClicked
                )),
            ]
                .spacing(10),
        )
            .width(Length::Fill)
            .align_x(Horizontal::Right)
            .padding(15)
            .into()
    }
    pub fn update(&mut self, message: FilterStatusButtonsMessage) {
        match message {
            FilterStatusButtonsMessage::AllClicked => {
                println!("All button clicked");
            }
            FilterStatusButtonsMessage::ActiveClicked => {
                println!("Active button clicked");
            }
            FilterStatusButtonsMessage::DoneClicked => {
                println!("Done button clicked");
            }
        }
    }
}

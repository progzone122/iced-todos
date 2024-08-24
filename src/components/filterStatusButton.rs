use crate::Message;
use iced::widget::{button, row, Container};
use iced::{Element, Length};
use iced::alignment::Horizontal;

pub struct FilterStatusButtons {
    pub active_state: u8,
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
                self.active_state = 0;
            }
            FilterStatusButtonsMessage::ActiveClicked => {
                self.active_state = 1;
            }
            FilterStatusButtonsMessage::DoneClicked => {
                self.active_state = 2;
            }
        }
    }
}
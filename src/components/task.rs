use iced::{Element, Length};
use iced::widget::{checkbox, row, text};
use crate::Message;

pub struct Task {
    pub id: usize,
    text: String,
    is_checked: bool
}
#[derive(Clone, Debug)]
pub enum TaskMessage {
    CheckboxToggled(bool)
}

impl Task {
    pub fn new(id: usize, text: &str) -> Self {
        Self {
            id,
            text: text.to_string(),
            is_checked: false
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        row![
            text(&self.text)
                .size(30)
                .width(Length::Fill),

            checkbox(&self.id.to_string(), self.is_checked)
                .on_toggle(|is_checked| Message::TaskMessage(self.id, TaskMessage::CheckboxToggled(is_checked)))
                .size(30)
        ].into()
    }
    pub fn update(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::CheckboxToggled(is_checked) => {
                self.is_checked = is_checked;
            }
        }
    }
}
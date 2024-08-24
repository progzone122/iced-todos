use iced::{Element, Length};
use iced::widget::{button, checkbox, row, text, svg};
use crate::Message;

pub struct Task {
    pub id: usize,
    text: String,
    pub is_checked: bool,
}
#[derive(Clone, Debug)]
pub enum TaskMessage {
    CheckboxToggled(bool),
    DeleteButtonPressed
}

impl Task {
    pub fn new(id: usize, text: &str) -> Self {
        Self {
            id,
            text: text.to_string(),
            is_checked: false,
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let icon = svg(svg::Handle::from_memory(include_bytes!("../images/delete_forever.svg").as_slice()));
        row![
            text(&self.text)
                .size(30)
                .width(Length::Fill),

            row![
                button(icon)
                    .on_press(Message::TaskMessage(self.id, TaskMessage::DeleteButtonPressed))
                    .width(30)
                    .height(30),

                checkbox("", self.is_checked)
                    .on_toggle(|is_checked| Message::TaskMessage(self.id, TaskMessage::CheckboxToggled(is_checked)))
                    .size(30)
            ]
                .spacing(10),
        ].into()
    }
    pub fn update(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::CheckboxToggled(is_checked) => {
                self.is_checked = is_checked;
            }
            TaskMessage::DeleteButtonPressed => {}
        }
    }
}
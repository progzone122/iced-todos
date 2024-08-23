mod components;

use iced::widget::{column, Container, row, text, text_input, button};
use iced::{executor, Application, Command, Element, Renderer, Settings, Theme, Length};

// components
use components::filterStatusButton::{FilterStatusButtons, FilterStatusButtonsMessage};
use components::task::{Task, TaskMessage};

struct ToDoList {
    input1: String,
    // components
    filter_status_buttons: FilterStatusButtons,
    tasks: Vec<Task>
}

#[derive(Debug, Clone)]
pub enum Message {
    Input1Changed(String),
    Input1Submit,
    // components
    FilterStatusButtonsMessage(FilterStatusButtonsMessage),
    TaskMessage(usize, TaskMessage)
}

impl Application for ToDoList {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                input1: "".to_string(),
                // компоненты
                filter_status_buttons: FilterStatusButtons::new(),
                tasks: vec![]
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("ToDo List")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Input1Changed(input) => {
                self.input1 = input;
            }
            Message::Input1Submit => {
                if !self.input1.is_empty() {
                    println!("input1: {}", &self.input1);
                    self.tasks.push(Task::new(self.tasks.len(), &self.input1));
                    self.input1.clear();
                }
            }
            // components
            Message::FilterStatusButtonsMessage(filter_message) => {
                self.filter_status_buttons.update(filter_message);
            }
            Message::TaskMessage(index, task_message) => {
                if let Some(task) = self.tasks.iter_mut().find(|task| task.id == index) {
                    task.update(task_message);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        let tasks_column = column(self.tasks.iter().map(|task| task.view()).collect::<Vec<_>>())
            .spacing(25)
            .padding(45);

        column![
            Container::new(
                text("ToDo List").size(50),
            )
                .width(Length::Fill)
                .center_x()
                .padding(15),

            self.filter_status_buttons.view(),

            Container::new(
                text_input("I'm currently working on...", &self.input1)
                    .on_input(Message::Input1Changed)
                    .on_submit(Message::Input1Submit)
                    .size(30)
            )
                .width(Length::Fill)
                .center_x()
                .padding([7, 30, 0, 30]),

            tasks_column
        ]
            .into()
    }
}

fn main() -> iced::Result {
    ToDoList::run(Settings::default())
}

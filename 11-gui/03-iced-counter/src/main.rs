use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::Reset => {
                self.value = 0;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            text("Counter Example").size(32),
            text(self.value).size(64),
            row![
                button("Decrement").on_press(Message::Decrement),
                button("Reset").on_press(Message::Reset),
                button("Increment").on_press(Message::Increment),
            ]
            .spacing(10),
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

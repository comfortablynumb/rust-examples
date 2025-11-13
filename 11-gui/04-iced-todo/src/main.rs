use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    TodoApp::run(Settings::default())
}

#[derive(Default)]
struct TodoApp {
    input: String,
    tasks: Vec<Task>,
    filter: Filter,
}

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    CreateTask,
    ToggleTask(usize),
    DeleteTask(usize),
    FilterChanged(Filter),
    ClearCompleted,
}

impl Sandbox for TodoApp {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Todo App - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input = value;
            }
            Message::CreateTask => {
                if !self.input.is_empty() {
                    let id = self.tasks.len();
                    self.tasks.push(Task {
                        id,
                        description: self.input.clone(),
                        completed: false,
                    });
                    self.input.clear();
                }
            }
            Message::ToggleTask(id) => {
                if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                    task.completed = !task.completed;
                }
            }
            Message::DeleteTask(id) => {
                self.tasks.retain(|task| task.id != id);
            }
            Message::FilterChanged(filter) => {
                self.filter = filter;
            }
            Message::ClearCompleted => {
                self.tasks.retain(|task| !task.completed);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let title = text("Todo List").size(32);

        let input = text_input("What needs to be done?", &self.input)
            .on_input(Message::InputChanged)
            .on_submit(Message::CreateTask)
            .padding(10);

        let tasks = self
            .tasks
            .iter()
            .filter(|task| match self.filter {
                Filter::All => true,
                Filter::Active => !task.completed,
                Filter::Completed => task.completed,
            })
            .fold(column![].spacing(10), |column, task| {
                column.push(
                    row![
                        checkbox("", task.completed)
                            .on_toggle(move |_| Message::ToggleTask(task.id)),
                        text(&task.description),
                        button("Delete").on_press(Message::DeleteTask(task.id)),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center),
                )
            });

        let tasks_scrollable = scrollable(tasks);

        let filters = row![
            filter_button("All", self.filter, Filter::All),
            filter_button("Active", self.filter, Filter::Active),
            filter_button("Completed", self.filter, Filter::Completed),
        ]
        .spacing(10);

        let completed_count = self.tasks.iter().filter(|t| t.completed).count();
        let active_count = self.tasks.len() - completed_count;

        let footer = row![
            text(format!("{} items left", active_count)),
            button("Clear completed").on_press(Message::ClearCompleted),
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        let content = column![title, input, tasks_scrollable, filters, footer]
            .spacing(20)
            .padding(20)
            .max_width(800);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .into()
    }
}

fn filter_button(label: &str, current: Filter, filter: Filter) -> Element<Message> {
    let button = button(text(label));

    if current == filter {
        button.into()
    } else {
        button.on_press(Message::FilterChanged(filter)).into()
    }
}

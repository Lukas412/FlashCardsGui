use iced::widget::column;
use iced::{Element, Sandbox, Settings};

fn main() -> iced::Result {
    FlashCards::run(Settings::default())
}

struct FlashCards {
    counter: i8,
}

impl Sandbox for FlashCards {
    type Message = ();

    fn new() -> Self {
        Self { counter: 0 }
    }

    fn title(&self) -> String {
        "Karteikarten App (Rust)".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![
            iced::widget::text("hallo"),
            iced::widget::text("other  text")
        ]
        .into()
    }
}

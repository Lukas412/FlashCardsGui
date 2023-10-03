use flash_card_parser::Topic;
use iced::widget::{button, column, text};
use iced::{Element, Sandbox, Settings};
use crate::learning::LearningElement;

mod learning;

fn main() -> iced::Result {
    FlashCardsApp::run(Settings::default())
}

struct FlashCardsApp {
    learning: LearningElement,
}

impl Sandbox for FlashCardsApp {
    type Message = ;

    fn new() -> Self {
        Self {  }
    }

    fn title(&self) -> String {
        "Karteikarten App (Rust)".to_owned()
    }

    fn update(&mut self, message: Self::Message) {}

    fn view(&self) -> Element<'_, Self::Message> {
        column![
            text("hallo"),
            text("other  text"),
            button("ajsldfkjsalkjf").on_press(())
        ]
        .into()
    }
}

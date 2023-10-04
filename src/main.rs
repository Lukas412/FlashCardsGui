use crate::learning::{LearningAction, LearningElement};
use derive_more::From;
use iced::widget::{button, column, text};
use iced::{Element, Sandbox, Settings};

mod learning;

fn main() -> iced::Result {
    FlashCardsApp::run(Settings::default())
}

struct FlashCardsApp {
    learning: LearningElement,
}

#[derive(Debug, Clone, Copy, From)]
enum AppActions {
    Learning(LearningAction),
}

impl Sandbox for FlashCardsApp {
    type Message = AppActions;

    fn new() -> Self {
        Self {
            learning: LearningElement::new()
        }
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

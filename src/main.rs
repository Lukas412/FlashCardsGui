use iced::widget::{button, column, text, Column};
use iced::{Element, Sandbox, Settings};

fn main() -> iced::Result {
    FlashCards::run(Settings::default())
}

enum FlashCards {
    QuestionPage { question: String },
    AnswerPage { answer: String },
}

impl Sandbox for FlashCards {
    type Message = ();

    fn new() -> Self {
        FlashCards::QuestionPage {
            question: "Das ist die Fragekarte.".to_owned(),
        }
    }

    fn title(&self) -> String {
        "Karteikarten App (Rust)".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![text("hallo"), text("other  text")].into()
    }
}

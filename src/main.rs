use iced::widget::{button, column, text};
use iced::{Element, Sandbox, Settings};

fn main() -> iced::Result {
    FlashCardsApp::run(Settings::default())
}

struct FlashCardsApp {}

impl Sandbox for FlashCardsApp {
    type Message = ();

    fn new() -> Self {
        Self { counter: 0 }
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

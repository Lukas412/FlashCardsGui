use flash_card_parser::Topic;
use iced::widget::{button, text, Column};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub(crate) struct LearningElement {
    topic: Topic<'static>,
    page: LearningPage,
}

impl LearningElement {
    pub(crate) fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        let topic = serde_json::from_reader(buffer)?;
        Ok(Self {
            topic,
            page: LearningPage::new(),
        })
    }
}

enum LearningPage {
    Question,
    Answer,
}

#[derive(Debug, Clone, Copy)]
pub enum LearningAction {
    Uncover,
    Right,
    Wrong,
}

impl LearningPage {
    fn new() -> Self {
        Self::Question
    }

    fn view(&mut self) -> Column<LearningAction> {
        Column::with_children(vec![
            text("Was ist eine Frage?").into(),
            button("Aufdecken").on_press(LearningAction::Uncover).into(),
        ])
    }
}

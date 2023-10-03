use flash_card_parser::Topic;
use iced::widget::Column;

pub(crate) struct LearningElement {
    topic: Topic<'static>,
    page: LearningPage,
}

enum LearningPage {
    Question,
    Answer,
}

enum LearningAction {
    Uncover,
    Right,
    Wrong,
}

impl LearningPage {
    fn view(&mut self) -> Column<LearningAction> {}
}

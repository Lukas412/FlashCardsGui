pub fn parse_topic_from_str(string: &str) -> Result<Topic, TopicParseError> {}

pub enum TopicParseError {}

pub struct OwnedTopic {
    name: Box<str>,
    cards: Vec<OwnedCard>,
}

pub struct OwnedCard {
    question: Box<str>,
    answer: Box<str>,
}

pub struct Topic<'a> {
    name: &'a str,
    cards: Vec<Card<'a>>,
}

pub struct Card<'a> {
    question: &'a str,
    answer: &'a str,
}

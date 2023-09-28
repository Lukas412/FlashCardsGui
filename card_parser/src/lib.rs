use derive_more::{Display, Error, From};
use nom::bytes::complete::take_until;
use nom::error::ParseError;
use nom::{IResult, Parser};

pub fn parse_topic_from_str(input: &str) -> Result<Topic, TopicParseError> {
    let (input, title) = take_topic_title(input)?;
    let (input, cards) = take_cards(input)?;
    if input.trim() != "" {
        return Err(TopicParseError::ExpectedToParseToTheEnd { remaining: input });
    }
    Ok(Topic { title, cards })
}

fn take_topic_title<'a, Error>(
    input: &'a str,
) -> Result<(&'a str, Box<str>), TitleParseError<'a, Error>>
where
    Error: ParseError<&'a str>,
{
    let (mut input, title) = take_until("\n")(input)
        .map_err(|error| TitleParseError::ExpectedFirstLine { inner: error })?;
    let title = title.trim();
    if title == "" {
        return Err(TitleParseError::ExpectedTitleOnFirstLineButWasEmpty);
    }
    loop {
        let (remaining, line) =
            take_until("\n")(input).map_err(|error| TitleParseError::ExpectedLinesAfterTitle {
                inner: error.into(),
            })?;
        match line.trim() {
            "" => input = remaining,
            "===" => return Ok((input, title.trim().into())),
            value => {
                return Err(TitleParseError::ExpectedCardSeparatorAfterTitle {
                    actual: value.into(),
                })
            }
        }
    }
}

fn take_cards(input: &str) -> IResult<&str, Vec<Card>> {}

#[derive(Debug, Display, From, Error)]
pub enum TopicParseError<'a, Error> {
    Title(TitleParseError<'a, Error>),
    ExpectedToParseToTheEnd { remaining: &'a str },
}

#[derive(Debug, Display, Error)]
pub enum TitleParseError<'a, Error> {
    ExpectedFirstLine { inner: Error },
    ExpectedLinesAfterTitle { inner: Error },
    ExpectedTitleOnFirstLineButWasEmpty,
    ExpectedCardSeparatorAfterTitle { actual: &'a str },
}

pub struct Topic {
    title: Box<str>,
    cards: Vec<Card>,
}

pub struct Card {
    question: Box<str>,
    answer: Box<str>,
}

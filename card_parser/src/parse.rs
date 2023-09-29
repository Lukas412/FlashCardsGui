use derive_more::{Display, Error, From};
use nom::bytes::complete::{tag, take_until};
use nom::FindSubstring;

pub(crate) fn topic_title(input: &str) -> Result<(&str, &str), ExpectedTopicTitle> {
    let (input, line) = trimed_line(input);
    if line == "" {
        return Err(ExpectedTopicTitle::EmptyTitle);
    }
    let input = take_empty_lines(line);
    if input == "" {
        return Ok(("", line));
    };
    let input = take_card_separator(input)?;
    Ok((input, line))
}

#[derive(Debug, Display, Error, From)]
pub enum ExpectedTopicTitle<'a> {
    #[display(fmt = "Expected a card separator after the Title.\n{inner}")]
    CardSeparator { inner: ExpectedCardSeparator<'a> },
    #[display(
        fmt = "The title of a topic cannot be empty. Please specigy a title in the first line of the file."
    )]
    EmptyTitle,
}

fn cards(input: &str) -> Result<(&str, &str), ExpectedCard> {}

fn card(input: &str) -> Result<(&str, &str), ExpectedCard> {}

#[derive(Debug, Display, Error, From)]
pub enum ExpectedCard {}

fn take_card_separator(input: &str) -> Result<&str, ExpectedCardSeparator> {
    let (input, line) = trimed_line(input);
    if line != "---" {
        return Err(ExpectedCardSeparator { line });
    }
    Ok(input)
}

#[derive(Debug, Display, Error, From)]
pub struct ExpectedCardSeparator<'a> {
    line: &'a str,
}

fn take_card_divider(input: &str) -> Result<&str, ExpectedCardDivider> {
    let (input, line) = trimed_line(input);
    if line != "&" {
        return Err(ExpectedCardDivider { line });
    }
    Ok(input)
}

pub struct ExpectedCardDivider<'a> {
    line: &'a str,
}

fn take_empty_lines(mut input: &str) -> &str {
    loop {
        let (remaining, line) = trimed_line(input);
        if line == "" || input == "" {
            return input;
        }
        input = remaining;
    }
}

fn trimed_line(input: &str) -> (&str, &str) {
    let (input, line) = line(input);
    let line = line.trim();
    (input, line)
}

fn line(input: &str) -> (&str, &str) {
    match input.find_substring("\n") {
        Some(index) => {
            let (input, line) = input.split_at(index);
            let input = &input[1..];
            (input, line)
        }
        None => ("", input),
    }
}

pub struct ExpectedLine<'a> {
    input: &'a str,
}

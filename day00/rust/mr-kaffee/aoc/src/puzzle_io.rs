use regex::Regex;
use reqwest::{blocking::RequestBuilder, StatusCode};
use std::{
    env, fs,
    io::{Error, ErrorKind},
};

const REPO: &str = "github.com/mr-kaffee/aoc-2022/day00/rust/mr-kaffee/aoc";
const AUTHOR: &str = "Peter Wieland (peter@die-wielands.net)";

/// Load the input using a [`PuzzleIO`] constructed from a session cookie loaded from the given `path` or
/// from `session.cookie` if `path` is `None`.
pub fn load_input(path: Option<&str>, year: u16, day: u16) -> Result<String, Error> {
    let session = fs::read_to_string(path.unwrap_or("sesssion.cookie"))?;
    PuzzleIO::from(session.trim()).load_input(year, day)
}

pub fn submit_results<S: std::fmt::Display>(
    path: &str,
    year: u16,
    day: u16,
    result1: Option<S>,
    result2: Option<S>,
) -> Result<(), Error> {
    for (result, star, env, msg) in [
        (result1, Star::One, "submit-1", "star 1"),
        (result2, Star::Two, "submit-2", "star 2"),
    ] {
        if let Some(result) = result {
            if env::args().any(|e| e == env) {
                match submit_result(Some(path), year, day, star, &result)? {
                    SubmitResultResponse::Right => return Ok(()),
                    v => {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Could not submit result for {msg}: {v:?}"),
                        ))
                    }
                }
            }
        }
    }

    Ok(())
}

/// Submit a result using a [`PuzzleIO`] constructed from a session cookie loaded from the given `path` or
/// from `session.cookie` if `path` is `None`.
pub fn submit_result<S: std::fmt::Display + ?Sized>(
    path: Option<&str>,
    year: u16,
    day: u16,
    star: Star,
    result: &S,
) -> Result<SubmitResultResponse, Error> {
    let session = fs::read_to_string(path.unwrap_or("sesssion.cookie"))?;
    PuzzleIO::from(session.trim()).submit_result(year, day, star, result)
}

/// Enum to submit solution for part one or part two (see [`PuzzleIO::submit_result`])
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Star {
    One,
    Two,
}

impl Star {
    const ONE: &str = "1";
    const TWO: &str = "2";

    fn as_level(&self) -> &'static str {
        match self {
            Star::One => Self::ONE,
            Star::Two => Self::TWO,
        }
    }
}

/// Possible responses when submitting result (see [`PuzzleIO::submit_result``])
#[derive(Debug, PartialEq, Eq)]
pub enum SubmitResultResponse {
    /// Response indicating that your result is correct, you earned a star!
    Right,
    /// Response indicating that you already solved this star
    AlreadySolved,
    /// Response indicating that your result is wrong
    Wrong,
    /// Response indicating that you have to wait for some time before you are allowed to
    /// re-submitting a result. The value of this variant, if present, indicates how many
    /// seconds you have to wait
    Wait(Option<usize>),
}

/// Puzzle IO
///
/// Create instances using [`PuzzleResult::from`]
#[derive(Debug)]
pub struct PuzzleIO<'a> {
    pub session: &'a str,
}

impl<'a> From<&'a str> for PuzzleIO<'a> {
    /// Create a [`PuzzleIO`] struct from a `&str` representing a session ID.
    fn from(session: &'a str) -> Self {
        Self { session }
    }
}

impl<'a> PuzzleIO<'a> {
    fn request_builder(&self, post: bool, year: u16, day: u16, path: &str) -> RequestBuilder {
        let url = format!("https://adventofcode.com/{year}/day/{day}/{path}");
        let client = reqwest::blocking::Client::new();
        if post {
            client.post(url.as_str())
        } else {
            client.get(url.as_str())
        }
        .header("Cookie", format!("session={}", self.session))
        .header(
            "User-Agent",
            format!(
                "{}/{} ({REPO} by {AUTHOR})",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ),
        )
    }

    /// Load input for given `year` and `day` into a `String`
    pub fn load_input(&self, year: u16, day: u16) -> Result<String, Error> {
        self.request_builder(false, year, day, "input")
            .send()
            .map_err(|err| Error::new(ErrorKind::Other, err))?
            .text()
            .map_err(|err| Error::new(ErrorKind::Other, err))
    }

    /// Submit the `result` for a given `year`, `day`, and `star`.
    ///
    /// The `result` is formatted into a `String` before submitting (using the `std::fmt::Display` trait)
    ///
    /// The function will print the response on `stdout` and return the [`SubmitResultResponse`] wrapped in a `Result`
    pub fn submit_result<S: std::fmt::Display + ?Sized>(
        &self,
        year: u16,
        day: u16,
        star: Star,
        result: &S,
    ) -> Result<SubmitResultResponse, Error> {
        let response = self
            .request_builder(true, year, day, "answer")
            .form(&[
                ("level", star.as_level()),
                ("answer", format!("{result}").as_str()),
            ])
            .send()
            .map_err(|err| Error::new(ErrorKind::Other, err))?;

        if response.status() != StatusCode::OK {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Response with status code {}", response.status()),
            ));
        }

        let text = response
            .text()
            .map_err(|err| Error::new(ErrorKind::Other, err))?;

        let main = text
            .find("<main>")
            .and_then(|start| text.find("</main>").map(|end| (start, end)));
        if let Some((start, end)) = main {
            let (end, suffix) = if end > 400 + start + 6 {
                (400 + start + 3, "...")
            } else {
                (end, "")
            };
            println!("{}{suffix}", &text[start + 6..end]);
        } else {
            if text.len() > 400 {
                println!("{}...", &text[0..397]);
            } else {
                println!("{text}");
            }
        }

        if text.contains("That's the right answer") {
            Ok(SubmitResultResponse::Right)
        } else if text.contains("Did you already complete it") {
            Ok(SubmitResultResponse::AlreadySolved)
        } else if text.contains("That's not the right answer") {
            Ok(SubmitResultResponse::Wrong)
        } else if text.contains("You gave an answer too recently") {
            let re = Regex::new(r"You have (?:(?P<m>\d+)m )?(?P<s>\d+)s left to wait").unwrap();
            let s = re.captures(text.as_str()).map(|c| {
                c.name("m")
                    .map(|m| m.as_str().parse::<usize>().unwrap())
                    .unwrap_or(0)
                    * 60
                    + c.name("s")
                        .map(|s| s.as_str().parse::<usize>().unwrap())
                        .unwrap()
            });
            Ok(SubmitResultResponse::Wait(s))
        } else {
            Err(Error::new(ErrorKind::Other, "Can't interpret answer."))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_submit() {
        let puzzle_io = PuzzleIO::from("53616c7465645f5f26a828ab5a2977e2f4893e0ab7aeab2f520b1c62f6db37c4b6425bb1626c7b38342a19cc02acbd686204588c82e03b0bcb202faf54e96241");

        let result = puzzle_io.submit_result(2022, 2, Star::One, "11841");
        assert!(
            matches!(result, Ok(_)),
            "Expected an ok value, found: {result:?}"
        );
    }
}

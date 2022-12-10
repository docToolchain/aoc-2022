use crate::err::PuzzleError;
use regex::Regex;
use reqwest::{blocking::RequestBuilder, StatusCode};
use std::{env, fs, path::Path};

/// Puzzle IO
///
/// Create instances using [`PuzzleResult::from`]
pub struct PuzzleIO {
    pub session: String,
}

impl std::fmt::Debug for PuzzleIO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // do not print session
        f.debug_struct("PuzzleIO")
            .field("session", &format!("{}...", &self.session[0..5]))
            .finish()
    }
}

impl From<&str> for PuzzleIO {
    /// Create a [`PuzzleIO`] struct from a `&str` representing a session ID.
    fn from(session: &str) -> Self {
        Self {
            session: session.to_string(),
        }
    }
}

impl TryFrom<&Path> for PuzzleIO {
    type Error = std::io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Ok(Self {
            session: fs::read_to_string(path)?.trim().to_string(),
        })
    }
}

impl PuzzleIO {
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
                "{}/{} ({} by {})",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_REPOSITORY"),
                env!("CARGO_PKG_AUTHORS")
            ),
        )
    }

    /// Load input for given `year` and `day` into a `String`
    pub fn load_input(&self, year: u16, day: u16) -> Result<String, PuzzleError> {
        self.request_builder(false, year, day, "input")
            .send()
            .map_err(|err| PuzzleError::from(err.to_string()))?
            .text()
            .map_err(|err| PuzzleError::from(err.to_string()))
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
        level: u8,
        result: &S,
    ) -> Result<SubmitResponse, PuzzleError> {
        let response = self
            .request_builder(true, year, day, "answer")
            .form(&[("level", level.to_string()), ("answer", result.to_string())])
            .send()
            .map_err(|err| PuzzleError::from(err.to_string()))?;

        if response.status() != StatusCode::OK {
            return Err(PuzzleError::from(format!(
                "Response with status code {}",
                response.status()
            )));
        }

        // get response body as text
        let text = response
            .text()
            .map_err(|err| PuzzleError::from(err.to_string()))?;

        // extract relevant part of response
        let text = match text
            .find("<main>")
            .and_then(|start| text.find("</main>").map(|end| (start + 6, end)))
        {
            Some((start, end)) => &text[start..end],
            None => &text,
        };

        // print response (truncated to 400 characters)
        if text.len() > 400 {
            println!("{}...", &text[0..397]);
        } else {
            println!("{text}");
        }

        // determine answer
        if text.contains("That's the right answer") {
            Ok(SubmitResponse::Right)
        } else if text.contains("Did you already complete it") {
            Ok(SubmitResponse::AlreadySolved)
        } else if text.contains("That's not the right answer") {
            Ok(SubmitResponse::Wrong)
        } else if text.contains("You gave an answer too recently") {
            let re = Regex::new(r"You have (?:(?P<m>\d+)m )?(?P<s>\d+)s left to wait").unwrap();
            let s = re.captures(text).map(|c| {
                c.name("m")
                    .map(|m| m.as_str().parse::<usize>().unwrap())
                    .unwrap_or(0)
                    * 60
                    + c.name("s")
                        .map(|s| s.as_str().parse::<usize>().unwrap())
                        .unwrap()
            });
            Ok(SubmitResponse::Wait(s))
        } else {
            Err(PuzzleError::from("Can't interpret answer."))
        }
    }
}

/// Possible responses when submitting result (see [`PuzzleIO::submit_result``])
#[derive(Debug, PartialEq, Eq)]
pub enum SubmitResponse {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_submit() {
        let puzzle_io = PuzzleIO::from("53616c7465645f5f26a828ab5a2977e2f4893e0ab7aeab2f520b1c62f6db37c4b6425bb1626c7b38342a19cc02acbd686204588c82e03b0bcb202faf54e96241");

        let result = puzzle_io.submit_result(2022, 2, 1, "11841");
        assert!(
            matches!(result, Ok(_)),
            "Expected an ok value, found: {result:?}"
        );
    }
}

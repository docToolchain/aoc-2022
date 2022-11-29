use err::PuzzleError;
use std::{str::FromStr, time::Instant};

/// Trait representing the result of a puzzle
///
/// This allows the solver functions to either return a plain value
/// or a [`Result`] with any error type that is convertible to [`PuzzleError`]
///
/// # Examples
/// ```
/// # use mr_kaffee_aoc::{PuzzleResult,err::{PuzzleError,PuzzleErrorKind}};
/// let value: Result<usize, &str> = Err("I am an error");
/// let result: Result<usize, _> = value.result();
/// assert!(matches!(result.err().unwrap().kind(), PuzzleErrorKind::Other));
///
/// let value = 7;
/// let result = value.result();
/// assert!(matches!(result, Ok(7)));
/// ```
pub trait PuzzleResult<T: PartialEq + std::fmt::Display> {
    /// get the puzzle result as a [`Result`]
    fn result(self) -> Result<T, PuzzleError>;
}

/// Implementation of the [`PuzzleResult`] type for any [`Result`] with error type convertible
/// to [`PuzzleError`]
impl<T, E> PuzzleResult<T> for Result<T, E>
where
    PuzzleError: From<E>,
    T: PartialEq + std::fmt::Display,
{
    fn result(self) -> Result<T, PuzzleError> {
        self.map_err(|e| e.into())
    }
}

/// Implementation of the [`PuzzleResult`] type for any result type. The [`PuzzleResult::result`]
/// function will always return a [`Result::Ok`] value in this case.
impl<T> PuzzleResult<T> for T
where
    T: PartialEq + std::fmt::Display,
{
    fn result(self) -> Result<T, PuzzleError> {
        Ok(self)
    }
}

/// The solution for an [Advent of Code](https:://adventofcode.com) puzzle of a specific day/year
pub struct Puzzle<S, T1, R1, T2, R2>
where
    S: 'static + FromStr,
    T1: 'static + PartialEq + std::fmt::Display,
    R1: 'static + PuzzleResult<T1>,
    T2: 'static + PartialEq + std::fmt::Display,
    R2: 'static + PuzzleResult<T2>,
    PuzzleError: From<<S as FromStr>::Err>,
{
    /// year of the puzzle
    pub year: u16,
    /// day of the puzzle
    pub day: u16,
    /// the puzzle input
    pub input: &'static str,
    /// the first star of the puzzle, if any
    pub star1: Option<Star<S, T1, R1>>,
    /// the second star of the puzzle, if any
    pub star2: Option<Star<S, T2, R2>>,
}

impl<S, T1, R1, T2, R2> Puzzle<S, T1, R1, T2, R2>
where
    S: 'static + FromStr,
    T1: 'static + PartialEq + std::fmt::Display,
    R1: 'static + PuzzleResult<T1>,
    T2: 'static + PartialEq + std::fmt::Display,
    R2: 'static + PuzzleResult<T2>,
    PuzzleError: From<<S as FromStr>::Err>,
{
    /// Solve a puzzle
    ///
    /// This will parse the input to the `IN` type and pass it on to the contained [`Star`]s
    ///
    /// # Examples
    /// ```
    /// # use mr_kaffee_aoc::{Puzzle,Star,err::PuzzleError,PuzzleResult};
    /// # use std::error::Error;
    /// // a simple puzzle instance with a star that simply outputs the input
    /// let puzzle: Puzzle<usize, usize, usize, usize, usize> = Puzzle {
    ///     year: 2022,
    ///     day: 24,
    ///     input: "10",
    ///     star1: Some(Star {
    ///         name: "my star",
    ///         f: &(|v| *v),
    ///         exp: Some(10),
    ///     }),
    ///     star2: None,
    /// };
    /// assert!(puzzle.solve().is_ok());
    ///
    /// // this will result in an error because the expected output does not match the input
    /// let puzzle = Puzzle { input: "20", ..puzzle };
    /// assert!(puzzle.solve().is_err());
    /// ```
    pub fn solve(&self) -> Result<(Option<T1>, Option<T2>), PuzzleError> {
        let t = Instant::now();

        let data = self.input.parse::<S>()?;

        let sol1 = self
            .star1
            .as_ref()
            .map(|p| p.solve_timed(&data))
            .transpose()?;
        let sol2 = self
            .star2
            .as_ref()
            .map(|p| p.solve_timed(&data))
            .transpose()?;

        println!(
            "-> Solved puzzle {:04}/{:02} in {:?}",
            self.year,
            self.day,
            t.elapsed()
        );

        Ok((sol1, sol2))
    }
}

/// Generic view on puzzles not depending on any internal types
pub trait GenericPuzzle {
    /// solve a puzzle and return `true` if successful
    fn solve_handle_err(&self) -> bool;

    /// solve a puzzle and forward errors to caller
    fn solve_report_err(&self) -> Result<(), PuzzleError>;

    /// get the year of the puzzle
    fn year(&self) -> u16;

    /// get the day of the puzzle
    fn day(&self) -> u16;
}

/// if [`GenericPuzzle`] is implemented for a type `T` also implement it for `&T`
impl<T> GenericPuzzle for &T
where
    T: GenericPuzzle,
{
    fn solve_handle_err(&self) -> bool {
        T::solve_handle_err(&self)
    }

    fn solve_report_err(&self) -> Result<(), PuzzleError> {
        T::solve_report_err(&self)
    }

    fn year(&self) -> u16 {
        T::year(&self)
    }

    fn day(&self) -> u16 {
        T::day(&self)
    }
}

impl<S, T1, R1, T2, R2> GenericPuzzle for Puzzle<S, T1, R1, T2, R2>
where
    S: 'static + FromStr,
    T1: 'static + PartialEq + std::fmt::Display,
    R1: 'static + PuzzleResult<T1>,
    T2: 'static + PartialEq + std::fmt::Display,
    R2: 'static + PuzzleResult<T2>,
    PuzzleError: From<<S as FromStr>::Err>,
{
    /// Calls the [`Puzzle::solve``] function and returns `true` if it does not return
    /// an [`Result::Err`].
    fn solve_handle_err(&self) -> bool {
        let result = self.solve();
        match result {
            Err(e) => {
                println!("Error solving puzzle {}/{}: {}", self.year, self.day, e);
                false
            }
            _ => true,
        }
    }

    /// Calls the [`Puzzle::solve``] function and maps the [``Result::Ok``] value to the
    /// unit type `()`
    fn solve_report_err(&self) -> Result<(), PuzzleError> {
        self.solve().map(|_| ())
    }

    fn year(&self) -> u16 {
        self.year
    }

    fn day(&self) -> u16 {
        self.day
    }
}

/// type for solver functions used for [`Star::f`]
pub type SolverFun<IN, R> = dyn Fn(&IN) -> R;

/// The solution for one star for a specific [`Puzzle`]
pub struct Star<S, T, R>
where
    S: 'static + FromStr,
    T: 'static + PartialEq + std::fmt::Display,
    R: 'static + PuzzleResult<T>,
{
    /// the name of the star
    pub name: &'static str,
    /// the solver function
    pub f: &'static SolverFun<S, R>,
    /// the expected result, if [`Option::None`], no result verification is performed in the
    /// solve functions [`Star::solve`] and [`Star::solve_timed`].
    pub exp: Option<T>,
}

impl<S, T, R> Star<S, T, R>
where
    S: 'static + FromStr,
    T: 'static + PartialEq + std::fmt::Display,
    R: 'static + PuzzleResult<T>,
{
    /// solve a star & verify result against expected result [`Star::exp`] if not [`Option::None`]
    ///
    /// # Examples
    /// ```
    /// # use mr_kaffee_aoc::{err::PuzzleError,PuzzleResult};
    /// # use mr_kaffee_aoc::Star;
    /// let star = Star { name: "my star", f: &(|v: &usize| *v), exp: Some(10) };
    /// assert_eq!(10, star.solve(&10).unwrap());
    /// assert!(star.solve(&8).is_err());
    /// ```
    pub fn solve(&self, data: &S) -> Result<T, PuzzleError> {
        match ((self.f)(data).result(), self.exp.as_ref()) {
            (Ok(act), Some(exp)) if exp != &act => {
                // expected result specified but does not match
                Err(PuzzleError::bad_result(self.name, exp, &act))
            }
            (act, _) => act,
        }
    }

    /// solve a star & verify result agains expected result [`Star::exp`] if not [`Option::None`].
    /// Measure time it takes to sovle the puzzle and print to standard out.
    ///
    /// # Examples
    /// ```
    /// # use std::convert::Infallible;
    /// # use mr_kaffee_aoc::err::PuzzleError;
    /// # use mr_kaffee_aoc::Star;
    /// let star: Star<_, _, Result<usize, Infallible>> = Star { name: "my star", f: &(|v| Ok(*v)), exp: Some(10) };
    /// assert_eq!(10, star.solve_timed(&10).unwrap());
    /// assert!(star.solve_timed(&8).is_err());
    /// ```
    pub fn solve_timed(&self, data: &S) -> Result<T, PuzzleError> {
        let t = Instant::now();
        let sol = self.solve(data)?;
        let v = if self.exp.is_none() {
            " (unverified)"
        } else {
            ""
        };
        println!("Solved {} in {:?}: {}{}", self.name, t.elapsed(), sol, v);
        Ok(sol)
    }
}

/// AoC error handling
pub mod err {
    use std::{
        convert::Infallible,
        error::Error,
        fmt,
        num::{ParseFloatError, ParseIntError},
    };

    /// puzzle errors
    #[derive(Debug)]
    pub enum PuzzleErrorKind {
        /// error caused by [`ParseIntError`]
        ParseIntError(ParseIntError),
        /// error caused by [`ParseFloatError`]
        ParseFloatError(ParseFloatError),
        /// error caused by other [`Error`]s
        GenericError(Box<dyn Error>),
        /// error caused by a bad result
        BadResult(&'static str, String),
        /// other unspecified error
        Other,
    }

    impl Default for PuzzleErrorKind {
        /// the default kind is [`PuzzleErrorKind::Other`]
        fn default() -> Self {
            Self::Other
        }
    }

    impl PuzzleErrorKind {
        /// return the source error for variants that have one, i.e., [`PuzzleErrorKind::ParseIntError`],
        /// [`PuzzleErrorKind::ParseFloatError`], or [`PuzzleErrorKind::GenericError`], return [`Option::None`]
        ///  for other variants.
        pub fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                Self::ParseIntError(error) => Some(error),
                Self::ParseFloatError(error) => Some(error),
                Self::GenericError(error) => Some(error.as_ref()),
                _ => None,
            }
        }
    }

    /// puzzle error
    ///
    /// # Examples
    /// ```
    /// # use mr_kaffee_aoc::err::{PuzzleError,PuzzleErrorKind};
    /// # use std::error::Error;
    /// // parse errors
    /// let err: PuzzleError = "not a float".parse::<f64>().err().unwrap().into();
    /// assert!(matches!(err.kind(), PuzzleErrorKind::ParseFloatError(_)));
    ///
    /// let err: PuzzleError = "not an int".parse::<u64>().err().unwrap().into();
    /// assert!(matches!(err.kind(), PuzzleErrorKind::ParseIntError(_)));
    ///
    /// // generic errors
    /// let source: Box<dyn Error> = std::io::Error::from_raw_os_error(7).into();
    /// let err: PuzzleError = source.into();
    /// assert!(matches!(err.kind(), PuzzleErrorKind::GenericError(_)));
    ///
    /// // bad result
    /// let err = PuzzleError::bad_result("star 1", 10, 20);
    /// assert!(matches!(err.kind(), PuzzleErrorKind::BadResult(_,_)));
    ///
    /// // generic error
    /// let msg = "a `&str` message";
    /// let err: PuzzleError = msg.into();
    /// assert!(matches!(err.kind(), PuzzleErrorKind::Other));
    /// assert_eq!(format!("{}", err), msg.to_string());
    ///
    /// let msg = "a String message";
    /// let err: PuzzleError = msg.to_string().into();
    /// assert!(matches!(err.kind(), PuzzleErrorKind::Other));
    /// assert_eq!(format!("{}", err), msg.to_string());
    /// ```
    #[derive(Debug, Default)]
    pub struct PuzzleError {
        kind: PuzzleErrorKind,
        message: Option<String>,
    }

    impl std::fmt::Display for PuzzleError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if let Some(source) = self.kind.source() {
                match self.message.as_ref() {
                    Some(message) => write!(f, "{}. Caused by: {}", message, source),
                    _ => std::fmt::Display::fmt(source, f),
                }
            } else if let PuzzleErrorKind::BadResult(star_name, message) = &self.kind {
                write!(f, "Bad result for {}: {}", star_name, message)
            } else if let Some(message) = self.message.as_ref() {
                message.fmt(f)
            } else {
                write!(f, "unknown / unspecified problem")
            }
        }
    }

    impl PuzzleError {
        /// get kind of the error
        pub fn kind(&self) -> &PuzzleErrorKind {
            &self.kind
        }

        /// construct an instance for a bad result
        pub fn bad_result<T>(star_name: &'static str, exp: T, act: T) -> Self
        where
            T: std::fmt::Display,
        {
            Self {
                kind: PuzzleErrorKind::BadResult(
                    star_name,
                    format!("expected {}, got {}", exp, act),
                ),
                message: Some("Unexpected result".into()),
            }
        }
    }

    impl Error for PuzzleError {
        /// get the source of the error using [`PuzzleErrorKind::source`]
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.kind.source()
        }
    }

    impl From<Box<dyn Error>> for PuzzleError {
        fn from(source: Box<dyn Error>) -> Self {
            Self {
                kind: PuzzleErrorKind::GenericError(source),
                message: Some("Generic error".into()),
            }
        }
    }

    impl From<ParseIntError> for PuzzleError {
        fn from(source: ParseIntError) -> Self {
            Self {
                kind: PuzzleErrorKind::ParseIntError(source),
                message: Some("Parse error".into()),
            }
        }
    }

    impl From<ParseFloatError> for PuzzleError {
        fn from(source: ParseFloatError) -> Self {
            Self {
                kind: PuzzleErrorKind::ParseFloatError(source),
                message: Some("Parse error".into()),
            }
        }
    }

    impl From<String> for PuzzleError {
        fn from(message: String) -> Self {
            Self {
                message: Some(message),
                ..Default::default()
            }
        }
    }

    impl From<&str> for PuzzleError {
        fn from(message: &str) -> Self {
            Self {
                message: Some(message.into()),
                ..Default::default()
            }
        }
    }

    impl From<Infallible> for PuzzleError {
        fn from(_: Infallible) -> Self {
            panic!("Unreachable code!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::err::PuzzleError;
    use std::error::Error;

    struct Data {
        values: Vec<usize>,
    }

    impl FromStr for Data {
        type Err = PuzzleError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self {
                values: s
                    .split(',')
                    .map(|s| s.parse::<usize>())
                    .collect::<Result<_, _>>()?,
            })
        }
    }

    #[test]
    fn test_parse_fail() {
        let data = "100,a".parse::<Data>();

        let error = data.err().expect("No error");

        println!("Error: {}", error);

        match error.kind() {
            err::PuzzleErrorKind::ParseIntError(_) => (),
            kind => assert!(false, "Unexpected kind: {:?}", kind),
        }
    }

    const PUZZLE_OK: Puzzle<Data, usize, Result<usize, &str>, usize, Result<usize, &str>> =
        Puzzle {
            year: 2022,
            day: 0,
            input: "100,200",
            star1: Some(Star {
                name: "part 1",
                f: &(|data| data.values.first().map(|v| *v).ok_or("No 1st point")),
                exp: Some(100),
            }),
            star2: Some(Star {
                name: "part 2",
                f: &(|data| data.values.last().map(|v| *v).ok_or("No last point")),
                exp: None,
            }),
        };

    const PUZZLE_FAIL: Puzzle<Data, usize, Result<usize, &str>, usize, Result<usize, &str>> =
        Puzzle {
            input: "200,100",
            ..PUZZLE_OK
        };

    #[test]
    fn test_puzzle_ok() {
        PUZZLE_OK.solve().unwrap();
    }

    #[test]
    fn test_puzzle_fail() {
        let result = PUZZLE_FAIL.solve();
        assert!(result.is_err(), "Expected error, {:?}", result);
    }

    #[test]
    fn test_star_fail_bad_result() {
        let star = PUZZLE_OK.star1.expect("No star1");
        let exp = star.exp.expect("No expected value");

        let result = star.solve(&Data {
            values: vec![exp + 1],
        });
        assert!(
            result.is_err(),
            "Result is expected to be an error. Found {:?}",
            result
        );
    }

    #[test]
    fn test_star_fail_no_result() {
        let star = PUZZLE_OK.star2.expect("No star2");
        assert!(
            star.exp.is_none(),
            "No expected value expected, found {:?}",
            star.exp
        );

        let result = star.solve(&Data { values: vec![] });
        assert!(
            result.is_err(),
            "Result is expected to be an error (no points). Found {:?}",
            result
        )
    }

    #[test]

    fn test_star_success_good_result() -> Result<(), Box<dyn Error>> {
        let star = PUZZLE_OK.star1.expect("No star1");
        let exp = star.exp.expect("No expected value");

        let result = star.solve(&Data { values: vec![exp] })?;
        assert_eq!(result, exp, "Unexpected result!");

        Ok(())
    }

    #[test]
    fn test_star_success_any_result() -> Result<(), Box<dyn Error>> {
        let star = PUZZLE_OK.star2.expect("No star2");
        assert!(
            star.exp.is_none(),
            "No expected value expected, found {:?}",
            star.exp
        );

        let result = star.solve(&Data { values: vec![0] })?;
        assert_eq!(result, 0, "Unexpected result!");

        Ok(())
    }
}

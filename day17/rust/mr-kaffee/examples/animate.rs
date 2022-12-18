use mr_kaffee_2022_17::{Chamber, RockInChamber};
use std::{thread, time::Duration};

const CONTENT: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

fn main() {
    let mut chamber = Chamber::from(&CONTENT.into());
    for _ in 0..2022 {
        chamber.integrate_rock(|chamber, rock, x, y, w| {
            print!("\x1B[1;1H\x1B[J"); // clear console
            println!(
                "{}",
                RockInChamber::RIC {
                    chamber,
                    rock,
                    x,
                    y,
                    w,
                }
            );
            thread::sleep(Duration::from_millis(20));
        });
    }
}

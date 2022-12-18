use mr_kaffee_2022_17::{Chamber, RockInChamber};
use std::{env, thread, time::Duration};

const CONTENT: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

// tag::main[]
fn main() {
    // accept a single numeric argumnet
    let arg = env::args().skip(1).next();
    let wait = arg
        .map(|arg| arg.parse().expect("a positive amount of ms to pause"))
        .unwrap_or(40);

    let mut chamber = Chamber::from(&CONTENT.into());
    for k in 0..2022 {
        chamber.integrate_rock(|chamber, rock, x, y, w| {
            print!("\x1B[1;1H\x1B[J"); // clear console
            println!(
                "{}",
                RockInChamber {
                    chamber,
                    rock,
                    x,
                    y,
                    w,
                    ..RockInChamber::default()
                }
            );
            println!("Rock {}", k + 1);
            thread::sleep(Duration::from_millis(wait));
        });
    }
}
// end::main[]

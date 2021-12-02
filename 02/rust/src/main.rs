use std::{env, fs};

use anyhow::{anyhow, Context};

fn main() -> anyhow::Result<()> {
    let input = env::args().nth(1).ok_or_else(|| anyhow!("No input file"))?;
    let measurements = read(&input)?;

    println!("The answer to the first part is {}", dive(&measurements)?);
    println!(
        "The answer to the second part is {}",
        dive_angled(&measurements)?
    );
    Ok(())
}

fn read(path: &str) -> anyhow::Result<String> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("Failed to read from {:?}", path))?;
    Ok(contents)
}

trait SubmarineController {
    fn dive(&mut self, amount: i32);
    fn forward(&mut self, amount: i32);
    fn course(&self) -> i32;

    fn parse(course: &str) -> anyhow::Result<i32>
    where
        Self: Sized + Default,
    {
        let mut ctrl: Self = Default::default();
        peg::parser! {
            grammar parser() for str {
                pub(crate) rule course(c: &mut dyn SubmarineController) -> i32
                    = command(c) ** "\n" "\n"* {
                        c.course()
                    }
                rule command(c: &mut dyn SubmarineController)
                    = down(c) / up(c) / forward(c)
                rule down(c: &mut dyn SubmarineController)
                    = "down " a:amount() { c.dive(a); }
                rule up(c: &mut dyn SubmarineController)
                    = "up " a:amount() { c.dive(-a); }
                rule forward(c: &mut dyn SubmarineController)
                    = "forward " a:amount() { c.forward(a); }
                rule amount() -> i32
                    = n:$(['0'..='9']+) { n.parse().unwrap() }
            }
        }

        Ok(parser::course(course, &mut ctrl)?)
    }
}

#[derive(Default)]
struct DirectDive {
    depth: i32,
    distance: i32,
}

impl SubmarineController for DirectDive {
    fn dive(&mut self, amount: i32) {
        self.depth += amount;
    }

    fn forward(&mut self, amount: i32) {
        self.distance += amount;
    }

    fn course(&self) -> i32 {
        self.depth * self.distance
    }
}

#[derive(Default)]
struct AngledDive {
    depth: i32,
    distance: i32,
    angle: i32,
}

impl SubmarineController for AngledDive {
    fn dive(&mut self, amount: i32) {
        self.angle += amount;
    }

    fn forward(&mut self, amount: i32) {
        self.distance += amount;
        self.depth += self.angle * amount;
    }

    fn course(&self) -> i32 {
        self.depth * self.distance
    }
}

fn dive(course: &str) -> anyhow::Result<i32> {
    DirectDive::parse(course)
}

fn dive_angled(course: &str) -> anyhow::Result<i32> {
    AngledDive::parse(course)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dive() {
        assert_eq!(
            dive(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2
"
            )
            .unwrap(),
            150
        );
    }

    #[test]
    fn test_angled_dive() {
        assert_eq!(
            dive_angled(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2
"
            )
            .unwrap(),
            900
        );
    }
}

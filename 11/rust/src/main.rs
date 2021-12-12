use std::{
    collections::HashSet,
    env,
    fmt::Debug,
    fs::read_to_string,
    ops::{Index, IndexMut},
};

use anyhow::{anyhow, Result};

fn main() -> anyhow::Result<()> {
    let path = env::args().nth(1).ok_or_else(|| anyhow!("No input file"))?;
    let input = read_to_string(&path)?;

    let m = Map::parse(&input)?;
    let flashes: usize = m.iter().take(100).map(|(f, _)| f).sum();
    println!("The answer to the first part is {}", flashes);

    Ok(())
}

#[derive(Clone, Copy, PartialEq)]
struct Dim {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Clone)]
struct Map {
    grid: Vec<u8>,
    dim: Dim,
}

impl Map {
    fn parse(input: &str) -> Result<Map> {
        let rows = input.trim().split('\n').collect::<Vec<_>>();
        let y = rows.len();
        let x = rows
            .get(0)
            .map(|r| r.len())
            .ok_or_else(|| anyhow!("Empty file"))?;

        let grid = rows
            .iter()
            .flat_map(move |&r| {
                r.chars().map(|c| {
                    c.to_digit(10)
                        .map(|d| d as u8)
                        .ok_or_else(|| anyhow!("Invalid digit {:?}", c))
                })
            })
            .collect::<Result<_>>()?;
        Ok(Self {
            grid,
            dim: Dim { x, y },
        })
    }

    fn neighbors(&'_ self, pos: [usize; 2]) -> impl Iterator<Item = [usize; 2]> + '_ {
        (-1..=1)
            .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
            .filter(|&(dx, dy)| dx != 0 || dy != 0)
            .map(move |(dx, dy)| ((pos[0] as i32 + dx) as usize, (pos[1] as i32 + dy) as usize))
            .filter(|&(x, y)| x < self.dim.x && y < self.dim.y)
            .map(move |(x, y)| [x, y])
    }

    fn iter_pos(&self) -> impl Iterator<Item = [usize; 2]> {
        let m = self.dim.x;
        let n = self.dim.y;
        (0..self.grid.len()).map(move |i| {
            let x = i / m;
            let y = i % n;
            [x, y]
        })
    }

    fn step(&self) -> (usize, Self) {
        let mut next = Self {
            grid: self.grid.iter().map(|&c| c + 1).collect::<Vec<_>>(),
            dim: self.dim,
        };

        let mut flashing = HashSet::new();
        let mut flashing_count = 0;

        loop {
            for pos in next.iter_pos() {
                if next[pos] > 9 && !flashing.contains(&pos) {
                    self.neighbors(pos).for_each(|pos| next[pos] += 1);
                    flashing.insert(pos);
                }
            }
            if flashing.len() == flashing_count {
                break;
            }
            flashing_count = flashing.len();
        }
        for pos in next.iter_pos() {
            if next[pos] > 9 {
                flashing.insert(pos);
            }
        }

        next.grid.iter_mut().for_each(|c| {
            if *c > 9 {
                *c = 0
            }
        });

        (flashing.len(), next)
    }

    fn iter(&self) -> impl Iterator<Item = (usize, Self)> {
        std::iter::repeat(()).scan((0, self.clone()), |st, _| {
            *st = st.1.step();
            Some(st.clone())
        })
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.dim.y {
            writeln!(
                f,
                "{}",
                self.grid[self.dim.x * r..self.dim.x * r + self.dim.y]
                    .iter()
                    .map(|&c| char::from_digit(c.into(), 10).unwrap())
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl Index<[usize; 2]> for Map {
    type Output = u8;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.grid[self.dim.x * index[0] + index[1]]
    }
}

impl IndexMut<[usize; 2]> for Map {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.grid[self.dim.x * index[0] + index[1]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn create_map() -> Result<Map> {
        Map::parse(indoc! {"
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        "})
    }

    #[test]
    fn test_parsing() -> Result<()> {
        let m = create_map()?;

        assert_eq!(m[[0, 0]], 5);
        assert_eq!(m[[9, 9]], 6);
        Ok(())
    }

    #[test]
    fn test_neighbours() -> Result<()> {
        let m = create_map()?;
        assert_eq!(
            m.neighbors([0, 0]).collect::<Vec<_>>(),
            vec![[0, 1], [1, 0], [1, 1]]
        );
        assert_eq!(
            m.neighbors([1, 1]).collect::<Vec<_>>(),
            vec![
                [0, 0],
                [0, 1],
                [0, 2],
                [1, 0],
                [1, 2],
                [2, 0],
                [2, 1],
                [2, 2]
            ]
        );
        Ok(())
    }

    #[test]
    fn test_step() -> Result<()> {
        let m = Map::parse(indoc! {"
            11111
            19991
            19191
            19991
            11111
        "})?;

        assert_eq!(
            m.step(),
            (
                9,
                Map::parse(indoc! {"
                    34543
                    40004
                    50005
                    40004
                    34543
                "})?
            )
        );

        assert_eq!(
            m.iter().nth(1).unwrap(),
            (
                0,
                Map::parse(indoc! {"
                    45654
                    51115
                    61116
                    51115
                    45654
                "})?
            )
        );
        Ok(())
    }

    #[test]
    fn test_flashes() -> Result<()> {
        let m = Map::parse(indoc! {"
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        "})?;

        let step_1 = Map::parse(indoc! {"
            6594254334
            3856965822
            6375667284
            7252447257
            7468496589
            5278635756
            3287952832
            7993992245
            5957959665
            6394862637
        "})?;
        assert_eq!(m.step(), (0, step_1));

        let flashes: usize = m
            .iter()
            .take(10)
            .map(|(f, m)| {
                dbg!((f, m));
                f
            })
            .sum();

        assert_eq!(flashes, 204);

        let flashes: usize = m.iter().take(100).map(|(f, _)| f).sum();
        assert_eq!(flashes, 1656);
        Ok(())
    }
}

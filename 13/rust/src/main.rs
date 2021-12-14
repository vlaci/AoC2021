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
    let ins = Instructions::parse(&input)?;
    let map = Map::from_instructions(&ins);

    println!(
        "The answer to the first part is {}",
        map.fold(ins.folds[0]).count()
    );

    let map = ins
        .folds
        .iter()
        .scan(map, |m, &f| {
            *m = m.fold(f);
            Some(m.clone())
        })
        .last();

    println!("The answer to the second part is {:?}", map);

    Ok(())
}

struct Instructions {
    coordinates: HashSet<[usize; 2]>,
    folds: Vec<[usize; 2]>,
}

impl Instructions {
    fn new() -> Self {
        Self {
            coordinates: HashSet::new(),
            folds: Vec::new(),
        }
    }

    fn parse(desc: &str) -> Result<Self> {
        peg::parser! {
            grammar parser() for str {
                pub(crate) rule instructions(c: &mut Instructions)
                    = coordinates(c) ** "\n" "\n\n" folds(c) ** "\n"
                rule coordinates(c: &mut Instructions)
                    = x:num() "," y:num() { c.coordinates.insert([x, y]); }
                rule num() -> usize
                    = n:$(['0'..='9']+) { n.parse().unwrap() }
                rule folds(c: &mut Instructions)
                    = "fold along " a:axis() "=" p:num() { c.folds.push(if a == 'x' { [p, 0] } else { [0, p] }); }
                rule axis() -> char
                    = a:$(['x' | 'y']) { a.chars().next().unwrap() }

            }
        }

        let mut instructions = Self::new();
        parser::instructions(desc.trim(), &mut instructions)?;
        Ok(instructions)
    }

    fn size(&self) -> [usize; 2] {
        let x = *self.coordinates.iter().map(|[x, _]| x).max().unwrap();
        let y = *self.coordinates.iter().map(|[_, y]| y).max().unwrap();
        [x + 1, y + 1]
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Dim {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Clone)]
struct Map {
    grid: Vec<bool>,
    dim: [usize; 2],
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}x{}", self.dim[0], self.dim[1])?;
        for r in 0..self.dim[1] {
            writeln!(
                f,
                "{}",
                self.grid[self.dim[0] * r..self.dim[0] * (r + 1)]
                    .iter()
                    .map(|&c| if c { '#' } else { '.' })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl Map {
    fn from_instructions(ins: &Instructions) -> Self {
        let mut map = Map::from_elem(false, ins.size());
        for p in ins.coordinates.iter() {
            map[*p] = true;
        }
        map
    }
    fn from_elem(v: bool, d: [usize; 2]) -> Self {
        Self {
            grid: vec![v; d[0] * d[1]],
            dim: d,
        }
    }

    fn count(&self) -> usize {
        self.grid.iter().filter(|&v| *v).count()
    }

    fn fold(&self, axis: [usize; 2]) -> Self {
        let mut rv;
        match axis {
            [x, 0] => {
                let dim = [x, self.dim[1]];
                rv = Self {
                    grid: vec![Default::default(); dim[0] * dim[1]],
                    dim,
                };
                for [x, y] in self.iter_pos() {
                    if x == dim[0] {
                        continue;
                    }
                    let xn = if x <= dim[0] {
                        x
                    } else {
                        dim[0] - (x - dim[0])
                    };
                    rv[[xn, y]] |= self[[x, y]];
                }
            }
            [0, y] => {
                let dim = [self.dim[0], y];
                rv = Self {
                    grid: vec![Default::default(); dim[0] * dim[1]],
                    dim,
                };
                for [x, y] in self.iter_pos() {
                    if y == dim[1] {
                        continue;
                    }
                    let yn = if y <= dim[1] {
                        y
                    } else {
                        dim[1] - (y - dim[1])
                    };
                    rv[[x, yn]] |= self[[x, y]];
                }
            }
            _ => unreachable!(),
        }
        rv
    }

    fn iter_pos(&self) -> impl Iterator<Item = [usize; 2]> {
        let n = self.dim[1];
        (0..self.grid.len()).map(move |i| {
            let x = i / n;
            let y = i % n;
            [x, y]
        })
    }
}

impl Index<[usize; 2]> for Map {
    type Output = bool;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.grid[self.dim[0] * index[1] + index[0]]
    }
}

impl IndexMut<[usize; 2]> for Map {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.grid[self.dim[0] * index[1] + index[0]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parsing() -> Result<()> {
        let input = indoc! {"
            6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0

            fold along y=7
            fold along x=5
        "};

        let ins = Instructions::parse(input).unwrap();
        let map = Map::from_instructions(&ins);

        assert_eq!(map.fold(ins.folds[0]).count(), 17);
        Ok(())
    }
}

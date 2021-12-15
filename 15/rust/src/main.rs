use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    env,
    fmt::Debug,
    fs::read_to_string,
    ops::{Index, IndexMut},
};

use anyhow::{anyhow, Result};

fn main() -> anyhow::Result<()> {
    let path = env::args().nth(1).ok_or_else(|| anyhow!("No input file"))?;
    let input = read_to_string(&path)?;
    let map = Map::parse(&input)?;
    let dst = map.find_shortest([0, 0], [map.shape[0] - 1, map.shape[1] - 1]);
    println!("The answer to the first part is {}", dst);
    let map = scale_map(&map);
    let dst = map.find_shortest([0, 0], [map.shape[0] - 1, map.shape[1] - 1]);
    println!("The answer to the second part is {}", dst);

    Ok(())
}

fn scale_map(map: &Map) -> Map {
    let [w, h] = map.shape;
    let mut new = Map::from_elem([w * 5, h * 5], 0);
    for x in 0..w * 5 {
        for y in 0..h * 5 {
            let f = x / w + y / w;
            let dx = x % w;
            let dy = y % w;

            new[[x, y]] = (map[[dx, dy]] + f - 1) % 9 + 1;
        }
    }
    new
}

#[derive(PartialEq, Clone)]
struct Map {
    grid: Vec<usize>,
    shape: [usize; 2],
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}x{}", self.shape[0], self.shape[1])?;
        for r in 0..self.shape[1] {
            writeln!(
                f,
                "{}",
                self.grid[self.shape[0] * r..self.shape[0] * (r + 1)]
                    .iter()
                    .map(|&c| char::from_digit((c as u8).into(), 10).unwrap())
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl Map {
    fn from_elem(shape: [usize; 2], elem: usize) -> Self {
        Self {
            grid: vec![elem; shape[0] * shape[1]],
            shape,
        }
    }
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
                        .map(|d| d as usize)
                        .ok_or_else(|| anyhow!("Invalid digit {:?}", c))
                })
            })
            .collect::<Result<_>>()?;
        Ok(Self {
            grid,
            shape: [x, y],
        })
    }

    fn neighbors(&'_ self, pos: [usize; 2]) -> impl Iterator<Item = ([usize; 2], usize)> + '_ {
        (-1..=1)
            .step_by(2)
            .map(move |dx| [(pos[0] as i32 + dx), pos[1] as i32])
            .chain(
                (-1..=1)
                    .step_by(2)
                    .map(move |dy| [pos[0] as i32, (pos[1] as i32 + dy)]),
            )
            .filter(|&[x, y]| {
                x >= 0 && y >= 0 && (x as usize) < self.shape[0] && (y as usize) < self.shape[1]
            })
            .map(|[x, y]| {
                let p = [x as usize, y as usize];
                (p, self[p])
            })
    }

    fn find_shortest(&self, start: [usize; 2], end: [usize; 2]) -> usize {
        let mut dist = Map::from_elem(self.shape, usize::MAX);
        let mut heap = BinaryHeap::new();
        dist[start] = 0;
        heap.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == end {
                return cost;
            }
            if cost > dist[position] {
                continue;
            }

            for (neigh, risk) in self.neighbors(position) {
                let next = State {
                    cost: cost + risk,
                    position: neigh,
                };

                if next.cost < dist[next.position] {
                    heap.push(next);
                    dist[next.position] = next.cost;
                }
            }
        }

        unreachable!()
    }
}

impl Index<[usize; 2]> for Map {
    type Output = usize;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.grid[self.shape[0] * index[1] + index[0]]
    }
}

impl IndexMut<[usize; 2]> for Map {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.grid[self.shape[0] * index[1] + index[0]]
    }
}

// From https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: [usize; 2],
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_path_finding() -> Result<()> {
        let input = indoc! {"
            1163751742
            1381373672
            2136511328
            3694931569
            7463417111
            1319128137
            1359912421
            3125421639
            1293138521
            2311944581
        "};

        let map = Map::parse(input)?;
        let dst = map.find_shortest([0, 0], [map.shape[0] - 1, map.shape[1] - 1]);
        assert_eq!(dst, 40);

        let map = scale_map(&map);
        let dst = map.find_shortest([0, 0], [map.shape[0] - 1, map.shape[1] - 1]);
        assert_eq!(dst, 315);

        Ok(())
    }
}

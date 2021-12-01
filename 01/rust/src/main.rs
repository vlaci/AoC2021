use std::{env, fs};

use anyhow::{anyhow, Context};

fn main() -> anyhow::Result<()> {
    let input = env::args().nth(1).ok_or_else(|| anyhow!("No input file"))?;
    let measurements = read(&input)?;

    println!("The answer to the first part is {}", depth(&measurements));
    println!("The answer to the second part is {}", depth2(&measurements, 3));
    Ok(())
}

fn read(path: &str) -> anyhow::Result<Vec<usize>> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("Failed to read from {:?}", path))?;
    contents
        .trim()
        .split('\n')
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.into())
}

fn depth(measurements: &[usize]) -> usize {
    measurements.windows(2).filter(|p| p[1] > p[0]).count()
}

fn depth2(measurements: &[usize], window_size: usize) -> usize {
    let fst = measurements.windows(window_size).map(|w| w.iter().sum());
    let snd = measurements[1..]
        .windows(window_size)
        .map(|w| w.iter().sum());

    fst.zip(snd).filter(|p: &(usize, usize)| p.1 > p.0).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth() {
        assert_eq!(
            depth(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
        assert_eq!(
            depth2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263], 1),
            7
        );
        assert_eq!(
            depth2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263], 3),
            5
        );
    }
}

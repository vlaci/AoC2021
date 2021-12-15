use std::{collections::HashMap, env, fs::read_to_string};

use anyhow::{anyhow, Result};
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let path = env::args().nth(1).ok_or_else(|| anyhow!("No input file"))?;
    let input = read_to_string(&path)?;
    let mut polymer = parse_input(&input)?;
    polymer.mutate(10);
    let result = polymer.count_result();
    println!("The answer to the first part is {}", result);

    polymer.mutate(30);
    let result = polymer.count_result();
    println!("The answer to the second part is {:?}", result);

    Ok(())
}

fn parse_input(rules: &str) -> Result<Polymer> {
    peg::parser! {
        grammar parser() for str {
            pub(crate) rule parse() -> Polymer
                = t:template() "\n\n" r:rules() ** "\n" { Polymer::from(&t, HashMap::from_iter(r.into_iter())) }
            rule template() -> String
                = p:$(['A'..='Z']+) { p.into() }
            rule rules() -> ([char; 2], char)
                = p:pattern() " -> " e:element() { (p, e) }
            rule pattern() -> [char; 2]
                = a:element() b:element() { [a, b] }
            rule element() -> char
                = a:$(['A'..='Z']) { a.chars().next().unwrap() }

        }
    }
    Ok(parser::parse(rules.trim())?)
}

struct Polymer {
    template: HashMap<[char; 2], usize>,
    ends: [char; 2],
    insertion_rules: HashMap<[char; 2], char>,
}

impl Polymer {
    fn from(base: &str, insertion_rules: HashMap<[char; 2], char>) -> Self {
        let mut template = HashMap::new();
        let chars: Vec<char> = base.chars().collect();
        for pair in chars.windows(2) {
            *template.entry([pair[0], pair[1]]).or_insert(0) += 1;
        }
        let ends = [*chars.get(0).unwrap(), *chars.last().unwrap()];

        Self {
            template,
            ends,
            insertion_rules,
        }
    }
    fn count_result(&self) -> usize {
        let mut counts = HashMap::new();
        for (k, v) in self
            .template
            .iter()
            .flat_map(|(&[a, b], &v)| [(a, v), (b, v)])
        {
            *counts.entry(k).or_insert(0) += v;
        }
        *counts.get_mut(&self.ends[0]).unwrap() += 1;
        *counts.get_mut(&self.ends[1]).unwrap() += 1;
        let ((_, min), (_, max)) = counts
            .iter()
            .minmax_by_key(|(_, &v)| v)
            .into_option()
            .unwrap();
        max / 2 - min / 2
    }

    fn apply_rules(&mut self) {
        let mut template = HashMap::new();
        for (pair, count) in &self.template {
            if let Some(&new) = self.insertion_rules.get(pair) {
                *template.entry([pair[0], new]).or_insert(0) += count;
                *template.entry([new, pair[1]]).or_insert(0) += count;
            }
        }
        self.template = template;
    }

    fn mutate(&mut self, count: usize) {
        for _ in 0..count {
            self.apply_rules()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_polymer() -> Result<()> {
        let input = indoc! {"
            NNCB

            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C
        "};

        let mut polymer = parse_input(input)?;
        assert_eq!(polymer.count_result(), 1);

        polymer.mutate(10);
        assert_eq!(polymer.count_result(), 1588);
        polymer.mutate(30);
        assert_eq!(polymer.count_result(), 2188189693529);

        Ok(())
    }
}

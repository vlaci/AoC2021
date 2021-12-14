use std::{
    collections::{HashMap, LinkedList},
    env,
    fmt::{Debug, Display},
    fs::read_to_string,
    ops::{Index, IndexMut},
};

use anyhow::{anyhow, Result};
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let path = env::args().nth(1).ok_or_else(|| anyhow!("No input file"))?;
    let input = read_to_string(&path)?;
    let polymer = parse_input(&input)?;
    let polymer = polymer.iter().take(10).last().unwrap();
    let counts = polymer
        .to_string()
        .chars()
        .counts();
    let ((_, min), (_, max)) = counts
        .iter()
        .minmax_by_key(|&(c, f)| f).into_option().unwrap();
    println!("The answer to the first part is {}", max-min);

    //println!("The answer to the second part is {:?}", map);

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
    template: Vec<char>,
    insertion_rules: HashMap<[char; 2], char>,
}

impl Polymer {
    fn from(template: &str, insertion_rules: HashMap<[char; 2], char>) -> Self {
        Self {
            template: template.chars().collect::<Vec<_>>(),
            insertion_rules,
        }
    }

    fn apply_rules(&self) -> Self {
        let template: Vec<_> = std::iter::once(self.template[0])
            .chain(self.template.windows(2).flat_map(|pair| {
                dbg!(pair);
                dbg!([self.insertion_rules[pair], pair[1]])
            }))
            .collect();
        let rv = Self {
            template,
            insertion_rules: self.insertion_rules.clone(),
        };
        dbg!(rv.to_string());
        rv
    }

    fn iter(&self) -> impl Iterator<Item = Polymer> + '_ {
        std::iter::successors(Some(self.apply_rules()), |p| Some(p.apply_rules()))
    }
}

impl Display for Polymer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.template.iter().collect::<String>())
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

        assert_eq!(polymer.to_string(), "NNCB");

        assert_eq!(
            polymer
                .iter()
                .take(4)
                .map(|p| p.to_string())
                .collect::<Vec<_>>(),
            [
                "NCNBCHB",
                "NBCCNBBBCBHCB",
                "NBBBCNCCNBBNBNBBCHBHHBCHB",
                "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
            ]
        );

        Ok(())
    }
}

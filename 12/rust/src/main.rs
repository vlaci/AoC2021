use std::{
    collections::{HashMap, HashSet},
    env,
    fmt::Debug,
    fs::read_to_string,
};

use anyhow::{anyhow, Result};

fn main() -> anyhow::Result<()> {
    let path = env::args().nth(1).ok_or_else(|| anyhow!("No input file"))?;
    let input = read_to_string(&path)?;
    let graph = Graph::parse(&input)?;

    println!("The answer to the first part is {}", graph.count_paths());
    Ok(())
}

struct Graph {
    nodes: HashMap<Node, HashSet<Node>>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Node {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "Start"),
            Self::End => write!(f, "End"),
            Self::Big(c) | Self::Small(c) => write!(f, "{}", c),
        }
    }
}

impl Graph {
    fn add_edge(&mut self, edge: [Node; 2]) {
        let [start, end] = edge;
        let entry = self.nodes.entry(start.clone()).or_insert_with(|| HashSet::new());
        (*entry).insert(end.clone());
        let entry = self.nodes.entry(end).or_insert_with(HashSet::new);
        (*entry).insert(start);
    }

    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn parse(desc: &str) -> Result<Self> {
        peg::parser! {
            grammar parser() for str {
                pub(crate) rule graph(c: &mut Graph)
                    = edge(c) ** "\n"
                rule edge(c: &mut Graph)
                    = s:node() "-" e:node() { c.add_edge([s, e]); }
                rule node() -> Node
                    = start() / end() / big() / small()
                rule start() -> Node
                    = "start" { Node::Start }
                rule end() -> Node
                    = "end" { Node::End }
                rule big() -> Node
                    = c:$(['A'..='Z']+) { Node::Big(c.into()) }
                rule small() -> Node
                    = c:$(['a'..='z']+) { Node::Small(c.into()) }

            }
        }

        let mut graph = Self::new();
        parser::graph(desc.trim(), &mut graph)?;
        Ok(graph)
    }

    fn count_paths(&self) -> usize {
        count_recursive(&Node::Start, &mut HashSet::new(), &self.nodes)
    }
}

fn count_recursive(
    pos: &Node,
    visited: &mut HashSet<Node>,
    edges: &HashMap<Node, HashSet<Node>>,
) -> usize {
    if let Node::Small(_) = pos {
        if !visited.insert(pos.clone()) {
            return 0;
        }
    }
    if *pos == Node::End {
        return 1;
    }

    let mut total = 0;
    for adjacent in edges[pos].iter() {
        if *adjacent != Node::Start && !&visited.contains(adjacent) {
            total += count_recursive(adjacent, &mut visited.clone(), edges);
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_graph() {
        let input = indoc! {"
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        "};
        let g = Graph::parse(input).unwrap();

        assert_eq!(g.count_paths(), 10);
    }
}

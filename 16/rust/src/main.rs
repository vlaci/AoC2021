use std::{env, fs::read_to_string};

use anyhow::{anyhow, Result};

fn main() -> anyhow::Result<()> {
    let path = env::args().nth(1).ok_or_else(|| anyhow!("No input file"))?;
    let input = read_to_string(&path)?;
    let bin = to_binstring(&input);
    let packets = decode(&bin)?;

    println!("The answer to the first part is {}", packets.sum_version());
    let res = packets.eval();
    println!("The answer to the second part is {}", res);

    Ok(())
}

fn to_binstring(input: &str) -> String {
    input
        .trim()
        .chars()
        .flat_map(|hex| match hex {
            '0' => "0000".chars(),
            '1' => "0001".chars(),
            '2' => "0010".chars(),
            '3' => "0011".chars(),
            '4' => "0100".chars(),
            '5' => "0101".chars(),
            '6' => "0110".chars(),
            '7' => "0111".chars(),
            '8' => "1000".chars(),
            '9' => "1001".chars(),
            'A' => "1010".chars(),
            'B' => "1011".chars(),
            'C' => "1100".chars(),
            'D' => "1101".chars(),
            'E' => "1110".chars(),
            'F' => "1111".chars(),
            _ => unreachable!(),
        })
        .collect::<String>()
}

fn to_number(input: &str) -> usize {
    input
        .chars()
        .rev()
        .enumerate()
        .fold(0, |acc, (exp, b)| acc + if b == '1' { 1 << exp } else { 0 })
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: usize,
    payload: Payload,
}

#[derive(Debug, PartialEq)]
enum Payload {
    Literal(usize),
    Subpacket(Operation, Vec<Packet>),
}

#[derive(Debug, PartialEq)]
enum Operation {
    Sum,
    Prod,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

impl Packet {
    fn sum_version(&self) -> usize {
        self.version + self.payload.sum_version()
    }
    fn eval(&self) -> usize {
        self.payload.eval()
    }
}

impl Payload {
    fn sum_version(&self) -> usize {
        match self {
            Payload::Literal(_) => 0,
            Payload::Subpacket(_, ps) => ps.iter().map(|p| p.sum_version()).sum(),
        }
    }
    fn eval_subpackets(sp: &[Packet]) -> impl Iterator<Item = usize> + '_ {
        sp.iter().map(|p| p.eval())
    }
    fn eval(&self) -> usize {
        match self {
            Payload::Literal(l) => *l,
            Payload::Subpacket(op, ps) => {
                let mut sub = Payload::eval_subpackets(ps);
                match op {
                Operation::Sum => sub.sum(),
                Operation::Prod => sub.product(),
                Operation::Min => sub.min().unwrap(),
                Operation::Max => sub.max().unwrap(),
                Operation::Gt => if sub.next().unwrap() > sub.next().unwrap() { 1 } else {0},
                Operation::Lt => if sub.next().unwrap() < sub.next().unwrap() { 1 } else { 0 },
                Operation::Eq => if sub.next().unwrap() == sub.next().unwrap() { 1 } else { 0 },
            }},
        }
     }
}

fn decode(input: &str) -> Result<Packet> {
    peg::parser! {
        grammar parser() for str {
            #[no_eof]
            pub(crate) rule decode() -> Packet
                = packet()
            pub(crate) rule packets() -> Vec<Packet>
                = packet()+
            rule packet() -> Packet
                = v:version() p:payload() { Packet { version: v, payload: p } }
            rule payload() -> Payload
                = literal() / operator()
            rule literal() -> Payload
                = "100" p:literal_value() { Payload::Literal(p) }
            rule operator() -> Payload
                = op_by_count() / op_by_length()
            rule op_by_length() -> Payload
                = id:tid() "0" len:number(15) sub:bits(len) {? Ok(Payload::Subpacket(id, packets(&sub).or(Err("Cannot parse subpackets"))?)) }
            rule op_by_count() -> Payload
                = id:tid() "1" count:number(11) p:packet()*<{count}> { Payload::Subpacket(id, p) }
            rule version() -> usize
                = number(3)
            rule tid() -> Operation
                = id:number(3) {
                    match id {
                        0 => Operation::Sum,
                        1 => Operation::Prod,
                        2 => Operation::Min,
                        3 => Operation::Max,
                     // 4 => Literal
                        5 => Operation::Gt,
                        6 => Operation::Lt,
                        7 => Operation::Eq,
                        _ => unreachable!()
                    }
                }
            rule literal_value() -> usize
                = bl:block()* e:endblock() {
                    let mut res = String::new();
                    for b in bl {
                        res.push_str(&b);
                    }
                    res.push_str(&e);
                    to_number(&res)
                }
            rule block() -> String
                = "1" p:bits(4) { p }
            rule endblock() -> String
                = "0" p:bits(4){ p }

            rule number(len: usize) -> usize
                = b:bits(len) { to_number(&b) }
            rule bits(len: usize) -> String
                = b:$(['0'|'1']*<{len}>) { b.to_string() }
        }
    }
    Ok(parser::decode(input)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_binstring() {
        assert_eq!(to_binstring("D2FE28"), "110100101111111000101000");
    }

    #[test]
    fn test_to_number() {
        assert_eq!(to_number("1010"), 10);
    }

    #[test]
    fn test_parse_literal() {
        assert_eq!(
            decode("110100101111111000101000").unwrap(),
            Packet {
                version: 6,
                payload: Payload::Literal(2021)
            }
        )
    }

    #[test]
    fn test_parse_operator_by_len() {
        assert_eq!(
            decode("00111000000000000110111101000101001010010001001000000000").unwrap(),
            Packet {
                version: 1,
                payload: Payload::Subpacket(Operation::Lt, vec![
                    Packet {
                        version: 6,
                        payload: Payload::Literal(10)
                    },
                    Packet {
                        version: 2,
                        payload: Payload::Literal(20)
                    }
                ])
            }
        )
    }

    #[test]
    fn test_parse_operator_by_count() {
        assert_eq!(
            decode("11101110000000001101010000001100100000100011000001100000").unwrap(),
            Packet {
                version: 7,
                payload: Payload::Subpacket(Operation::Max, vec![
                    Packet {
                        version: 2,
                        payload: Payload::Literal(1)
                    },
                    Packet {
                        version: 4,
                        payload: Payload::Literal(2)
                    },
                    Packet {
                        version: 1,
                        payload: Payload::Literal(3)
                    }
                ])
            }
        )
    }
}

use std::{env, fs::read_to_string};

use anyhow::{anyhow, Result};

fn main() -> anyhow::Result<()> {
    let path = env::args().nth(1).ok_or_else(|| anyhow!("No input file"))?;
    let input = read_to_string(&path)?;
    let bin = to_binstring(&input);
    let packets = decode(&bin)?;

    let vsum = sum_versions(&packets);
    println!("The answer to the first part is {}", vsum);

    Ok(())
}

fn sum_versions(packets: &[Packet]) -> usize {
    packets
        .iter()
        .map(|p| {
            let mut v = p.version;
            if let Payload::Subpacket(ps) = &p.payload {
                v += sum_versions(ps);
            }
            v
        })
        .sum()
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
    Subpacket(Vec<Packet>),
}

fn decode(input: &str) -> Result<Vec<Packet>> {
    peg::parser! {
        grammar parser() for str {
            #[no_eof]
            pub(crate) rule decode() -> Vec<Packet>
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
                = tid() "0" len:number(15) sub:bits(len) { Payload::Subpacket(decode(&sub).unwrap()) }
            rule op_by_count() -> Payload
                = tid() "1" count:number(11) p:packet()*<{count}> { Payload::Subpacket(p) }
            rule version() -> usize
                = number(3)
            rule tid() -> usize
                = number(3)
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
            vec![Packet {
                version: 6,
                payload: Payload::Literal(2021)
            }]
        )
    }

    #[test]
    fn test_parse_operator_by_len() {
        assert_eq!(
            decode("00111000000000000110111101000101001010010001001000000000").unwrap(),
            vec![Packet {
                version: 1,
                payload: Payload::Subpacket(vec![
                    Packet {
                        version: 6,
                        payload: Payload::Literal(10)
                    },
                    Packet {
                        version: 2,
                        payload: Payload::Literal(20)
                    }
                ])
            }]
        )
    }

    #[test]
    fn test_parse_operator_by_count() {
        assert_eq!(
            decode("11101110000000001101010000001100100000100011000001100000").unwrap(),
            vec![Packet {
                version: 7,
                payload: Payload::Subpacket(vec![
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
            }]
        )
    }
}

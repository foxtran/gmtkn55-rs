#![allow(dead_code)]

use pest::Parser;
use pest_derive::Parser;
use std::error::Error;
use std::io::ErrorKind;

#[derive(Default)]
pub struct Reaction {
    pub compounds: Vec<(String, f64)>,
}

#[derive(Parser)]
#[grammar = "reaction_grammar.pest"]
struct ReactionParser;

impl Reaction {
    pub fn new(reaction: &String) -> Result<Self, Box<dyn Error>> {
        if reaction.chars().filter(|c| *c == '=').count() > 1 {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                format!("Reaction '{reaction}' contains too many '=' symbols!"),
            )));
        }

        let parsed_reaction = ReactionParser::parse(Rule::reaction, reaction.as_str())?
            .next()
            .unwrap();

        use pest::iterators::Pair;
        fn parse(pair: Pair<Rule>) -> Reaction {
            let mut reaction = Reaction { compounds: vec![] };
            match pair.as_rule() {
                Rule::reaction => {
                    let mut is_left: bool = true;
                    let mut op: char = '+';
                    for in_pair in pair.into_inner() {
                        match in_pair.as_rule() {
                            Rule::factor_compound => {
                                let mut factor: f64 = 1.0;
                                let mut compound: String = String::from("");
                                for in_in_pair in in_pair.into_inner() {
                                    match in_in_pair.as_rule() {
                                        Rule::factor => {
                                            factor = in_in_pair.as_str().parse().unwrap();
                                        }
                                        Rule::compound => {
                                            compound = in_in_pair.to_string();
                                        }
                                        Rule::EOI
                                        | Rule::reaction
                                        | Rule::operation
                                        | Rule::factor_compound
                                        | Rule::add
                                        | Rule::subtract
                                        | Rule::equal => unreachable!(),
                                    }
                                }
                                if is_left && op == '+' {
                                    factor = factor;
                                } else if !is_left && op == '-' {
                                    factor = factor;
                                } else if is_left && op == '-' {
                                    factor = -factor;
                                } else if !is_left && op == '+' {
                                    factor = -factor;
                                }
                                reaction.compounds.push((compound, factor));
                            }
                            Rule::equal => {
                                is_left = false;
                            }
                            Rule::add => {
                                op = '+';
                            }
                            Rule::subtract => {
                                op = '-';
                            }
                            Rule::EOI => (),
                            Rule::reaction | Rule::operation | Rule::factor | Rule::compound => {
                                unreachable!()
                            }
                        }
                    }
                }
                Rule::EOI
                | Rule::operation
                | Rule::factor_compound
                | Rule::factor
                | Rule::add
                | Rule::subtract
                | Rule::equal
                | Rule::compound => unreachable!(),
            }
            return reaction;
        }

        Ok(parse(parsed_reaction))
    }
}

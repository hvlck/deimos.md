// std

use std::{convert::TryInto, unimplemented};

// crates
use pest::{error::Error as PestError, Parser};
use pest_derive::*;

// local
mod parser;
use parser::*;

#[derive(Parser)]
#[grammar = "lib.pest"]
pub struct ProteusMd;

/// Parses a given source (`src`) string into HTML.
fn parse(src: &str) -> Result<String, PestError<Rule>> {
    let mut ast: Vec<AstNode> = Vec::new();

    let exp = ProteusMd::parse(Rule::any_type, src)?;
    // todo: fix doc pest grammar implementation
    for pair in exp {
        println!("In parse(): {:#?}", pair.as_rule());
        match pair.as_rule() {
            Rule::any_type | Rule::doc => {
                ast.push(parse_to_ast(pair));
            }
            _ => ast.push(AstNode::Error(Error::Invalid)),
        }
    }

    let mut final_output = String::new();
    for i in ast {
        println!("{:#?}", i);
        match i.to_output() {
            Ok(v) => {
                final_output.push_str(v.as_str());
            }
            Err(e) => {}
        }
    }

    Ok(final_output)
}

/// Parses tokens into an AST
fn parse_to_ast(pair: pest::iterators::Pair<Rule>) -> AstNode {
    println!("In parse_to_ast: {:#?}", pair.as_rule());
    match pair.as_rule() {
        Rule::any_type | Rule::doc => parse_to_ast(pair.into_inner().next().unwrap()),
        Rule::header => {
            let mut pair = pair.into_inner();
            let (level, text): (u8, &str) = match pair.next() {
                Some(v) => {
                    let matches: Vec<&str> = v.as_str().matches('#').collect();
                    let level: u8 = match matches.len() {
                        1..=6 => matches.len().try_into().unwrap(),
                        _ => return AstNode::Error(Error::Invalid),
                    };

                    let text = v.as_str().split_at((level + 1).into()).1;
                    (level, text)
                }
                None => return AstNode::Error(Error::Invalid),
            };
            AstNode::Heading { level, text }
        }
        Rule::paragraph => {
            let t = pair.into_inner();
            println!("T: {:#?}", t);
            for pair in t {
                parse_rich_text(pair);
            }

            AstNode::Error(Error::Invalid)
        }
        _ => unimplemented!(),
    }
}

fn parse_rich_text(pair: pest::iterators::Pair<Rule>) -> Result<AstNode, Error> {
    println!("In parse_rich_text: {:#?}", pair.as_rule());
    Ok(AstNode::Code(CodeBlock {
        block_type: CodeType::Fenced,
        contents: String::from("Test"),
    }))
}

#[cfg(test)]
mod output_tests {
    use super::*;
    #[test]
    fn check_complete() {
        let p = parse("## This is a markdown document\nThis is some **rich** *text*, with ~~strikethrough~~, ^superscript^, __subscript__, and more!\n");
    }

    #[test]
    fn check_table_output() {
        let p = parse("| Test | Second Heading |\n| Value | Other Value |");
        assert_eq!(p.unwrap(), "<table><tr><th>Test</th><th>Second Heading</th></tr><tr><td>Value</td><td>Other Value</td></tr>")
    }
}

/// Tests for parsing grammar
#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn verify_paragraphs() {
        let p = ProteusMd::parse(
            Rule::paragraph,
            "**This is bold**. *This is italics*. ^This is superscript^. ___This is subscript___",
        );
        //        assert!(p.is_ok());
    }

    #[test]
    fn verify_headings() {
        let h = ProteusMd::parse(Rule::header, "\n### Test\n");
        assert!(h.is_ok());
        //        assert_eq!();
    }
}

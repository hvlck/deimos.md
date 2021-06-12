use std::collections::HashMap;
// std
use std::fmt;
use std::unimplemented;

// crates

// std

#[derive(Debug, Clone, PartialEq)]
pub struct Tok<'a> {
    /// the type of token
    pub token_type: Token,
    /// token placement
    pub span: (usize, usize),
    /// token source
    pub source: &'a str,
}

type RichText = Vec<Token>;

/// Represents a node.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Heading {
        level: u8,
        text: String,
    },
    Paragraph(RichText),
    /// Italics, denoted using opening and closing asterisks (`*`)
    Italics(String),
    /// Bold text, denoted using opening and closing double asterisks (`**`)
    Bold(String),
    /// Subscript, denoted using opening and closing triple underscores (`___`)
    Subscript(String),
    /// Superscript, denoted using opening and closing carets (`^`)
    Superscript(String),
    /// Strikethrough text, denoted using opening and closing double tildes (`~~`)
    StrikeThrough(String),
    /// Normal text.
    Text(String),
    Table((Vec<String>, Vec<RichText>)),
    /// First is language, second is contents
    FencedCode((String, String)),
    /// An inline code block, denoted using opening and closing backticks
    InlineCode(String),
    Error(LexError),
    OrderedList(RichText),
    UnorderedList(RichText),
    Aside(RichText),
    Footnote(),
    Fragment,
    Image(String, String),
    Math,
    Metadata(HashMap<String, String>),
    Data,
    Details((RichText, RichText)),
    HorizontalBreak,
    NegatedVariable(String),
    Variable(String),
    Whitespace,
    Comment,
    EOI,
}

impl Output for Token {
    fn to_output(&self) -> Result<String, LexError> {
        match self {
            Token::Text(text) => Ok(text.to_owned()),
            Token::Bold(text) => Ok(format!("<strong>{}</strong>", text)),
            Token::Italics(text) => Ok(format!("<em>{}</em>", text)),
            Token::StrikeThrough(text) => Ok(format!("<del>{}</del>", text)),
            Token::Subscript(text) => Ok(format!("<sub>{}</sub>", text)),
            Token::Superscript(text) => Ok(format!("<sup>{}</sup>", text)),
            Token::InlineCode(text) => Ok(format!("<code>{}</code>", text)),
            Token::Heading { level, text } => Ok(format!(
                "<h{level}>{text}<h{level}/>",
                level = &level.to_string(),
                text = text
            )),
            Token::Paragraph(text) => {
                let mut output = String::new();
                for i in text {
                    if let Ok(s) = i.to_output() {
                        output.push_str(&s);
                    }
                }

                Ok(output)
            }
            Token::OrderedList(list) => {
                let mut list_items = String::new();
                for i in list {
                    if let Ok(s) = i.to_output() {
                        list_items.push_str(&format!("<li>{}</li>", s));
                    }
                }

                Ok(format!("<ol>{}</ol>", list_items))
            }
            Token::UnorderedList(list) => {
                let mut list_items = String::new();
                for i in list {
                    if let Ok(s) = i.to_output() {
                        list_items.push_str(&format!("<li>{}</li>", s));
                    }
                }

                Ok(format!("<ul>{}</ul>", list_items))
            }
            Token::Table(table) => {
                todo!()
                // let mut headers = String::new();

                // for i in &table.0 {
                //     headers.push_str(format!("<th>{}</th>", i).as_str());
                // }

                // let mut rows = String::new();

                // for (i, idx) in table.1.into_iter().enumerate() {
                //     let mut row = String::new();
                //     for text in table.1.get(idx) {
                //         for rich_text in text.iter() {
                //             if let Ok(s) = rich_text.to_output() {
                //                 row.push_str(&format!("<td>{}</td>", s));
                //             }
                //         }
                //     }
                //     rows.push_str(format!("<tr>{}</tr>", row).as_str())
                // }

                // Ok(format!(
                //     "<table><tr>{headers}</tr>{rows}</table>",
                //     headers = headers,
                //     rows = rows
                // ))
            }
            Token::Error(error) => Ok(error.to_string()),
            Token::FencedCode(_) => todo!(),
            Token::Aside(text) => {
                let output = text
                    .iter()
                    .map(|i| i.to_output().unwrap())
                    .collect::<String>();
                Ok(format!("<aside>{}</aside>", output))
            }
            Token::Footnote() => todo!(),
            Token::Fragment => todo!(),
            Token::Image(title, link) => Ok(format!("<img src=\"{}\" alt=\"{}\" />", link, title)),
            Token::Math => todo!(),
            Token::Metadata(_) => todo!(),
            Token::Data => todo!(),
            Token::Details((summary, content)) => {
                let sum_output = summary
                    .iter()
                    .map(|i| i.to_output().unwrap())
                    .collect::<String>();

                let content_output = content
                    .iter()
                    .map(|i| i.to_output().unwrap())
                    .collect::<String>();

                Ok(format!(
                    "<details><summary>{}</summary>{}</details>",
                    sum_output, content_output
                ))
            }
            Token::HorizontalBreak => Ok(String::from("<hr>")),
            Token::NegatedVariable(val) => Ok(format!("<span>{}</span>", val)),
            Token::Variable(val) => Ok(format!("<var>{}</var>", val)),
            Token::Whitespace | Token::EOI | Token::Comment => Ok(String::new()),
        }
    }
}

/// A table
#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    /// The headers of the table
    pub headers: Vec<String>,
    /// The rows within the table
    pub rows: Vec<RichText>,
}

/// Generic Error type
#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    /// Niche errors
    Other(&'static str),
    /// The given source is invalid in some way
    Invalid,
}

impl std::error::Error for LexError {}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::Other(s) => {
                write!(f, "{}", s)
            }
            LexError::Invalid => {
                write!(f, "The given source has invalid syntax.")
            }
            _ => unimplemented!(),
        }
    }
}

/// Implemtors of this trait can generate HTML from a token.
pub trait Output {
    fn to_output(&self) -> Result<String, LexError>;
}

// std
use std::fmt;
use std::unimplemented;

// crates

// std

/// Represents a node.
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode<'a> {
    Heading { level: u8, text: &'a str },
    Paragraph(Text),
    Table(Table),
    Code(CodeBlock),
    Error(Error),
    List(List),
}

fn generate_rich_text_output(rich_text: RichText) -> String {
    match rich_text {
        RichText::Text(text) => text,
        RichText::Bold(text) => format!("<b>{}</b>", text),
        RichText::Italics(text) => format!("<i>{}</i>", text),
        RichText::StrikeThrough(text) => format!("<del>{}</del>", text),
        RichText::Subscript(text) => format!("<sub>{}</sub>", text),
        RichText::Superscript(text) => format!("<sup>{}</sup>", text),
        RichText::CodeBlock(text) => {
            format!("<code>{}</code>", text.contents)
        }
    }
}

impl Output for AstNode<'_> {
    fn to_output(&self) -> Result<String, Error> {
        match self {
            AstNode::Heading { level, text } => Ok(format!(
                "<h{level}>{text}<h{level}/>",
                level = &level.to_string(),
                text = text
            )),
            AstNode::Paragraph(text) => {
                let mut output = String::new();
                for i in text.children.clone() {
                    output.push_str(generate_rich_text_output(i).as_str());
                }

                Ok(output)
            }
            AstNode::List(list) => {
                let mut list_items = String::new();
                for i in list.items.children.clone() {
                    list_items
                        .push_str(format!("<li>{}</li>", generate_rich_text_output(i)).as_str());
                }

                match list.list_type {
                    ListType::Ordered => Ok(format!("<ol>{}</ol>", list_items)),
                    ListType::Unordered => Ok(format!("<ul>{}</ul>", list_items)),
                }
            }
            AstNode::Code(code) => match code.block_type {
                CodeType::Inline => Ok(format!("<code>{}</code>", code.contents)),
                CodeType::Fenced => Ok(format!("<pre><code>{}</code></pre>", code.contents)),
            },
            AstNode::Table(table) => {
                let mut headers = String::new();

                for i in table.headers.clone() {
                    headers.push_str(format!("<th>{}</th>", i).as_str());
                }

                let mut rows = String::new();

                table
                    .rows
                    .clone()
                    .into_iter()
                    .enumerate()
                    .for_each(|(idx, i)| {
                        let mut row = String::new();
                        for text in table.rows.get(idx) {
                            for rich_text in text.children.clone() {
                                row.push_str(format!(
                                    "<td>{}</td>",
                                    generate_rich_text_output(rich_text)
                                ).as_str());
                            }
                        }
                        rows.push_str(format!("<tr>{}</tr>", row).as_str())
                    });

                Ok(format!(
                    "<table><tr>{headers}</tr>{rows}</table>",
                    headers = headers,
                    rows = rows
                ))
            }
            AstNode::Error(error) => Ok(error.to_string()),
            _ => unimplemented!(),
        }
    }
}

/// Represents a type of rich text within a paragraph block.
#[derive(Debug, Clone, PartialEq)]
pub enum RichText {
    /// Italics, denoted using opening and closing asterisks (`*`)
    Italics(String),
    /// Bold text, denoted using opening and closing double asterisks (`**`)
    Bold(String),
    /// Subscript, denoted using opening and closing triple underscores (`___`)
    Subscript(String),
    /// Superscript, denoted using opening and closing carets (`^`)
    Superscript(String),
    /// An inline code block, denoted using opening and closing backticks
    CodeBlock(CodeBlock),
    /// Strikethrough text, denoted using opening and closing double tildes (`~~`)
    StrikeThrough(String),
    /// Normal text.
    Text(String),
}

/// Represents a paragraph.
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub children: Vec<RichText>,
}

/// A code block
#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    /// The type of code block
    pub block_type: CodeType,
    /// The contents of the code block
    pub contents: String,
}

/// The type of code block
#[derive(Debug, Clone, PartialEq)]
pub enum CodeType {
    /// Separate code
    Fenced,
    /// Inline code within a paragraph
    Inline,
}

/// Represents an ordered or unordered list.
#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub list_type: ListType,
    pub items: Text,
}

/// Represents a type of list
#[derive(Debug, Clone, PartialEq)]
pub enum ListType {
    /// Ordered list (`ol` element)
    Ordered,
    /// Unordered list (`ul` element)
    Unordered,
}

/// A table
#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    /// The headers of the table
    pub headers: Vec<String>,
    /// The rows within the table
    pub rows: Vec<Text>,
}

/// Generic Error type
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Niche errors
    Other(String),
    /// The given source is invalid in some way
    Invalid,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Other(s) => {
                write!(f, "{}", s)
            }
            Error::Invalid => {
                write!(f, "The given source has invalid syntax.")
            }
            _ => unimplemented!(),
        }
    }
}

/// Implemtors of this trait can generate HTML from an AST node.
pub trait Output {
    fn to_output(&self) -> Result<String, Error>;
}

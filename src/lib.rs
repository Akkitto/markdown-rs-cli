//! Library conversion helpers for `markdown-rs-cli`.
//!
//! `markdown` parser docs:
//! https://docs.rs/markdown/latest/markdown/
//!
//! `mdast_util_to_markdown` serializer docs:
//! https://docs.rs/mdast_util_to_markdown/latest/mdast_util_to_markdown/

use anyhow::{Context, Result, anyhow};
use clap::ValueEnum;
use markdown::{Constructs, ParseOptions, mdast::Node};

pub const DEFAULT_MAX_BYTES: usize = 10_000_000;

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
pub enum MarkdownDialect {
    Commonmark,
    Gfm,
    Mdx,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MdastOptions {
    pub dialect: MarkdownDialect,
    pub frontmatter: bool,
    pub math: bool,
    pub strict_gfm: bool,
    pub single_dollar_math: bool,
    pub compact: bool,
}

impl Default for MdastOptions {
    fn default() -> Self {
        Self {
            dialect: MarkdownDialect::Commonmark,
            frontmatter: false,
            math: false,
            strict_gfm: false,
            single_dollar_math: true,
            compact: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MarkdownSerializeOptions {
    pub single_dollar_math: bool,
}

impl Default for MarkdownSerializeOptions {
    fn default() -> Self {
        Self {
            single_dollar_math: true,
        }
    }
}

pub fn markdown_to_mdast(markdown_source: &str, options: &MdastOptions) -> Result<Node> {
    let parse_options = parse_options(options);

    markdown::to_mdast(markdown_source, &parse_options)
        .map_err(|error| anyhow!("failed to parse Markdown as mdast: {error}"))
}

pub fn markdown_to_mdast_json(markdown_source: &str, options: &MdastOptions) -> Result<String> {
    let tree = markdown_to_mdast(markdown_source, options)?;

    if options.compact {
        serde_json::to_string(&tree).context("failed to serialize mdast as compact JSON")
    } else {
        serde_json::to_string_pretty(&tree).context("failed to serialize mdast as pretty JSON")
    }
}

pub fn mdast_json_to_markdown(
    mdast_json: &str,
    options: &MarkdownSerializeOptions,
) -> Result<String> {
    let tree: Node =
        serde_json::from_str(mdast_json).context("input is not valid markdown-rs mdast JSON")?;

    let serialize_options = mdast_util_to_markdown::Options {
        single_dollar_text_math: options.single_dollar_math,
        ..mdast_util_to_markdown::Options::default()
    };

    let markdown = mdast_util_to_markdown::to_markdown_with_options(&tree, &serialize_options)
        .map_err(|error| anyhow!("failed to serialize mdast JSON to Markdown: {error}"))?;

    Ok(remove_unnecessary_url_underscore_escapes(&markdown))
}

fn parse_options(options: &MdastOptions) -> ParseOptions {
    let mut parse_options = match options.dialect {
        MarkdownDialect::Commonmark => ParseOptions::default(),
        MarkdownDialect::Gfm => ParseOptions::gfm(),
        MarkdownDialect::Mdx => ParseOptions::mdx(),
    };

    if options.frontmatter {
        parse_options.constructs.frontmatter = true;
    }

    if options.math {
        parse_options.constructs = Constructs {
            math_flow: true,
            math_text: true,
            ..parse_options.constructs
        };
    }

    if options.strict_gfm {
        parse_options.gfm_strikethrough_single_tilde = false;
    }

    parse_options.math_text_single_dollar = options.single_dollar_math;

    parse_options
}

fn remove_unnecessary_url_underscore_escapes(markdown: &str) -> String {
    let bytes = markdown.as_bytes();
    let mut output = String::with_capacity(markdown.len());
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'\\'
            && bytes.get(index + 1) == Some(&b'_')
            && is_inside_url_like_token(markdown, index)
        {
            output.push('_');
            index += 2;
            continue;
        }

        let character = markdown[index..]
            .chars()
            .next()
            .expect("index must point to a valid UTF-8 character boundary");

        output.push(character);
        index += character.len_utf8();
    }

    output
}

fn is_inside_url_like_token(markdown: &str, escape_index: usize) -> bool {
    let bytes = markdown.as_bytes();

    let mut start = escape_index;
    while start > 0 && !is_url_token_boundary(bytes[start - 1]) {
        start -= 1;
    }

    let mut end = escape_index + 2;
    while end < bytes.len() && !is_url_token_boundary(bytes[end]) {
        end += 1;
    }

    has_url_scheme(&markdown[start..end])
}

fn is_url_token_boundary(byte: u8) -> bool {
    byte.is_ascii_whitespace()
        || matches!(
            byte,
            b'<' | b'>' | b'(' | b')' | b'[' | b']' | b'{' | b'}' | b'"' | b'\'' | b'`'
        )
}

fn has_url_scheme(token: &str) -> bool {
    let Some(scheme_end) = token.find("://") else {
        return false;
    };

    let scheme = &token[..scheme_end];

    !scheme.is_empty()
        && scheme
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.'))
}

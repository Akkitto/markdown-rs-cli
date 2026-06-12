use anyhow::{Context, Result};
use markdown::{ParseOptions, to_mdast};

/// Markdown dialect preset used before applying extra construct toggles.
///
/// markdown-rs docs:
/// https://docs.rs/markdown/latest/markdown/
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MarkdownDialect {
    CommonMark,
    Gfm,
    Mdx,
}

/// Parser/output options independent from the CLI layer.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MdastOptions {
    pub dialect: MarkdownDialect,
    pub frontmatter: bool,
    pub math: bool,
    pub strict_gfm: bool,
    pub single_dollar_math: bool,
    pub pretty: bool,
}

impl Default for MdastOptions {
    fn default() -> Self {
        Self {
            dialect: MarkdownDialect::CommonMark,
            frontmatter: false,
            math: false,
            strict_gfm: false,
            single_dollar_math: true,
            pretty: true,
        }
    }
}

/// Convert Markdown text to mdast JSON.
///
/// `markdown::to_mdast()`:
/// https://docs.rs/markdown/latest/markdown/fn.to_mdast.html
///
/// The `markdown` crate's `serde` feature makes the AST serializable:
/// https://docs.rs/markdown/latest/markdown/#features
pub fn markdown_to_mdast_json(markdown: &str, options: &MdastOptions) -> Result<String> {
    let parse_options = build_parse_options(options);

    let ast = to_mdast(markdown, &parse_options)
        .map_err(|error| anyhow::anyhow!("failed to parse Markdown: {error:?}"))?;

    if options.pretty {
        serde_json::to_string_pretty(&ast).context("failed to serialize mdast as pretty JSON")
    } else {
        serde_json::to_string(&ast).context("failed to serialize mdast as compact JSON")
    }
}

fn build_parse_options(options: &MdastOptions) -> ParseOptions {
    let mut parse_options = match options.dialect {
        MarkdownDialect::CommonMark => ParseOptions::default(),
        MarkdownDialect::Gfm => ParseOptions::gfm(),
        MarkdownDialect::Mdx => ParseOptions::mdx(),
    };

    // Add optional constructs on top of the selected dialect.
    parse_options.constructs.frontmatter |= options.frontmatter;
    parse_options.constructs.math_flow |= options.math;
    parse_options.constructs.math_text |= options.math;

    // markdown-rs defaults to GitHub-style single-tilde strikethrough.
    // This disables that behavior for stricter GFM parsing.
    parse_options.gfm_strikethrough_single_tilde = !options.strict_gfm;

    // Single-dollar inline math can conflict with ordinary currency text.
    parse_options.math_text_single_dollar = options.single_dollar_math;

    parse_options
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_heading_to_json() {
        let json = markdown_to_mdast_json("# Title\n\n## Child\n\nText.", &MdastOptions::default())
            .expect("Markdown should parse");

        assert!(json.contains("\"heading\"") || json.contains("\"Heading\""));
        assert!(json.contains("Title"));
        assert!(json.contains("Child"));
    }

    #[test]
    fn supports_gfm_task_lists() {
        let options = MdastOptions {
            dialect: MarkdownDialect::Gfm,
            ..MdastOptions::default()
        };

        let json =
            markdown_to_mdast_json("- [x] done\n", &options).expect("GFM Markdown should parse");

        assert!(json.contains("done"));
    }
}

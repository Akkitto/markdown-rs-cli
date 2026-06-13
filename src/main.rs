use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use markdown::mdast::Node;
use markdown_rs_cli::{
    DEFAULT_MAX_BYTES, MarkdownDialect, MarkdownSerializeOptions, MdastOptions,
    markdown_to_mdast_json, mdast_json_to_markdown,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
enum InputFormat {
    Auto,
    Markdown,
    Mdast,
}

#[derive(Debug, Parser)]
#[command(
    name = "markdown-rs-cli",
    version,
    about = "Convert Markdown to markdown-rs mdast JSON, or mdast JSON to canonical Markdown."
)]
struct Cli {
    /// Input file. Reads stdin when omitted or set to "-".
    #[arg(value_name = "INPUT")]
    input: Option<PathBuf>,

    /// Output file. Writes stdout when omitted or set to "-".
    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<PathBuf>,

    /// Input format. Auto-detects valid markdown-rs mdast JSON by default.
    #[arg(long = "from", value_enum, default_value = "auto")]
    from: InputFormat,

    /// Markdown dialect preset for Markdown -> mdast.
    #[arg(long, value_enum, default_value = "commonmark")]
    dialect: MarkdownDialect,

    /// Enable frontmatter parsing for Markdown -> mdast.
    #[arg(long)]
    frontmatter: bool,

    /// Enable flow and inline math parsing for Markdown -> mdast.
    #[arg(long)]
    math: bool,

    /// Disable GitHub-style single-tilde strikethrough for Markdown -> mdast.
    #[arg(long)]
    strict_gfm: bool,

    /// Disable single-dollar inline math parsing/serialization.
    #[arg(long)]
    no_single_dollar_math: bool,

    /// Emit compact JSON instead of pretty JSON for Markdown -> mdast.
    #[arg(long)]
    compact: bool,

    /// Refuse input larger than this many bytes.
    #[arg(long, default_value_t = DEFAULT_MAX_BYTES, value_name = "BYTES")]
    max_bytes: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let input = read_input(cli.input.as_deref(), cli.max_bytes)?;
    let input_format = match cli.from {
        InputFormat::Auto => detect_input_format(&input),
        explicit_input_format => explicit_input_format,
    };

    let mdast_options = MdastOptions {
        dialect: cli.dialect,
        frontmatter: cli.frontmatter,
        math: cli.math,
        strict_gfm: cli.strict_gfm,
        single_dollar_math: !cli.no_single_dollar_math,
        compact: cli.compact,
    };

    let markdown_options = MarkdownSerializeOptions {
        single_dollar_math: !cli.no_single_dollar_math,
    };

    let output = match input_format {
        InputFormat::Auto => unreachable!("auto input format must be resolved before conversion"),
        InputFormat::Markdown => markdown_to_mdast_json(&input, &mdast_options)?,
        InputFormat::Mdast => mdast_json_to_markdown(&input, &markdown_options)?,
    };

    write_output(cli.output.as_deref(), &output)
}

fn detect_input_format(input: &str) -> InputFormat {
    if serde_json::from_str::<Node>(input).is_ok() {
        InputFormat::Mdast
    } else {
        InputFormat::Markdown
    }
}

fn read_input(input: Option<&Path>, max_bytes: usize) -> Result<String> {
    match input {
        Some(path) if !is_dash(path) => {
            let file = File::open(path)
                .with_context(|| format!("failed to open input file {}", path.display()))?;
            read_string_limited(file, max_bytes)
        }
        _ => {
            let stdin = io::stdin();
            read_string_limited(stdin.lock(), max_bytes)
        }
    }
}

fn read_string_limited<R>(reader: R, max_bytes: usize) -> Result<String>
where
    R: Read,
{
    let read_limit = max_bytes
        .checked_add(1)
        .context("--max-bytes is too large")?;
    let read_limit = u64::try_from(read_limit).context("--max-bytes is too large")?;

    let mut bytes = Vec::new();
    let mut limited_reader = reader.take(read_limit);
    limited_reader
        .read_to_end(&mut bytes)
        .context("failed to read input")?;

    if bytes.len() > max_bytes {
        bail!("input exceeds --max-bytes limit ({max_bytes} bytes)");
    }

    String::from_utf8(bytes).context("input is not valid UTF-8")
}

fn write_output(output: Option<&Path>, contents: &str) -> Result<()> {
    match output {
        Some(path) if !is_dash(path) => {
            std::fs::write(path, contents)
                .with_context(|| format!("failed to write output file {}", path.display()))?;
        }
        _ => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            handle
                .write_all(contents.as_bytes())
                .context("failed to write stdout")?;
        }
    }

    Ok(())
}

fn is_dash(path: &Path) -> bool {
    path.as_os_str() == OsStr::new("-")
}

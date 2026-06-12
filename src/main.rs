use std::{
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use markdown_rs_cli::{MarkdownDialect, MdastOptions, markdown_to_mdast_json};

/// Convert Markdown to markdown-rs mdast JSON.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Input Markdown file. Reads stdin when omitted or when set to "-".
    input: Option<PathBuf>,

    /// Output JSON file. Writes stdout when omitted or when set to "-".
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Markdown dialect to parse.
    #[arg(long, value_enum, default_value_t = DialectArg::Commonmark)]
    dialect: DialectArg,

    /// Enable frontmatter parsing.
    #[arg(long)]
    frontmatter: bool,

    /// Enable math parsing.
    #[arg(long)]
    math: bool,

    /// Disable GitHub-style single-tilde strikethrough.
    ///
    /// Useful when you want stricter GFM behavior.
    #[arg(long)]
    strict_gfm: bool,

    /// Disable single-dollar inline math.
    ///
    /// Useful for documents containing normal currency text like "$10".
    #[arg(long)]
    no_single_dollar_math: bool,

    /// Emit compact JSON instead of pretty JSON.
    #[arg(long)]
    compact: bool,

    /// Maximum accepted input size in bytes.
    ///
    /// markdown-rs itself recommends capping untrusted input size.
    /// Security notes:
    /// https://github.com/wooorm/markdown-rs#security
    #[arg(long, default_value_t = 10_000_000)]
    max_bytes: u64,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum DialectArg {
    Commonmark,
    Gfm,
    Mdx,
}

impl From<DialectArg> for MarkdownDialect {
    fn from(value: DialectArg) -> Self {
        match value {
            DialectArg::Commonmark => Self::CommonMark,
            DialectArg::Gfm => Self::Gfm,
            DialectArg::Mdx => Self::Mdx,
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let markdown = read_input(args.input.as_deref(), args.max_bytes)?;

    let options = MdastOptions {
        dialect: args.dialect.into(),
        frontmatter: args.frontmatter,
        math: args.math,
        strict_gfm: args.strict_gfm,
        single_dollar_math: !args.no_single_dollar_math,
        pretty: !args.compact,
    };

    let json = markdown_to_mdast_json(&markdown, &options)?;

    write_output(args.output.as_deref(), &json)?;

    Ok(())
}

fn read_input(input: Option<&Path>, max_bytes: u64) -> Result<String> {
    match input {
        Some(path) if path != Path::new("-") => {
            let file = fs::File::open(path)
                .with_context(|| format!("failed to open input file: {}", path.display()))?;

            read_with_limit(file, max_bytes, &path.display().to_string())
        }
        _ => {
            let stdin = io::stdin();
            read_with_limit(stdin.lock(), max_bytes, "stdin")
        }
    }
}

fn read_with_limit<R: Read>(reader: R, max_bytes: u64, source: &str) -> Result<String> {
    let read_limit = max_bytes
        .checked_add(1)
        .context("--max-bytes is too large")?;

    let mut limited_reader = reader.take(read_limit);
    let mut buffer = Vec::new();

    limited_reader
        .read_to_end(&mut buffer)
        .with_context(|| format!("failed to read Markdown from {source}"))?;

    let actual_bytes = u64::try_from(buffer.len()).context("input size does not fit into u64")?;

    if actual_bytes > max_bytes {
        bail!("input from {source} exceeds --max-bytes limit of {max_bytes} bytes");
    }

    String::from_utf8(buffer).with_context(|| format!("input from {source} is not valid UTF-8"))
}

fn write_output(output: Option<&Path>, json: &str) -> Result<()> {
    match output {
        Some(path) if path != Path::new("-") => {
            fs::write(path, format!("{json}\n"))
                .with_context(|| format!("failed to write output file: {}", path.display()))?;
        }
        _ => {
            let mut stdout = io::stdout().lock();

            stdout
                .write_all(json.as_bytes())
                .context("failed to write JSON to stdout")?;

            stdout
                .write_all(b"\n")
                .context("failed to write trailing newline to stdout")?;

            stdout.flush().context("failed to flush stdout")?;
        }
    }

    Ok(())
}

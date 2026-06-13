# markdown-rs-cli

Convert Markdown to `markdown-rs` [mdast](https://github.com/syntax-tree/mdast) JSON, and convert mdast JSON to canonical Markdown.

## What

`markdown-rs-cli` is a small command-line tool around the Rust `markdown` crate, also known as `markdown-rs`.

It parses Markdown input and writes the resulting [mdast](https://github.com/syntax-tree/mdast)-compatible abstract syntax tree as JSON.

It can also read exported mdast JSON and serialize it to Markdown.

By default, the CLI auto-detects the input format:

* valid `markdown-rs` mdast JSON is converted to canonical Markdown
* all other input is parsed as Markdown and emitted as mdast JSON

Use `--from markdown` or `--from mdast` to force one direction explicitly.

Input can come from:

* a Markdown file
* an mdast JSON file
* stdin

Output can go to:

* stdout
* a JSON file
* a Markdown file

Supported parser modes:

* CommonMark
* GitHub Flavored Markdown
* MDX
* optional frontmatter
* optional math

## Why

Markdown is easy for humans to write, but awkward for automation when it stays plain text.

This tool exists to make Markdown documents usable in scripts, CI jobs, static analysis, documentation tooling, indexing, and custom converters.

It is useful when you want to:

* inspect Markdown structurally
* extract headings, sections, links, lists, or text nodes
* convert Markdown into your own JSON/YAML schema
* build documentation automation without relying on fragile regular expressions
* keep the raw parser output available before creating opinionated projections
* serialize mdast JSON back to canonical Markdown

The tool intentionally emits raw [mdast](https://github.com/syntax-tree/mdast) JSON instead of inventing its own schema.

Reverse conversion produces canonical Markdown from mdast JSON. It is not guaranteed to restore the original Markdown byte-for-byte, because mdast is an abstract syntax tree and does not preserve every concrete formatting detail.

## How

### Parse a file to stdout

```bash
markdown-rs-cli README.md
```

### Parse a file to another file

```bash
markdown-rs-cli README.md --output README.mdast.json
```

### Parse stdin to a file

```bash
cat README.md | markdown-rs-cli --output README.mdast.json
```

### Explicit stdin/stdout

```bash
markdown-rs-cli - --output -
```

### Parse as GitHub Flavored Markdown

```bash
markdown-rs-cli --dialect gfm README.md
```

### Enable frontmatter and math

```bash
markdown-rs-cli --frontmatter --math README.md
```

### Emit compact JSON

```bash
markdown-rs-cli --compact README.md
```

### Convert mdast JSON back to Markdown

```bash
markdown-rs-cli README.mdast.json --output README.restored.md
```

### Convert mdast JSON from stdin

```bash
cat README.mdast.json | markdown-rs-cli > README.restored.md
```

### Force mdast JSON input

```bash
markdown-rs-cli --from mdast README.mdast.json --output README.restored.md
```

### Force Markdown input

```bash
markdown-rs-cli --from markdown README.md --output README.mdast.json
```

### Markdown round-trip workflow

```bash
markdown-rs-cli README.md --output README.mdast.json
markdown-rs-cli README.mdast.json --output README.restored.md
```

The restored Markdown is canonical Markdown generated from the mdast tree.

It is not guaranteed to be byte-for-byte identical to the original input, because mdast is an abstract syntax tree and does not preserve every concrete Markdown formatting detail, such as marker choice, wrapping, blank line count, escaping style, or trailing whitespace.

### Options

```text
Usage:
  markdown-rs-cli [OPTIONS] [INPUT]

Arguments:
  [INPUT]
    Input file.
    Reads stdin when omitted or when set to "-".
    By default, input format is auto-detected.

Options:
  -o, --output <OUTPUT>
    Output file.
    Writes stdout when omitted or when set to "-".
    With Markdown input, output is mdast JSON.
    With mdast JSON input, output is Markdown.

  --from <auto|markdown|mdast>
    Input format.
    "auto" treats valid markdown-rs mdast JSON as mdast input.
    Otherwise input is treated as Markdown.
    Default: auto

  --dialect <commonmark|gfm|mdx>
    Markdown dialect preset for Markdown input.
    Default: commonmark

  --frontmatter
    Enable frontmatter parsing for Markdown input.

  --math
    Enable flow and inline math parsing for Markdown input.

  --strict-gfm
    Disable GitHub-style single-tilde strikethrough for Markdown input.

  --no-single-dollar-math
    Disable single-dollar inline math parsing and serialization.

  --compact
    Emit compact JSON instead of pretty JSON.
    Only applies when outputting mdast JSON.

  --max-bytes <BYTES>
    Refuse input larger than this size.
    Default: 10000000

  -h, --help
    Print help.

  -V, --version
    Print version.
```

## Where

### Install from source

```bash
git clone https://github.com/Akkitto/markdown-rs-cli.git
cd markdown-rs-cli

cargo install --path .
```

### Run from source without installing

```bash
cargo run -- README.md
```

### Build release binary

```bash
cargo build --release
```

The compiled binary is written to:

```text
target/release/markdown-rs-cli
```

### Install from crates.io

```bash
cargo install markdown-rs-cli
```

### Install from Git

```bash
cargo install --git https://github.com/Akkitto/markdown-rs-cli.git
```

## Contribute

Contributions are welcome.

This project uses `pre-commit` for local Rust quality checks.

Install `pre-commit`:

```bash
pipx install pre-commit
```

Install the repository hooks:

```bash
pre-commit install --install-hooks
```

Run all hooks manually before opening a pull request:

```bash
pre-commit run --all-files
```

The committed `.pre-commit-config.yaml` is expected to run the project checks:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Recommended contribution rules:

* install and use the committed `pre-commit` hooks
* keep the CLI small and predictable
* keep parser logic in `src/lib.rs`
* keep CLI argument handling in `src/main.rs`
* add integration tests for user-visible CLI behavior
* do not change JSON output expectations without tests
* prefer explicit options over hidden behavior
* keep stdout/stderr behavior script-friendly
* keep dependencies minimal
* commit `Cargo.lock`, because this is a CLI application

For larger changes, open an issue first and describe:

* the use case
* the proposed CLI behavior
* expected input and output examples
* compatibility concerns

Git hooks are local to each clone. CI should still run the same checks for enforcement.

## Licence

Copyright © 2026  [Daniel Braniewski](https://brani.dev/)

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

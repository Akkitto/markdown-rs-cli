# markdown-rs-cli

Convert Markdown to `markdown-rs` [mdast](https://github.com/syntax-tree/mdast) JSON.

## What

`markdown-rs-cli` is a small command-line tool around the Rust `markdown` crate, also known as `markdown-rs`.

It parses Markdown input and writes the resulting [mdast](https://github.com/syntax-tree/mdast)-compatible abstract syntax tree as JSON.

Input can come from:

- a Markdown file
- stdin

Output can go to:

- stdout
- a JSON file

Supported parser modes:

- CommonMark
- GitHub Flavored Markdown
- MDX
- optional frontmatter
- optional math

## Why

Markdown is easy for humans to write, but awkward for automation when it stays plain text.

This tool exists to make Markdown documents usable in scripts, CI jobs, static analysis, documentation tooling, indexing, and custom converters.

It is useful when you want to:

- inspect Markdown structurally
- extract headings, sections, links, lists, or text nodes
- convert Markdown into your own JSON/YAML schema
- build documentation automation without relying on fragile regular expressions
- keep the raw parser output available before creating opinionated projections

The tool intentionally emits raw [mdast](https://github.com/syntax-tree/mdast) JSON instead of inventing its own schema.

## How

### Parse a file to stdout

```bash
markdown-rs-cli README.md
````

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

### Options

```text
Usage:
  markdown-rs-cli [OPTIONS] [INPUT]

Arguments:
  [INPUT]
    Input Markdown file.
    Reads stdin when omitted or when set to "-".

Options:
  -o, --output <OUTPUT>
    Output JSON file.
    Writes stdout when omitted or when set to "-".

  --dialect <commonmark|gfm|mdx>
    Markdown dialect preset.
    Default: commonmark

  --frontmatter
    Enable frontmatter parsing.

  --math
    Enable flow and inline math parsing.

  --strict-gfm
    Disable GitHub-style single-tilde strikethrough.

  --no-single-dollar-math
    Disable single-dollar inline math parsing.

  --compact
    Emit compact JSON instead of pretty JSON.

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

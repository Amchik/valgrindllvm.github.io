# FMF Markdown Language

FMF (_field, millet, fox_) is a format for markdown-like pages.

## Usage

`fmf/` is a main format library. `fmfcc/` is a compiler and "project manager" for
fmf format. To generate HTML from `.fmf` file just use `fmfcc cc`:

```console
# Generate file
$ fmfcc cc -o /tmp/foo.html foo.fmf

# Query file metadata in human-readable format:
$ fmfcc q foo.fmf
Title: Some title

Author: ValgrindLLVM
Date: 24 Feb 2023
Category: example

# Query file metadata in json:
$ fmfcc q --json bar.fmf
{ "title": "Some title", "author": null, "date": null, "category": null }
```

To build this site use `Makefile` and `make` tool:

```console
# Build in release mode:
$ make all

# Build in dev mode (do not remove templates)
$ make dev

# Run web server (using netcat)
$ make serve

# Run web server with build on reload:
$ ./serve.sh 8080 autoreload
```

## Documentation

*TIP*: do `cargo doc --open` in project directory to read full documentation.

There only 0 contexts, 0 inline formatting and 0 special formats.

### Special formats

Special formats is one-line format, like headings or cards.

#### Heading

Like markdown, headings defined using `#` symbol:

```markdown
# Heading 1
## Heading 2
###### Heading 6 (maximum value)
```

#### Author card

Usually author cards adding after h1.

```markdown
/// author$name / date / category
```

It will be formatted as Author author$name with avatar /authorname.jpg.

### Contexts

Context is multi-line markdown like paragraphs or code blocks.

#### Paragraph

Paragraph is just text without blank lines. Example:

```markdown
Paragraph 1.
Still paragraph 1.

Paragraph 2.
```

#### Code blocks

Like markdown, code blocks defined using three `` ` `` symbols:

```markdown
(due markdown limits, code blocks here starts with space, but it not required)
 ```language: foo, filename: bar, ...
int a = 10;
print(\*a\*); // To use inline-formatting in code blocks add `\` before format symbol
 ```_
Please ignore `_` in line upper
```

#### Ordered and unordered lists

```markdown
Ordered list:

@ First
@ Second
@ Half-Life

Unordered list:

- Foo
- Bar
- Pizza
```

### Inline-formatting

Inline-formatting is formatting just in line, like *bold*, `code`, etc...

1. *Bold*. Writes using `*`: `normal *bold*`
2. _Italic_. Writes using `_`: `normal _italic_`
3. `Code`. Writes using `` ` ``: ``normal `code` ``
4. Links. Example: `\(google.com)`, `\(google.com)[Google]`, `\[Just blue text]`


use std::{
    fmt::{Display, Write},
    marker::PhantomData,
};

use crate::format::{DefaultFormatEngine, FormatEngine, FormatType, ListType};

/// Provides text formatting (inline-formatting)
pub struct Text<'a, E: FormatEngine = DefaultFormatEngine> {
    pub v: &'a str,
    pub _marker: PhantomData<E>,
}

impl<'a> Text<'a> {
    #[inline(always)]
    pub fn text(v: &'a str) -> Self {
        Self::new(v)
    }
}
impl<'a, E: FormatEngine> Text<'a, E> {
    #[inline(always)]
    pub fn new(v: &'a str) -> Self {
        Self {
            v,
            _marker: PhantomData,
        }
    }
}

impl<'a, E: FormatEngine> Display for Text<'a, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[derive(Default)]
        struct Inline {
            bold: bool,
            italic: bool,
            code: bool,
            link: u8,
        }
        let mut inline = Inline::default();
        let exec_inline = |f: &mut std::fmt::Formatter<'_>, v: &mut bool, ty: FormatType| {
            *v = !*v;

            if !*v {
                f.write_str(E::end(ty))
            } else {
                f.write_str(E::begin(ty))
            }
        };

        let mut lstart = 0;
        let mut escape = E::escape().is_none();
        for (idx, c) in self.v.char_indices() {
            if Some(c) == E::escape() {
                escape = true;
                continue;
            }

            match c {
                '&' => f.write_str("&amp;")?,
                '<' => f.write_str("&lt;")?,
                '>' => f.write_str("&gt;")?,

                _ if !escape => f.write_char(c)?,

                '\\' if inline.link == 0 => {
                    inline.link = 1;
                }
                '(' if inline.link == 1 => {
                    f.write_str(E::begin(FormatType::InlineLink))?;
                    f.write_str(E::begin(FormatType::InlineLinkUrl))?;
                    inline.link = 2;
                    lstart = idx + 1;
                }
                '[' if inline.link == 1 => {
                    f.write_str(E::begin(FormatType::InlineLink))?;
                    f.write_str(E::begin(FormatType::InlineLinkText))?;
                    inline.link = 4;
                }
                _ if inline.link == 1 => {
                    f.write_char(c)?;
                    inline.link = 0;
                }
                ')' if inline.link == 2 => {
                    f.write_str(E::end(FormatType::InlineLinkUrl))?;
                    inline.link = 3;
                }
                '[' if inline.link == 3 => {
                    f.write_str(E::begin(FormatType::InlineLinkText))?;
                    inline.link = 4;
                }
                _ if inline.link == 3 => {
                    f.write_str(E::begin(FormatType::InlineLinkText))?;
                    f.write_str(&self.v[lstart..idx - 1])?;
                    f.write_str(E::end(FormatType::InlineLinkText))?;
                    f.write_str(E::end(FormatType::InlineLink))?;
                    f.write_char(c)?;
                    inline.link = 0;
                }
                ']' if inline.link == 4 => {
                    f.write_str(E::end(FormatType::InlineLinkText))?;
                    f.write_str(E::end(FormatType::InlineLink))?;
                    inline.link = 0;
                }

                '*' => exec_inline(f, &mut inline.bold, FormatType::InlineBold)?,
                '_' => exec_inline(f, &mut inline.italic, FormatType::InlineItalic)?,
                '`' => exec_inline(f, &mut inline.code, FormatType::InlineCode)?,

                _ => f.write_char(c)?,
            }

            escape = E::escape().is_none();
        }

        Ok(())
    }
}

/// Provides full document formatting. See module-level docs for more...
pub struct Document<'a, E: FormatEngine = DefaultFormatEngine> {
    pub v: &'a str,
    pub _marker: PhantomData<E>,
}

impl<'a, E: FormatEngine> Document<'a, E> {
    pub fn new(v: &'a str) -> Self {
        Document {
            v,
            _marker: PhantomData,
        }
    }

    /// Gets the first h1 title. If there no title returns empty string
    pub fn get_title(&self) -> &'a str {
        self.v
            .lines()
            .map(str::trim)
            .filter(|f| f.starts_with('#') && !f[1..].starts_with('#'))
            .map(|f| f[1..].trim_start())
            .next()
            .unwrap_or_default()
    }
    /// Gets the first author card if any
    pub fn get_author_card(&self) -> Option<(&'a str, &'a str, &'a str)> {
        self.v
            .lines()
            .map(str::trim)
            .filter(|s| s.starts_with("///"))
            .map(|f| f[3..].split('/').collect::<Vec<_>>())
            .filter(|v| v.len() == 3)
            .map(|f| (f[0].trim(), f[1].trim(), f[2].trim()))
            .next()
    }
}

struct ASCIIWriter<'a>(&'a str);
impl<'a> Display for ASCIIWriter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for fragment in self
            .0
            .split(|p| !matches!(p, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
        {
            f.write_str(fragment)?;
        }
        Ok(())
    }
}

impl<'a, E: FormatEngine> Display for Document<'a, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[derive(PartialEq, Eq)]
        enum State {
            Empty,
            Paragraph,
            Codeblock,
            List(ListType),
        }
        let mut state = State::Empty;

        for (raw_line, line) in self.v.lines().map(|l| (l, l.trim())) {
            match state {
                State::Codeblock if line != "```" => {
                    writeln!(f, "{}", Text::<E::CodeblockEngine>::new(raw_line))?
                }

                _ if line.is_empty() => {
                    match state {
                        State::Paragraph => f.write_str(E::end(FormatType::Paragraph))?,
                        State::Codeblock => write!(
                            f,
                            "{}{}",
                            E::end(FormatType::CodeBlockContents),
                            E::end(FormatType::CodeBlock)
                        )?,
                        State::List(l) => write!(
                            f,
                            "{}{}",
                            E::end(FormatType::ListItem(l)),
                            E::end(FormatType::List(l))
                        )?,
                        State::Empty => {}
                    }
                    state = State::Empty;
                }

                State::Empty if line.starts_with("//!") => {},

                State::Empty if line.starts_with("///") => {
                    let (name, date, category) = match line[3..].split('/').collect::<Vec<_>>() {
                        v if v.len() == 3 => (v[0], v[1], v[2]),
                        _ => continue,
                    };
                    write!(
                        f,
                        "{}{}/{}.jpg{}{}{}{}{}{}{}{}{}{}{}",
                        E::begin(FormatType::CardAuthor),
                        E::begin(FormatType::CardAuthorAvatar),
                        ASCIIWriter(name.trim()),
                        E::end(FormatType::CardAuthorAvatar),
                        E::begin(FormatType::CardAuthorName),
                        name.trim(),
                        E::end(FormatType::CardAuthorName),
                        E::begin(FormatType::CardAuthorDate),
                        date.trim(),
                        E::end(FormatType::CardAuthorDate),
                        E::begin(FormatType::CardAuthorCategory),
                        category.trim(),
                        E::end(FormatType::CardAuthorCategory),
                        E::end(FormatType::CardAuthor)
                    )?;
                }

                State::Empty if line.starts_with('#') => {
                    let cnt = line.bytes().take_while(|&f| f == b'#').count();
                    write!(
                        f,
                        "{}{}{}",
                        E::begin(FormatType::Heading(cnt as u8)),
                        Text::text(line[cnt..].trim_start()),
                        E::end(FormatType::Heading(cnt as u8)),
                    )?;
                }
                State::Empty if line.starts_with('-') => {
                    writeln!(
                        f,
                        "{}{}{}",
                        E::begin(FormatType::List(ListType::Unordered)),
                        E::begin(FormatType::ListItem(ListType::Unordered)),
                        Text::text(line[1..].trim_start()),
                    )?;
                    state = State::List(ListType::Unordered);
                }
                State::Empty if line.starts_with('@') => {
                    writeln!(
                        f,
                        "{}{}{}",
                        E::begin(FormatType::List(ListType::Ordered)),
                        E::begin(FormatType::ListItem(ListType::Ordered)),
                        Text::text(line[1..].trim_start()),
                    )?;
                    state = State::List(ListType::Ordered);
                }
                State::Empty if line.starts_with("```") => {
                    write!(
                        f,
                        "{}{}{}{}{}",
                        E::begin(FormatType::CodeBlock),
                        E::begin(FormatType::CodeBlockMeta),
                        Text::text(line[3..].trim_start()),
                        E::end(FormatType::CodeBlockMeta),
                        E::begin(FormatType::CodeBlockContents),
                    )?;
                    state = State::Codeblock;
                }
                State::Empty => {
                    writeln!(f, "{}{}", E::begin(FormatType::Paragraph), Text::text(line))?;
                    state = State::Paragraph;
                }

                State::List(ListType::Unordered) if line.starts_with('-') => {
                    writeln!(
                        f,
                        "{}{}{}",
                        E::end(FormatType::ListItem(ListType::Unordered)),
                        E::begin(FormatType::ListItem(ListType::Unordered)),
                        Text::text(line[1..].trim_start()),
                    )?;
                }
                State::List(ListType::Ordered) if line.starts_with('@') => {
                    writeln!(
                        f,
                        "{}{}{}",
                        E::end(FormatType::ListItem(ListType::Ordered)),
                        E::begin(FormatType::ListItem(ListType::Ordered)),
                        Text::text(line[1..].trim_start()),
                    )?;
                }

                State::Paragraph | State::List(_) => writeln!(f, "{}", Text::text(line))?,

                State::Codeblock => {
                    write!(
                        f,
                        "{}{}",
                        E::end(FormatType::CodeBlockContents),
                        E::end(FormatType::CodeBlock)
                    )?;
                    state = State::Empty;
                }
            }
        }

        Ok(())
    }
}

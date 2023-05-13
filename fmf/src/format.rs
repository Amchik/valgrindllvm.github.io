/// Type of list
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListType {
    /// Ordered list (1. 2. 3.)
    Ordered,
    /// Unordered list (-foo -bar -baz)
    Unordered,
}
/// Type of format that should be used.
///
/// # Notes
///
/// Some formats have unique syntax. Lets call `FOO` as start of
/// FOO and `/FOO` as end of FOO:
/// 1. `CodeBlock [CodeBlockMeta /CodeBlockMeta] CodeBlockContents [...] /CodeBlockContents /CodeBlock`
/// 2. `List [ListItem [...] /ListItem]... /List`
/// 3. `InlineLink InlineLinkUrl /InlineLinkUrl InlineLinkText [...] /InlineLinkText`
/// 4. `CardAuthor CardAuthorAvatar /CardAuthorAvatar CardAuthorName /CardAuthorName CardAuthorDate
///    /CardAuthorDate CardAuthorCategory /CardAuthorCategory`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatType {
    Heading(u8),
    Paragraph,
    List(ListType),
    ListItem(ListType),
    CodeBlock,
    CodeBlockMeta,
    CodeBlockContents,

    InlineBold,
    InlineItalic,
    InlineCode,
    InlineLink,
    InlineLinkUrl,
    InlineLinkText,

    CardAuthor,
    CardAuthorAvatar,
    CardAuthorName,
    CardAuthorDate,
    CardAuthorCategory,
}

/// Describes format engine
pub trait FormatEngine {
    type CodeblockEngine: FormatEngine;

    /// Gets inline escape
    fn escape() -> Option<char>;
    /// Gets begin of [`FormatType`]
    fn begin(ty: FormatType) -> &'static str;
    /// Gets end of [`FormatType`]
    fn end(ty: FormatType) -> &'static str;
}

/// Bind for default format engine
pub type DefaultFormatEngine = ParagraphEngine;

/// Paragraph engine. Formats into HTML. Implements trait [`FormatEngine`]
pub struct ParagraphEngine;
impl FormatEngine for ParagraphEngine {
    type CodeblockEngine = CodeblockEngine;

    fn escape() -> Option<char> {
        None
    }
    fn begin(ty: FormatType) -> &'static str {
        match ty {
            FormatType::Heading(1) => "<h1>",
            FormatType::Heading(2) => "<h2>",
            FormatType::Heading(3) => "<h3>",
            FormatType::Heading(4) => "<h4>",
            FormatType::Heading(5) => "<h5>",
            FormatType::Heading(_) => "<h6>",

            FormatType::Paragraph => "<p>",
            FormatType::List(ListType::Ordered) => "<ol>",
            FormatType::List(ListType::Unordered) => "<ul>",
            FormatType::ListItem(_) => "<li>",

            FormatType::CodeBlock => "<div class=\"codeblock\">",
            FormatType::CodeBlockMeta => "<div class=\"prelude\"><span>",
            FormatType::CodeBlockContents => "<pre>",

            FormatType::InlineBold => "<b>",
            FormatType::InlineItalic => "<i>",
            FormatType::InlineCode => "<code>",
            FormatType::InlineLink => "<a",
            FormatType::InlineLinkUrl => " href=\"",
            FormatType::InlineLinkText => ">",

            FormatType::CardAuthor => "<div class=\"author-card\">",
            FormatType::CardAuthorAvatar => "<img src=\"",
            FormatType::CardAuthorName => "<div class=\"text\"><span class=\"name\">",
            FormatType::CardAuthorDate => "<div class=\"info\"><span class=\"date\">",
            FormatType::CardAuthorCategory => "<span class=\"badge\">",
        }
    }
    fn end(ty: FormatType) -> &'static str {
        match ty {
            FormatType::Heading(1) => "</h1>",
            FormatType::Heading(2) => "</h2>",
            FormatType::Heading(3) => "</h3>",
            FormatType::Heading(4) => "</h4>",
            FormatType::Heading(5) => "</h5>",
            FormatType::Heading(_) => "</h6>",

            FormatType::Paragraph => "</p>",
            FormatType::List(ListType::Ordered) => "</ol>",
            FormatType::List(ListType::Unordered) => "</ul>",
            FormatType::ListItem(_) => "</li>",

            FormatType::CodeBlock => "</div>",
            FormatType::CodeBlockMeta => "</span></div>",
            FormatType::CodeBlockContents => "</pre>",

            FormatType::InlineBold => "</b>",
            FormatType::InlineItalic => "</i>",
            FormatType::InlineCode => "</code>",
            FormatType::InlineLink => "</a>",
            FormatType::InlineLinkUrl => "\"",
            FormatType::InlineLinkText => "",

            FormatType::CardAuthor => "</div>",
            FormatType::CardAuthorAvatar => "\" alt=\"\" width=\"32px\" height=\"32px\">",
            FormatType::CardAuthorName => "</span>",
            FormatType::CardAuthorDate => "</span>",
            FormatType::CardAuthorCategory => "</span></div></div>",
        }
    }
}
/// Codeblock engine. Formats into HTML. Uses [`ParagraphEngine`] for formatting, but provides `\` escape.
/// Implements trait [`FormatEngine`]
pub struct CodeblockEngine;
impl FormatEngine for CodeblockEngine {
    type CodeblockEngine = Self;

    fn escape() -> Option<char> {
        Some('\\')
    }
    #[inline(always)]
    fn begin(ty: FormatType) -> &'static str {
        ParagraphEngine::begin(ty)
    }
    #[inline(always)]
    fn end(ty: FormatType) -> &'static str {
        ParagraphEngine::end(ty)
    }
}


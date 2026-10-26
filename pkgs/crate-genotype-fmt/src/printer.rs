//! Width-aware document printing with flat/expanded groups and bounded lookahead.

use crate::prelude::internal::*;
use unicode_width::UnicodeWidthStr;

pub(crate) struct GtfmtPrinter;

impl GtfmtPrinter {
    pub(crate) fn print(elements: &[GtfmtDocEl<'_>], width: usize, capacity: usize) -> String {
        let mut output = String::with_capacity(capacity);
        let mut state = GtfmtPrinterLineState::new();
        let mut stack = vec![];
        Self::push(&mut stack, elements, 0, GtfmtPrinterMode::Expanded);

        while let Some(command) = stack.pop() {
            let GtfmtPrinterCommand {
                element,
                indent,
                mode,
            } = command;

            match element {
                GtfmtDocEl::Text(text) if !text.is_empty() => {
                    output.extend(std::iter::repeat_n(' ', state.prefix_width(indent)));
                    output.push_str(text);
                    if let Some((_, last)) = text.rsplit_once('\n') {
                        // Literal lines are opaque: do not indent or normalize their bytes.
                        state = GtfmtPrinterLineState {
                            column: Self::text_width(last),
                            start: last.is_empty(),
                            space: false,
                        };
                    } else {
                        state.text(text, indent);
                    }
                }

                GtfmtDocEl::Space => state.space(),

                GtfmtDocEl::Line(GtfmtDocElLine::Soft) if mode == GtfmtPrinterMode::Flat => {}

                GtfmtDocEl::Line(GtfmtDocElLine::SoftOrSpace) if mode == GtfmtPrinterMode::Flat => {
                    state.space()
                }

                GtfmtDocEl::Line(line) => {
                    if !output.is_empty() {
                        if !output.ends_with('\n') {
                            output.push('\n');
                        }
                        if matches!(line, GtfmtDocElLine::Empty) && !output.ends_with("\n\n") {
                            output.push('\n');
                        }
                    }
                    state = GtfmtPrinterLineState::new();
                }
                GtfmtDocEl::Group(GtfmtDocElGroup { content, expand }) => {
                    let mode = if !expand
                        && (mode == GtfmtPrinterMode::Flat
                            || Self::fits(content, indent, &stack, state, width))
                    {
                        GtfmtPrinterMode::Flat
                    } else {
                        GtfmtPrinterMode::Expanded
                    };
                    Self::push(&mut stack, content, indent, mode);
                }

                GtfmtDocEl::Indent(content) => Self::push(&mut stack, content, indent + 1, mode),

                GtfmtDocEl::IfBreak(content) if mode == GtfmtPrinterMode::Expanded => {
                    Self::push(&mut stack, content, indent, mode)
                }

                _ => {}
            }
        }

        output
    }

    fn text_width(text: &str) -> usize {
        // A tab in opaque source text has the same display width as one indent.
        text.split('\t').map(UnicodeWidthStr::width).sum::<usize>()
            + text.bytes().filter(|&byte| byte == b'\t').count() * GTFMT_INDENT_WIDTH
    }

    fn push<'doc, 'src>(
        stack: &mut Vec<GtfmtPrinterCommand<'doc, 'src>>,
        content: &'doc [GtfmtDocEl<'src>],
        indent: usize,
        mode: GtfmtPrinterMode,
    ) {
        stack.extend(content.iter().rev().map(|element| GtfmtPrinterCommand {
            element,
            indent,
            mode,
        }));
    }

    /// Measure a flat candidate plus the remainder of its line (including closing
    /// delimiters). The borrowed tail avoids copying the remaining module per group.
    fn fits<'doc, 'src>(
        content: &'doc [GtfmtDocEl<'src>],
        indent: usize,
        rest: &[GtfmtPrinterCommand<'doc, 'src>],
        mut state: GtfmtPrinterLineState,
        width: usize,
    ) -> bool {
        let mut stack = vec![];
        Self::push(&mut stack, content, indent, GtfmtPrinterMode::Flat);

        let mut tail = rest.iter().rev();
        while let Some(command) = stack.pop().or_else(|| tail.next().copied()) {
            let GtfmtPrinterCommand {
                element,
                indent,
                mode,
            } = command;

            match element {
                GtfmtDocEl::Text(text) if !text.is_empty() => {
                    if let Some((first, _)) = text.split_once('\n') {
                        state.text(first, indent);
                        return state.column <= width;
                    }
                    state.text(text, indent);
                }

                GtfmtDocEl::Space => state.space(),

                GtfmtDocEl::Line(GtfmtDocElLine::Soft) if mode == GtfmtPrinterMode::Flat => {}

                GtfmtDocEl::Line(GtfmtDocElLine::SoftOrSpace) if mode == GtfmtPrinterMode::Flat => {
                    state.space()
                }

                GtfmtDocEl::Line(_) => return state.column <= width,

                GtfmtDocEl::Group(GtfmtDocElGroup { content, expand }) => {
                    Self::push(
                        &mut stack,
                        content,
                        indent,
                        if *expand {
                            GtfmtPrinterMode::Expanded
                        } else {
                            GtfmtPrinterMode::Flat
                        },
                    );
                }

                GtfmtDocEl::Indent(content) => Self::push(&mut stack, content, indent + 1, mode),

                GtfmtDocEl::IfBreak(content) if mode == GtfmtPrinterMode::Expanded => {
                    Self::push(&mut stack, content, indent, mode)
                }

                _ => {}
            }

            if state.column > width {
                return false;
            }
        }

        state.column <= width
    }
}

#[derive(Clone, Copy, PartialEq)]
enum GtfmtPrinterMode {
    Flat,
    Expanded,
}

#[derive(Clone, Copy)]
struct GtfmtPrinterCommand<'doc, 'src> {
    element: &'doc GtfmtDocEl<'src>,
    indent: usize,
    mode: GtfmtPrinterMode,
}

#[derive(Clone, Copy)]
struct GtfmtPrinterLineState {
    column: usize,
    start: bool,
    space: bool,
}

impl GtfmtPrinterLineState {
    fn new() -> Self {
        Self {
            column: 0,
            start: true,
            space: false,
        }
    }

    fn space(&mut self) {
        if !self.start {
            self.space = true;
        }
    }

    fn prefix_width(&self, indent: usize) -> usize {
        if self.start {
            indent * GTFMT_INDENT_WIDTH
        } else {
            usize::from(self.space)
        }
    }

    fn text(&mut self, text: &str, indent: usize) {
        self.column += self.prefix_width(indent) + GtfmtPrinter::text_width(text);
        self.start = false;
        self.space = false;
    }
}

#[cfg(test)]
mod tests {
    use super::GtfmtPrinter;
    use crate::doc::*;
    use insta::assert_snapshot;

    fn render(doc: GtfmtElements<'_>, width: usize) -> String {
        GtfmtPrinter::print(&doc, width, 0)
    }

    #[test]
    fn groups_fit_exactly_and_measure_following_delimiters() {
        let mut doc = vec![GtfmtDocEl::text("(")];
        doc.extend(GtfmtDocEl::group(
            vec![
                GtfmtDocEl::text("aa"),
                GtfmtDocEl::line(),
                GtfmtDocEl::text("bb"),
            ],
            false,
        ));
        doc.push(GtfmtDocEl::text(")"));
        assert_snapshot!(render(doc.clone(), 7), @"(aa bb)");
        assert_snapshot!(render(doc, 6), @"
        (aa
        bb)
        ");
    }

    #[test]
    fn nested_groups_can_stay_flat_inside_an_expanded_parent() {
        let mut body = vec![GtfmtDocEl::soft_line()];
        body.extend(GtfmtDocEl::group(
            vec![
                GtfmtDocEl::text("a"),
                GtfmtDocEl::line(),
                GtfmtDocEl::text("b"),
            ],
            false,
        ));
        body.extend([GtfmtDocEl::text(","), GtfmtDocEl::line()]);
        body.extend(GtfmtDocEl::group(
            vec![
                GtfmtDocEl::text("c"),
                GtfmtDocEl::line(),
                GtfmtDocEl::text("d"),
            ],
            false,
        ));
        let mut doc = vec![GtfmtDocEl::text("[")];
        doc.extend(GtfmtDocEl::indent(body));
        doc.extend([GtfmtDocEl::soft_line(), GtfmtDocEl::text("]")]);
        assert_snapshot!(render(GtfmtDocEl::group(doc, false), 8), @"
        [
          a b,
          c d
        ]
        ");
    }

    #[test]
    fn mandatory_breaks_propagate_through_groups_and_indentation() {
        let mut doc = vec![GtfmtDocEl::text("head"), GtfmtDocEl::line()];
        doc.extend(GtfmtDocEl::indent(GtfmtDocEl::group(
            vec![
                GtfmtDocEl::text("a"),
                GtfmtDocEl::hard_line(),
                GtfmtDocEl::text("b"),
            ],
            false,
        )));
        assert_snapshot!(render(GtfmtDocEl::group(doc, false), 80), @"
        head
          a
          b
        ");

        let mut doc = vec![GtfmtDocEl::text("head"), GtfmtDocEl::line()];
        doc.extend(GtfmtDocEl::group(
            vec![
                GtfmtDocEl::text("a"),
                GtfmtDocEl::line(),
                GtfmtDocEl::text("b"),
            ],
            true,
        ));
        assert_snapshot!(render(GtfmtDocEl::group(doc, false), 80), @"
        head
        a
        b
        ");
    }

    #[test]
    fn conditional_content_uses_its_own_group() {
        let mut union = GtfmtDocEl::if_break(vec![GtfmtDocEl::text("|"), GtfmtDocEl::space()]);
        union.extend([
            GtfmtDocEl::text("alpha"),
            GtfmtDocEl::line(),
            GtfmtDocEl::text("| beta"),
        ]);
        let mut doc = vec![GtfmtDocEl::text("X:")];
        let mut value = vec![GtfmtDocEl::line()];
        value.extend(union);
        doc.extend(GtfmtDocEl::group(GtfmtDocEl::indent(value), false));
        assert_snapshot!(render(doc.clone(), 80), @"X: alpha | beta");
        assert_snapshot!(render(doc, 10), @"
        X:
          | alpha
          | beta
        ");
    }

    #[test]
    fn layout_whitespace_is_coalesced_and_never_trails() {
        let mut doc = vec![
            GtfmtDocEl::hard_line(),
            GtfmtDocEl::empty_line(),
            GtfmtDocEl::space(),
            GtfmtDocEl::text("a"),
            GtfmtDocEl::space(),
            GtfmtDocEl::space(),
            GtfmtDocEl::text("b"),
            GtfmtDocEl::space(),
            GtfmtDocEl::hard_line(),
            GtfmtDocEl::hard_line(),
            GtfmtDocEl::empty_line(),
            GtfmtDocEl::empty_line(),
        ];
        doc.extend(GtfmtDocEl::indent(vec![
            GtfmtDocEl::space(),
            GtfmtDocEl::text("c"),
            GtfmtDocEl::space(),
            GtfmtDocEl::hard_line(),
        ]));
        doc.extend([GtfmtDocEl::text("d"), GtfmtDocEl::space()]);
        assert_snapshot!(render(doc, 80), @"
        a b

          c
        d
        ");
    }

    #[test]
    fn measures_unicode_display_width_and_tabs() {
        let doc = GtfmtDocEl::group(
            vec![
                GtfmtDocEl::text("界"),
                GtfmtDocEl::line(),
                GtfmtDocEl::text("e\u{301}"),
            ],
            false,
        );
        assert_snapshot!(render(doc.clone(), 4), @"界 é");
        assert_snapshot!(render(doc, 3), @"
        界
        é
        ");
        let doc = GtfmtDocEl::group(
            vec![
                GtfmtDocEl::text("🚀"),
                GtfmtDocEl::line(),
                GtfmtDocEl::text("x"),
            ],
            false,
        );
        assert_snapshot!(render(doc.clone(), 4), @"🚀 x");
        assert_snapshot!(render(doc, 3), @"
        🚀
        x
        ");
        let doc = GtfmtDocEl::group(
            vec![
                GtfmtDocEl::text("a\tb"),
                GtfmtDocEl::line(),
                GtfmtDocEl::text("c"),
            ],
            false,
        );
        assert_snapshot!(render(doc.clone(), 6), @"a\tb c");
        assert_snapshot!(render(doc, 5), @"
        a\tb
        c
        ");
    }

    #[test]
    fn multiline_text_is_preserved_byte_for_byte() {
        let mut doc = vec![GtfmtDocEl::text("["), GtfmtDocEl::hard_line()];
        doc.extend(GtfmtDocEl::indent(vec![
            GtfmtDocEl::text("\"first\r\n  second\nthird\""),
            GtfmtDocEl::text(","),
            GtfmtDocEl::hard_line(),
            GtfmtDocEl::text("end"),
        ]));
        doc.extend([GtfmtDocEl::hard_line(), GtfmtDocEl::text("]")]);
        // Snapshots normalize line endings; use byte equality to check literal CRLF.
        assert_eq!(
            render(doc, 80),
            "[\n  \"first\r\n  second\nthird\",\n  end\n]"
        );
    }

    #[test]
    fn indivisible_text_can_exceed_width_without_losing_content() {
        let doc = GtfmtDocEl::group(
            vec![
                GtfmtDocEl::text("indivisible"),
                GtfmtDocEl::line(),
                GtfmtDocEl::text("next"),
            ],
            false,
        );
        assert_snapshot!(render(doc, 4), @"
        indivisible
        next
        ");
    }
}

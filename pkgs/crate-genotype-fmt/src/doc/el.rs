pub type GtfmtElements<'a> = Vec<GtfmtDocEl<'a>>;

#[derive(Debug, Clone)]
pub enum GtfmtDocEl<'a> {
    Text(&'a str),
    Space,
    Line(GtfmtDocElLine),
    Group(GtfmtDocElGroup<'a>),
    Indent(GtfmtElements<'a>),
    IfBreak(GtfmtElements<'a>),
}

impl<'a> GtfmtDocEl<'a> {
    pub fn text(value: &'a str) -> Self {
        Self::Text(value)
    }

    pub fn space() -> Self {
        Self::Space
    }

    pub fn line() -> Self {
        Self::Line(GtfmtDocElLine::SoftOrSpace)
    }

    pub fn soft_line() -> Self {
        Self::Line(GtfmtDocElLine::Soft)
    }

    pub fn hard_line() -> Self {
        Self::Line(GtfmtDocElLine::Hard)
    }

    pub fn empty_line() -> Self {
        Self::Line(GtfmtDocElLine::Empty)
    }

    pub fn group(content: GtfmtElements<'a>, expand: bool) -> GtfmtElements<'a> {
        let expand = expand || content.iter().any(Self::forces_break);
        vec![Self::Group(GtfmtDocElGroup { content, expand })]
    }

    pub fn indent(doc: GtfmtElements<'a>) -> GtfmtElements<'a> {
        vec![Self::Indent(doc)]
    }

    pub fn if_break(doc: GtfmtElements<'a>) -> GtfmtElements<'a> {
        vec![Self::IfBreak(doc)]
    }

    fn forces_break(&self) -> bool {
        match self {
            Self::Text(text) => text.contains('\n'),
            Self::Line(GtfmtDocElLine::Hard | GtfmtDocElLine::Empty) => true,
            Self::Group(group) => group.expand,
            Self::Indent(content) | Self::IfBreak(content) => {
                content.iter().any(Self::forces_break)
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GtfmtDocElLine {
    Soft,
    SoftOrSpace,
    Hard,
    Empty,
}

#[derive(Debug, Clone)]
pub struct GtfmtDocElGroup<'a> {
    pub content: GtfmtElements<'a>,
    pub expand: bool,
}

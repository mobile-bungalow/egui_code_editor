#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LintLevel {
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lint {
    pub level: LintLevel,
    pub line: usize,
    pub content: String,
}

impl Lint {
    pub fn new(level: LintLevel, line: usize, content: String) -> Self {
        Self {
            level,
            line,
            content,
        }
    }

    pub fn info(line: usize, content: String) -> Self {
        Self::new(LintLevel::Info, line, content)
    }

    pub fn warn(line: usize, content: String) -> Self {
        Self::new(LintLevel::Warn, line, content)
    }

    pub fn error(line: usize, content: String) -> Self {
        Self::new(LintLevel::Error, line, content)
    }
}

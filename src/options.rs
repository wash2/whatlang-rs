use crate::lang::Lang;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum List {
    White(Vec<Lang>),
    Black(Vec<Lang>),
}

/// Allows to customize behaviour of [Detector](struct.Detector.html).
#[derive(Debug, Clone, PartialEq)]
pub struct Options {
    pub(crate) list: Option<List>,
    pub(crate) confidence_threshold: f64,
}

impl Default for Options {
    fn default() -> Self { Options {list: None, confidence_threshold: 0.85,} }
}

impl Options {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_whitelist(mut self, whitelist: Vec<Lang>) -> Self {
        self.list = Some(List::White(whitelist));
        self
    }

    pub fn set_blacklist(mut self, blacklist: Vec<Lang>) -> Self {
        self.list = Some(List::Black(blacklist));
        self
    }

    pub fn set_confidence_threshold(mut self, threshold: f64) -> Self {
        self.confidence_threshold = threshold;
        self
    }
}

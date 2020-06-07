use crate::lang::Lang;
use crate::script::Script;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

const RELIABLE_CONFIDENCE_THRESHOLD: f64 = 0.8;

/// Represents a full outcome of language detection.
#[derive(Debug, Clone, Copy)]
pub struct Info {
    pub(crate) lang: Lang,
    pub(crate) script: Script,
    pub(crate) confidence: f64,
}

impl Info {
    pub fn lang(&self) -> Lang {
        self.lang
    }

    pub fn script(&self) -> Script {
        self.script
    }

    pub fn is_reliable(&self) -> bool {
        self.confidence > RELIABLE_CONFIDENCE_THRESHOLD
    }

    pub fn confidence(&self) -> f64 {
        self.confidence
    }
}

impl Hash for Info {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.lang.hash(hasher)
    }
}

impl PartialEq for Info {
    fn eq(&self, other: &Self) -> bool {
        self.lang == other.lang
    }
}

impl PartialOrd for Info {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.confidence.partial_cmp(&other.confidence)
    }
}
// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-hfp-hsp: hfp hsp component model.
//!
//! Real implementation: hsps, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hsp {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HfpHspError {
    EmptyHspName,
    DuplicateHsp(String),
    EmptyEventSubject,
}

impl fmt::Display for HfpHspError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyHspName => write!(f, "hsp name must not be empty"),
            Self::DuplicateHsp(id) => write!(f, "duplicate hsp {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for HfpHspError {}

pub struct HfpHsp {
    pub name: String,
    hsps: BTreeMap<String, Hsp>,
    events: BTreeMap<String, Event>,
}

impl HfpHsp {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            hsps: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_hsp(&mut self, item: Hsp) -> Result<(), HfpHspError> {
        if item.name.trim().is_empty() {
            return Err(HfpHspError::EmptyHspName);
        }
        let id = item.id.clone();
        if self.hsps.contains_key(&id) {
            return Err(HfpHspError::DuplicateHsp(id.clone()));
        }
        self.hsps.insert(id, item);
        Ok(())
    }

    pub fn hsp_count(&self) -> usize {
        self.hsps.len()
    }

    pub fn get_hsp(&self, id: &str) -> Option<&Hsp> {
        self.hsps.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), HfpHspError> {
        if event.subject.trim().is_empty() {
            return Err(HfpHspError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_hsp(id: &str, name: &str) -> Hsp {
        Hsp {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = HfpHsp::new("HfpHsp");
        assert_eq!(m.name, "HfpHsp");
        assert_eq!(m.hsp_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_hsp() {
        let mut m = HfpHsp::new("HfpHsp");
        m.add_hsp(sample_hsp("a1", "Alpha")).unwrap();
        assert_eq!(m.hsp_count(), 1);
        assert!(m.get_hsp("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = HfpHsp::new("HfpHsp");
        assert!(m.add_hsp(sample_hsp("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = HfpHsp::new("HfpHsp");
        m.add_hsp(sample_hsp("x", "One")).unwrap();
        assert!(m.add_hsp(sample_hsp("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = HfpHsp::new("HfpHsp");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

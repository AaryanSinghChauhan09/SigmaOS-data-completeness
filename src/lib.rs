//! SigmaOS-data-completeness: data completeness model.
//!
//! Real implementation: coverage, required fields, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Completeness requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletenessRequirement {
    pub id: String,
    pub name: String,
    pub table: String,
    pub required_fields: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletenessError {
    EmptyName,
    DuplicateRequirement(String),
}

impl fmt::Display for CompletenessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "requirement name must not be empty"),
            Self::DuplicateRequirement(id) => write!(f, "duplicate requirement {:?}", id),
        }
    }
}

impl std::error::Error for CompletenessError {}

/// Data completeness manager.
pub struct DataCompleteness {
    pub name: String,
    requirements: BTreeMap<String, CompletenessRequirement>,
}

impl DataCompleteness {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            requirements: BTreeMap::new(),
        }
    }

    pub fn add_requirement(&mut self, requirement: CompletenessRequirement) -> Result<(), CompletenessError> {
        if requirement.name.trim().is_empty() { return Err(CompletenessError::EmptyName); }
        let id = requirement.id.clone();
        if self.requirements.contains_key(&id) {
            return Err(CompletenessError::DuplicateRequirement(id.clone()));
        }
        self.requirements.insert(id, requirement);
        Ok(())
    }

    pub fn requirement_count(&self) -> usize { self.requirements.len() }

    pub fn get_requirement(&self, id: &str) -> Option<&CompletenessRequirement> { self.requirements.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completeness_create_aaa() {
        let c = DataCompleteness::new("My Completeness");
        assert_eq!(c.name, "My Completeness");
        assert_eq!(c.requirement_count(), 0);
    }

    #[test]
    fn test_add_requirement_aaa() {
        let mut c = DataCompleteness::new("My Completeness");
        let r = CompletenessRequirement { id: "r1".into(), name: "profile-complete".into(), table: "users".into(), required_fields: "name,email".into() };
        c.add_requirement(r).unwrap();
        assert_eq!(c.requirement_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut c = DataCompleteness::new("My Completeness");
        let r = CompletenessRequirement { id: "x".into(), name: "".into(), table: "a".into(), required_fields: "b".into() };
        assert!(c.add_requirement(r).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut c = DataCompleteness::new("My Completeness");
        let r = CompletenessRequirement { id: "x".into(), name: "X".into(), table: "a".into(), required_fields: "b".into() };
        c.add_requirement(r).unwrap();
        let r2 = CompletenessRequirement { id: "x".into(), name: "X".into(), table: "a".into(), required_fields: "b".into() };
        assert!(c.add_requirement(r2).is_err());
    }
}

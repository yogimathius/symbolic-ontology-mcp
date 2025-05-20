// Test modules for ontology-core
pub mod db;
pub mod domain;

// Import common test utilities
pub mod common {
    // Re-export test utilities from workspace common
    pub use crate::domain::fixtures;
}

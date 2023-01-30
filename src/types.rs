//! Implements the types used to identify the various sections of the Terraform files.

use std::fmt;

/// Holds the various entities to be exported to the documentation
#[derive(Debug)]
pub struct DocItem {
    /// The type of entity, ie. `comment`, `resource`, `output`, `variable`.
    pub category: BlockType,

    /// The name of the entity
    pub name: String,

    /// The `#` comments and/or `description` fields associated with the entity
    pub description: Vec<String>,

    /// The file in which the entity was found
    pub filename: String,
}

impl DocItem {
    /// Creates a new empty `DocItem` entity
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DocItem {
    /// Creates a default `DocItem`
    fn default() -> Self {
        Self {
            category: BlockType::None,
            name: String::new(),
            description: vec![],
            filename: String::new(),
        }
    }
}

impl fmt::Display for DocItem {
    /// Formats a `DocItem` for display
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.name.is_empty() {
            write!(f, "{}", self.description.join(" "))
        } else {
            write!(f, "`{}`: {}", self.name, self.description.join(" "))
        }
    }
}

/// Enumerates the types of blocks recognized.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BlockType {
    Comment,
    Data,
    Output,
    None,
    Resource,
    Variable,
}

impl fmt::Display for BlockType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = match self {
            Self::Comment => "Comment",
            Self::Data => "Data",
            Self::Output => "Output",
            Self::None => "None",
            Self::Resource => "Resource",
            Self::Variable => "Variable",
        };
        write!(f, "{t}")
    }
}

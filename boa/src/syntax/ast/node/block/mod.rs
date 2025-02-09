//! Block AST node.

use super::{Node, StatementList};
use crate::gc::{Finalize, Trace};
use std::{collections::HashSet, fmt};

#[cfg(feature = "deser")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// A `block` statement (or compound statement in other languages) is used to group zero or
/// more statements.
///
/// The block statement is often called compound statement in other languages.
/// It allows you to use multiple statements where JavaScript expects only one statement.
/// Combining statements into blocks is a common practice in JavaScript. The opposite behavior
/// is possible using an empty statement, where you provide no statement, although one is
/// required.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#prod-BlockStatement
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/block
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "deser", serde(transparent))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub struct Block {
    #[cfg_attr(feature = "deser", serde(flatten))]
    statements: StatementList,
}

impl Block {
    /// Gets the list of statements and declarations in this block.
    pub(crate) fn items(&self) -> &[Node] {
        self.statements.items()
    }

    pub(crate) fn lexically_declared_names(&self) -> HashSet<&str> {
        self.statements.lexically_declared_names()
    }

    pub(crate) fn var_declared_named(&self) -> HashSet<&str> {
        self.statements.var_declared_names()
    }

    /// Implements the display formatting with indentation.
    pub(super) fn display(&self, f: &mut fmt::Formatter<'_>, indentation: usize) -> fmt::Result {
        writeln!(f, "{{")?;
        self.statements.display(f, indentation + 1)?;
        write!(f, "{}}}", "    ".repeat(indentation))
    }
}

impl<T> From<T> for Block
where
    T: Into<StatementList>,
{
    fn from(list: T) -> Self {
        Self {
            statements: list.into(),
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl From<Block> for Node {
    fn from(block: Block) -> Self {
        Self::Block(block)
    }
}

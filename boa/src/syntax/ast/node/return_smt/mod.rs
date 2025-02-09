use crate::{
    gc::{Finalize, Trace},
    syntax::ast::node::Node,
};
use std::fmt;

#[cfg(feature = "deser")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// The `return` statement ends function execution and specifies a value to be returned to the
/// function caller.
///
/// Syntax: `return [expression];`
///
/// `expression`:
///  > The expression whose value is to be returned. If omitted, `undefined` is returned
///  > instead.
///
/// When a `return` statement is used in a function body, the execution of the function is
/// stopped. If specified, a given value is returned to the function caller.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#prod-ReturnStatement
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/return
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub struct Return {
    expr: Option<Box<Node>>,
    label: Option<Box<str>>,
}

impl Return {
    pub fn label(&self) -> Option<&str> {
        self.label.as_ref().map(Box::as_ref)
    }

    pub fn expr(&self) -> Option<&Node> {
        self.expr.as_ref().map(Box::as_ref)
    }

    /// Creates a `Return` AST node.
    pub fn new<E, OE, L>(expr: OE, label: L) -> Self
    where
        E: Into<Node>,
        OE: Into<Option<E>>,
        L: Into<Option<Box<str>>>,
    {
        Self {
            expr: expr.into().map(E::into).map(Box::new),
            label: label.into(),
        }
    }
}

impl From<Return> for Node {
    fn from(return_smt: Return) -> Node {
        Node::Return(return_smt)
    }
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.expr() {
            Some(ex) => write!(f, "return {}", ex),
            None => write!(f, "return"),
        }
    }
}

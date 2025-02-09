//! Declaration nodes
use crate::{
    gc::{Finalize, Trace},
    syntax::ast::node::{join_nodes, Identifier, Node},
};
use std::fmt;

#[cfg(feature = "deser")]
use serde::{Deserialize, Serialize};

pub mod arrow_function_decl;
pub mod async_function_decl;
pub mod async_function_expr;
pub mod async_generator_decl;
pub mod async_generator_expr;
pub mod function_decl;
pub mod function_expr;
pub mod generator_decl;
pub mod generator_expr;

pub use self::{
    arrow_function_decl::ArrowFunctionDecl, async_function_decl::AsyncFunctionDecl,
    async_function_expr::AsyncFunctionExpr, async_generator_decl::AsyncGeneratorDecl,
    async_generator_expr::AsyncGeneratorExpr, function_decl::FunctionDecl,
    function_expr::FunctionExpr,
};

#[cfg(test)]
mod tests;

#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub enum DeclarationList {
    /// The `const` statements are block-scoped, much like variables defined using the `let`
    /// keyword.
    ///
    /// This declaration creates a constant whose scope can be either global or local to the block
    /// in which it is declared. Global constants do not become properties of the window object,
    /// unlike var variables.
    ///
    /// An initializer for a constant is required. You must specify its value in the same statement
    /// in which it's declared. (This makes sense, given that it can't be changed later.)
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-let-and-const-declarations
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/const
    /// [identifier]: https://developer.mozilla.org/en-US/docs/Glossary/identifier
    /// [expression]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Expressions_and_Operators#Expressions
    Const(Box<[Declaration]>),

    /// The `let` statement declares a block scope local variable, optionally initializing it to a
    /// value.
    ///
    ///
    /// `let` allows you to declare variables that are limited to a scope of a block statement, or
    /// expression on which it is used, unlike the `var` keyword, which defines a variable
    /// globally, or locally to an entire function regardless of block scope.
    ///
    /// Just like const the `let` does not create properties of the window object when declared
    /// globally (in the top-most scope).
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-let-and-const-declarations
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/let
    Let(Box<[Declaration]>),

    /// The `var` statement declares a variable, optionally initializing it to a value.
    ///
    /// var declarations, wherever they occur, are processed before any code is executed. This is
    /// called hoisting, and is discussed further below.
    ///
    /// The scope of a variable declared with var is its current execution context, which is either
    /// the enclosing function or, for variables declared outside any function, global. If you
    /// re-declare a JavaScript variable, it will not lose its value.
    ///
    /// Assigning a value to an undeclared variable implicitly creates it as a global variable (it
    /// becomes a property of the global object) when the assignment is executed.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#prod-VariableStatement
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/var
    Var(Box<[Declaration]>),
}

impl AsRef<[Declaration]> for DeclarationList {
    fn as_ref(&self) -> &[Declaration] {
        use DeclarationList::*;
        match self {
            Var(list) | Const(list) | Let(list) => list,
        }
    }
}

impl fmt::Display for DeclarationList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.as_ref().is_empty() {
            use DeclarationList::*;
            match &self {
                Let(_) => write!(f, "let ")?,
                Const(_) => write!(f, "const ")?,
                Var(_) => write!(f, "var ")?,
            }
            join_nodes(f, self.as_ref())
        } else {
            Ok(())
        }
    }
}

impl From<DeclarationList> for Node {
    fn from(list: DeclarationList) -> Self {
        use DeclarationList::*;
        match &list {
            Let(_) => Node::LetDeclList(list),
            Const(_) => Node::ConstDeclList(list),
            Var(_) => Node::VarDeclList(list),
        }
    }
}

impl From<Declaration> for Box<[Declaration]> {
    fn from(d: Declaration) -> Self {
        Box::new([d])
    }
}

/// Declaration represents either an individual binding or a binding pattern.
///
/// For `let` and `const` declarations this type represents a [LexicalBinding][spec1]
///
/// For `var` declarations this type represents a [VariableDeclaration][spec2]
///
/// More information:
///  - [ECMAScript reference: 14.3 Declarations and the Variable Statement][spec3]
///
/// [spec1]: https://tc39.es/ecma262/#prod-LexicalBinding
/// [spec2]: https://tc39.es/ecma262/#prod-VariableDeclaration
/// [spec3]:  https://tc39.es/ecma262/#sec-declarations-and-the-variable-statement
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub enum Declaration {
    Identifier {
        ident: Identifier,
        init: Option<Node>,
    },
    Pattern(DeclarationPattern),
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Identifier { ident, init } => {
                fmt::Display::fmt(&ident, f)?;
                if let Some(ref init) = &init {
                    write!(f, " = {}", init)?;
                }
            }
            Self::Pattern(pattern) => {
                fmt::Display::fmt(&pattern, f)?;
            }
        }
        Ok(())
    }
}

impl Declaration {
    /// Creates a new variable declaration with a BindingIdentifier.
    #[inline]
    pub(in crate::syntax) fn new_with_identifier<N, I>(ident: N, init: I) -> Self
    where
        N: Into<Identifier>,
        I: Into<Option<Node>>,
    {
        Self::Identifier {
            ident: ident.into(),
            init: init.into(),
        }
    }

    /// Creates a new variable declaration with an ObjectBindingPattern.
    #[inline]
    pub(in crate::syntax) fn new_with_object_pattern<I>(
        bindings: Vec<BindingPatternTypeObject>,
        init: I,
    ) -> Self
    where
        I: Into<Option<Node>>,
    {
        Self::Pattern(DeclarationPattern::Object(DeclarationPatternObject::new(
            bindings,
            init.into(),
        )))
    }

    /// Creates a new variable declaration with an ArrayBindingPattern.
    #[inline]
    pub(in crate::syntax) fn new_with_array_pattern<I>(
        bindings: Vec<BindingPatternTypeArray>,
        init: I,
    ) -> Self
    where
        I: Into<Option<Node>>,
    {
        Self::Pattern(DeclarationPattern::Array(DeclarationPatternArray::new(
            bindings,
            init.into(),
        )))
    }

    /// Gets the initialization node for the declaration, if any.
    #[inline]
    pub(crate) fn init(&self) -> Option<&Node> {
        match &self {
            Self::Identifier { init, .. } => init.as_ref(),
            Self::Pattern(pattern) => pattern.init(),
        }
    }
}

/// DeclarationPattern represents an object or array binding pattern.
///
/// This enum mostly wraps the functionality of the specific binding pattern types.
///
/// More information:
///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - BindingPattern][spec1]
///
/// [spec1]: https://tc39.es/ecma262/#prod-BindingPattern
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub enum DeclarationPattern {
    Object(DeclarationPatternObject),
    Array(DeclarationPatternArray),
}

impl fmt::Display for DeclarationPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DeclarationPattern::Object(o) => {
                fmt::Display::fmt(o, f)?;
            }
            DeclarationPattern::Array(a) => {
                fmt::Display::fmt(a, f)?;
            }
        }
        Ok(())
    }
}

impl DeclarationPattern {
    /// Gets the list of identifiers declared by the binding pattern.
    ///
    /// A single binding pattern may declare 0 to n identifiers.
    #[inline]
    pub fn idents(&self) -> Vec<&str> {
        match &self {
            DeclarationPattern::Object(pattern) => pattern.idents(),
            DeclarationPattern::Array(pattern) => pattern.idents(),
        }
    }

    /// Gets the initialization node for the binding pattern, if any.
    #[inline]
    pub fn init(&self) -> Option<&Node> {
        match &self {
            DeclarationPattern::Object(pattern) => pattern.init(),
            DeclarationPattern::Array(pattern) => pattern.init(),
        }
    }
}

/// DeclarationPatternObject represents an object binding pattern.
///
/// This struct holds a list of bindings, and an optional initializer for the binding pattern.
///
/// More information:
///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - ObjectBindingPattern][spec1]
///
/// [spec1]: https://tc39.es/ecma262/#prod-ObjectBindingPattern
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub struct DeclarationPatternObject {
    bindings: Vec<BindingPatternTypeObject>,
    init: Option<Node>,
}

impl fmt::Display for DeclarationPatternObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt("{", f)?;
        for (i, binding) in self.bindings.iter().enumerate() {
            if i == self.bindings.len() - 1 {
                write!(f, "{} ", binding)?;
            } else {
                write!(f, "{},", binding)?;
            }
        }
        fmt::Display::fmt("}", f)?;
        if let Some(ref init) = self.init {
            write!(f, " = {}", init)?;
        }
        Ok(())
    }
}

impl DeclarationPatternObject {
    /// Create a new object binding pattern.
    #[inline]
    pub(in crate::syntax) fn new(
        bindings: Vec<BindingPatternTypeObject>,
        init: Option<Node>,
    ) -> Self {
        Self { bindings, init }
    }

    /// Gets the initialization node for the object binding pattern, if any.
    #[inline]
    pub(crate) fn init(&self) -> Option<&Node> {
        self.init.as_ref()
    }

    /// Gets the bindings for the object binding pattern.
    #[inline]
    pub(crate) fn bindings(&self) -> &Vec<BindingPatternTypeObject> {
        &self.bindings
    }

    /// Gets the list of identifiers declared by the object binding pattern.
    #[inline]
    pub(crate) fn idents(&self) -> Vec<&str> {
        let mut idents = Vec::new();

        for binding in &self.bindings {
            use BindingPatternTypeObject::*;

            match binding {
                Empty => {}
                SingleName {
                    ident,
                    property_name: _,
                    default_init: _,
                } => {
                    idents.push(ident.as_ref());
                }
                RestProperty {
                    ident: property_name,
                    excluded_keys: _,
                } => {
                    idents.push(property_name.as_ref());
                }
                BindingPattern {
                    ident: _,
                    pattern,
                    default_init: _,
                } => {
                    for ident in pattern.idents() {
                        idents.push(ident);
                    }
                }
            }
        }

        idents
    }
}

/// DeclarationPatternArray represents an array binding pattern.
///
/// This struct holds a list of bindings, and an optional initializer for the binding pattern.
///
/// More information:
///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - ArrayBindingPattern][spec1]
///
/// [spec1]: https://tc39.es/ecma262/#prod-ArrayBindingPattern
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub struct DeclarationPatternArray {
    bindings: Vec<BindingPatternTypeArray>,
    init: Option<Node>,
}

impl fmt::Display for DeclarationPatternArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt("[", f)?;
        for (i, binding) in self.bindings.iter().enumerate() {
            if i == self.bindings.len() - 1 {
                match binding {
                    BindingPatternTypeArray::Elision => write!(f, "{}, ", binding)?,
                    _ => write!(f, "{} ", binding)?,
                }
            } else {
                write!(f, "{},", binding)?;
            }
        }
        fmt::Display::fmt("]", f)?;
        if let Some(ref init) = self.init {
            write!(f, " = {}", init)?;
        }
        Ok(())
    }
}

impl DeclarationPatternArray {
    /// Create a new array binding pattern.
    #[inline]
    pub(in crate::syntax) fn new(
        bindings: Vec<BindingPatternTypeArray>,
        init: Option<Node>,
    ) -> Self {
        Self { bindings, init }
    }

    /// Gets the initialization node for the array binding pattern, if any.
    #[inline]
    pub(crate) fn init(&self) -> Option<&Node> {
        self.init.as_ref()
    }

    /// Gets the bindings for the array binding pattern.
    #[inline]
    pub(crate) fn bindings(&self) -> &Vec<BindingPatternTypeArray> {
        &self.bindings
    }

    /// Gets the list of identifiers declared by the array binding pattern.
    #[inline]
    pub(crate) fn idents(&self) -> Vec<&str> {
        let mut idents = Vec::new();

        for binding in &self.bindings {
            use BindingPatternTypeArray::*;

            match binding {
                Empty => {}
                Elision => {}
                SingleName {
                    ident,
                    default_init: _,
                } => {
                    idents.push(ident.as_ref());
                }
                BindingPattern { pattern } | BindingPatternRest { pattern } => {
                    let mut i = pattern.idents();
                    idents.append(&mut i)
                }
                SingleNameRest { ident } => idents.push(ident),
            }
        }

        idents
    }
}

/// BindingPatternTypeObject represents the different types of bindings that an object binding pattern may contain.
///
/// More information:
///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - ObjectBindingPattern][spec1]
///
/// [spec1]: https://tc39.es/ecma262/#prod-ObjectBindingPattern
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub enum BindingPatternTypeObject {
    /// Empty represents an empty object binding pattern e.g. `{ }`.
    Empty,

    /// SingleName represents one of the following properties:
    ///
    /// - `SingleNameBinding` with an identifier and an optional default initializer.
    /// - `BindingProperty` with an property name and a `SingleNameBinding` as  the `BindingElement`.
    ///
    /// More information:
    ///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - SingleNameBinding][spec1]
    ///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - BindingProperty][spec2]
    ///
    /// [spec1]: https://tc39.es/ecma262/#prod-SingleNameBinding
    /// [spec2]: https://tc39.es/ecma262/#prod-BindingProperty
    SingleName {
        ident: Box<str>,
        property_name: Box<str>,
        default_init: Option<Node>,
    },

    /// RestProperty represents a `BindingRestProperty` with an identifier.
    ///
    /// It also includes a list of the property keys that should be excluded from the rest,
    /// because they where already assigned.
    ///
    /// More information:
    ///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - BindingRestProperty][spec1]
    ///
    /// [spec1]: https://tc39.es/ecma262/#prod-BindingRestProperty
    RestProperty {
        ident: Box<str>,
        excluded_keys: Vec<Box<str>>,
    },

    /// BindingPattern represents a `BindingProperty` with a `BindingPattern` as the `BindingElement`.
    ///
    /// Additionally to the identifier of the new property and the nested binding pattern,
    /// this may also include an optional default initializer.
    ///
    /// More information:
    ///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - BindingProperty][spec1]
    ///
    /// [spec1]: https://tc39.es/ecma262/#prod-BindingProperty
    BindingPattern {
        ident: Box<str>,
        pattern: DeclarationPattern,
        default_init: Option<Node>,
    },
}

impl fmt::Display for BindingPatternTypeObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BindingPatternTypeObject::Empty => {}
            BindingPatternTypeObject::SingleName {
                ident,
                property_name,
                default_init,
            } => {
                if ident == property_name {
                    write!(f, " {}", ident)?;
                } else {
                    write!(f, " {} : {}", property_name, ident)?;
                }
                if let Some(ref init) = default_init {
                    write!(f, " = {}", init)?;
                }
            }
            BindingPatternTypeObject::RestProperty {
                ident: property_name,
                excluded_keys: _,
            } => {
                write!(f, " ... {}", property_name)?;
            }
            BindingPatternTypeObject::BindingPattern {
                ident: property_name,
                pattern,
                default_init,
            } => {
                write!(f, " {} : {}", property_name, pattern)?;
                if let Some(ref init) = default_init {
                    write!(f, " = {}", init)?;
                }
            }
        }
        Ok(())
    }
}

/// BindingPatternTypeArray represents the different types of bindings that an array binding pattern may contain.
///
/// More information:
///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - ArrayBindingPattern][spec1]
///
/// [spec1]: https://tc39.es/ecma262/#prod-ArrayBindingPattern
#[cfg_attr(feature = "deser", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub enum BindingPatternTypeArray {
    /// Empty represents an empty array binding pattern e.g. `[ ]`.
    ///
    /// This may occur because the `Elision` and `BindingRestElement` in the first type of
    /// array binding pattern are both optional.
    ///
    /// More information:
    ///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - ArrayBindingPattern][spec1]
    ///
    /// [spec1]: https://tc39.es/ecma262/#prod-ArrayBindingPattern
    Empty,

    /// Elision represents the elision of an item in the array binding pattern.
    ///
    /// An `Elision` may occur at multiple points in the pattern and may be multiple elisions.
    /// This variant strictly represents one elision. If there are multiple, this should be used multiple times.
    ///
    /// More information:
    ///  - [ECMAScript reference: 13.2.4 Array Initializer - Elision][spec1]
    ///
    /// [spec1]: https://tc39.es/ecma262/#prod-Elision
    Elision,

    /// SingleName represents a `SingleNameBinding` with an identifier and an optional default initializer.
    ///
    /// More information:
    ///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - SingleNameBinding][spec1]
    ///
    /// [spec1]: https://tc39.es/ecma262/#prod-SingleNameBinding
    SingleName {
        ident: Box<str>,
        default_init: Option<Node>,
    },

    /// BindingPattern represents a `BindingPattern` in a `BindingElement` of an array binding pattern.
    ///
    /// The pattern and the optional default initializer are both stored in the DeclarationPattern.
    ///
    /// More information:
    ///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - BindingElement][spec1]
    ///
    /// [spec1]: https://tc39.es/ecma262/#prod-BindingElement
    BindingPattern { pattern: DeclarationPattern },

    /// SingleNameRest represents a `BindingIdentifier` in a `BindingRestElement` of an array binding pattern.
    ///
    /// More information:
    ///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - BindingRestElement][spec1]
    ///
    /// [spec1]: https://tc39.es/ecma262/#prod-BindingRestElement
    SingleNameRest { ident: Box<str> },

    /// SingleNameRest represents a `BindingPattern` in a `BindingRestElement` of an array binding pattern.
    ///
    /// More information:
    ///  - [ECMAScript reference: 14.3.3 Destructuring Binding Patterns - BindingRestElement][spec1]
    ///
    /// [spec1]: https://tc39.es/ecma262/#prod-BindingRestElement
    BindingPatternRest { pattern: DeclarationPattern },
}

impl fmt::Display for BindingPatternTypeArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BindingPatternTypeArray::Empty => {}
            BindingPatternTypeArray::Elision => {
                fmt::Display::fmt(" ", f)?;
            }
            BindingPatternTypeArray::SingleName {
                ident,
                default_init,
            } => {
                write!(f, " {}", ident)?;
                if let Some(ref init) = default_init {
                    write!(f, " = {}", init)?;
                }
            }
            BindingPatternTypeArray::BindingPattern { pattern } => {
                write!(f, " {}", pattern)?;
            }
            BindingPatternTypeArray::SingleNameRest { ident } => {
                write!(f, " ... {}", ident)?;
            }
            BindingPatternTypeArray::BindingPatternRest { pattern } => {
                write!(f, " ... {}", pattern)?;
            }
        }
        Ok(())
    }
}

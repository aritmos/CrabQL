#![allow(unused)]

// --- READER ---

pub struct Reader;

impl Reader {
    pub fn filter(self, expr: impl Boolean + 'static) -> Self {
        let expr: Box<dyn Boolean> = Box::new(expr);
        self._filter(expr)
    }

    fn _filter(self, _e: Box<dyn Boolean>) -> Self {
        self
    }

    pub fn select_all(self) -> Self {
        self
    }
}

// --- EXPR ---

// A general expression defines any set of internal conditions that need to be
// checked by an external source (a `Checker`).
pub trait Expression {}

// These subtraits are implemented for expressions which have the respective certain return type
pub trait Textual: Expression {}
pub trait Boolean: Expression {}
pub trait Numeric: Expression {}

pub trait Miscellaneous: Expression {}

pub trait Anything: Textual + Boolean + Numeric {}

impl<'t, T: Boolean + 't> From<T> for Box<dyn Boolean + 't> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

// --- TYPES ---

// Literal usizes as `Numeric` expressions
impl Expression for usize {}
impl Numeric for usize {}

// Literal Strings as `Textual` expressions
impl Expression for String {}
impl Textual for String {}

// Literal booleans as `Boolean` expressions
impl Expression for bool {}
impl Boolean for bool {}

// Columns as `Anything`
struct Column {
    name: String,
}
impl Expression for Column {}
impl Textual for Column {}
impl Numeric for Column {}
impl Boolean for Column {}
impl Anything for Column {}

// No need for `Table`s, this is a clean approach with less indirection.
fn col(name: impl Into<String>) -> Column {
    Column { name: name.into() }
}

// Every type will implement its operand functionality individually.
// In the case of `Anything`s we need to be careful about how we distinguish
// `Add<R: Textual>` from `Add<R: Numeric>` and so on.
// Literal types should work well with this approach as their default
// implementation makes sense in the expression's context.
impl<R: Boolean + 'static> std::ops::BitAnd<R> for Column {
    type Output = AndExpr;

    fn bitand(self, rhs: R) -> Self::Output {
        And::and(self, rhs)
    }
}

// `LENGTH(textual)` expression
pub struct LenExpr {
    inner: Box<dyn Textual>,
}
impl Expression for LenExpr {}
impl Numeric for LenExpr {}

#[allow(clippy::len_without_is_empty)]
pub trait Len {
    fn len(self) -> LenExpr;
}

impl<T: Textual + 'static> Len for T {
    fn len(self) -> LenExpr {
        LenExpr {
            inner: Box::new(self),
        }
    }
}

// `textual || textual` expression
pub struct StrConcatExpr {
    lhs: Box<dyn Textual>,
    rhs: Box<dyn Textual>,
}
impl Expression for StrConcatExpr {}
impl Textual for StrConcatExpr {}

// `numeric > numeric` expression
pub struct GTExpr<'t> {
    lhs: Box<dyn Numeric + 't>,
    rhs: Box<dyn Numeric + 't>,
}
impl<'t> Expression for GTExpr<'t> {}
impl<'t> Boolean for GTExpr<'t> {}

pub trait GT<'t, R> {
    fn gt(self, rhs: R) -> GTExpr<'t>
    where
        Self: Sized;
}

impl<'t, L: Numeric + 't, R: Numeric + 't> GT<'t, R> for L {
    fn gt(self, rhs: R) -> GTExpr<'t> {
        GTExpr {
            lhs: Box::new(self),
            rhs: Box::new(rhs),
        }
    }
}

// `boolean AND boolean` expression
pub struct AndExpr {
    lhs: Box<dyn Boolean>,
    rhs: Box<dyn Boolean>,
}
impl Expression for AndExpr {}
impl Boolean for AndExpr {}

pub trait And<R>: Boolean {
    fn and(self, rhs: R) -> AndExpr;
}
impl<L: Boolean + 'static, R: Boolean + 'static> And<R> for L {
    fn and(self, rhs: R) -> AndExpr {
        AndExpr {
            lhs: Box::new(self),
            rhs: Box::new(rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let _r = Reader
            .filter({
                let is_adult = col("age").gt(17);
                col("employeed") & is_adult
            })
            .select_all();
    }
}

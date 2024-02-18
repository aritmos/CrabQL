#![allow(unused)]

type Expr = Box<dyn Expression>;

type CheckExpr = String;

// Expression evaluation types that need to be enforced by a checker.
pub enum ExprType {
    Numeric,
    Textual,
    Boolean,
    Any, // used as a NULLOP condition
}

// The checker takes an iterator of expressions and evaluates them,
// stopping when it encounters an error (?) or perhaps accumulating all errors
// assuming that any error propagation in constructions can be mitigated.
pub trait Checker {
    fn check(&mut self, conditions: impl IntoIterator<Item = CheckExpr>) -> Result<(), String>;
}

// A general expression defines any set of internal conditions that need to be
// checked by an external source (a `Checker`).
pub trait Expression {
    fn conditions(&self, ctx: ExprType) -> Box<dyn Iterator<Item = CheckExpr> + '_>;
}

// These subtraits are implemented for expressions which have the respective certain return type
pub trait Textual: Expression {}
pub type Text = Box<dyn Textual>;

pub trait Boolean: Expression {}
pub type Bool = Box<dyn Boolean>;

pub trait Numeric: Expression {}
pub type Num = Box<dyn Numeric>;

// `Misc` is for unique expression returns.
// e.g. an ordering expression: `t["name"].asc()`
pub trait Miscellaneous: Expression {}
pub type Misc = Box<dyn Miscellaneous>;

// `Any` is used for expressions which don't have a fixed return type.
// e.g. columns and CASE expressions.
pub trait Anything: Textual + Boolean + Numeric {}
pub type Any = Box<dyn Anything>;

pub struct LitExpr<V> {
    val: V,
}
impl<V> Expression for LitExpr<V> {
    fn conditions(&self, _: ExprType) -> Box<dyn Iterator<Item = CheckExpr>> {
        // literary expressions have no inner conditions to check
        Box::new(std::iter::empty())
    }
}
impl Textual for LitExpr<String> {}
impl Numeric for LitExpr<usize> {}

impl From<usize> for Num {
    fn from(val: usize) -> Self {
        Box::new(LitExpr { val })
    }
}

struct StrConcatExpr {
    lhs: Text,
    rhs: Text,
}
impl Expression for StrConcatExpr {
    // exprtype not needed as `Self` is strictly `Textual`
    fn conditions(&self, _: ExprType) -> Box<dyn Iterator<Item = CheckExpr> + '_> {
        Box::new(
            [&self.lhs, &self.rhs]
                .into_iter()
                .flat_map(|elem| elem.conditions(ExprType::Textual)),
        )
    }
}
impl Textual for StrConcatExpr {}

impl std::ops::Add for Box<dyn Textual> {
    type Output = Box<dyn Textual>;

    fn add(self, rhs: Self) -> Self::Output {
        Box::new(StrConcatExpr { lhs: self, rhs })
    }
}

struct LenExpr {
    inner: Text,
}
impl Expression for LenExpr {
    // exprtype not needed as `Self` is strictly `Textual`
    fn conditions(&self, _: ExprType) -> Box<dyn Iterator<Item = CheckExpr> + '_> {
        // inner type must be textual
        self.inner.conditions(ExprType::Textual)
    }
}

struct Column {
    name: String,
}
impl Expression for Column {
    fn conditions(&self, ctx: ExprType) -> Box<dyn Iterator<Item = CheckExpr>> {
        let exist = format!("Column `{}` must exist in context", self.name);

        // debug printing to test
        let kind = match ctx {
            ExprType::Numeric => Some("numeric"),
            ExprType::Textual => Some("textual"),
            ExprType::Boolean => Some("boolean"),
            _ => None,
        };
        if let Some(kind) = kind {
            let kind_condition = format!("Column `{}` must be {}", self.name, kind);
            Box::new([exist, kind_condition].into_iter())
        } else {
            Box::new(std::iter::once(exist))
        }
    }
}
impl Textual for Column {}
impl Numeric for Column {}
impl Boolean for Column {}
impl Anything for Column {}

pub struct GTExpr {
    lhs: Num,
    rhs: Num,
}
impl Expression for GTExpr {
    fn conditions(&self, ctx: ExprType) -> Box<dyn Iterator<Item = CheckExpr> + '_> {
        Box::new(
            [&self.lhs, &self.rhs]
                .into_iter()
                .flat_map(|x| x.conditions(ExprType::Numeric)),
        )
    }
}
impl Boolean for GTExpr {}

pub trait GT {
    fn gt(self, rhs: impl Into<Num>) -> Bool;
}

impl GT for Num {
    fn gt(self, rhs: impl Into<Num>) -> Bool {
        Box::new(GTExpr {
            lhs: self,
            rhs: rhs.into(),
        })
    }
}

impl GT for Any {
    fn gt(self, rhs: impl Into<Num>) -> Bool {
        Box::new(GTExpr {
            lhs: self,
            rhs: rhs.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_upcasting() {
        let col: Box<dyn Anything> = Box::new(Column {
            name: "fname".to_owned(),
        });

        let txt1: Box<dyn Textual> = Box::new(LitExpr {
            val: "hello".to_owned(),
        });

        let txt2: Box<dyn Textual> = Box::new(LitExpr {
            val: "hello".to_owned(),
        });

        let str_concat = txt1 + col + txt2;

        // manual casting is required if the Any type comes first
        // let str_concat = (col as Text) + txt2;

        // ExprType not needed as StrConcatExpr is Textual
        for cond in str_concat.conditions(ExprType::Any) {
            println!("CONDITION: {}", cond);
        }
    }

    #[test]
    fn col_condition() {
        let col = Column {
            name: "test".to_owned(),
        };
        for cond in col.conditions(ExprType::Any) {
            println!("CONDITION: {}", cond);
        }
    }

    #[test]
    fn test_gt() {
        let col: Box<dyn Anything> = Box::new(Column {
            name: "id".to_owned(),
        });

        // let _ = 3.as_expr().gt(5);
        let _ = col.gt(3);
    }
}

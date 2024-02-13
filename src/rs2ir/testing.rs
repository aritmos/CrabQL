#![allow(unused)]

type CheckExpr = String;

// these signify any comptime-known expression "returns" that might need to be enforced by a
// checker
enum ExprType {
    Numeric,
    Textual,
    Boolean,
    Any, // used as a NULLOP condition
}

// these subtraits are only implemented for types with the respective comptime-known returns.
trait Textual: Expr {}
trait Boolean: Expr {}
trait Numeric: Expr {}

// the checker takes an iterator of expressions and evaluates them,
// stopping when it encounters an error (?) or perhaps accumulating all errors
// assuming that any error propagation in constructions can be mitigated.
trait Checker {
    fn check(&mut self, conditions: impl IntoIterator<Item = CheckExpr>) -> Result<(), String>;
}

// a general expression defines any set of internal conditions that need to be checked via a checker.
trait Expr {
    fn conditions(&self, ctx: ExprType) -> Box<dyn Iterator<Item = CheckExpr>>;
}

struct LitExpr<V> {
    val: V,
}
impl<V> Expr for LitExpr<V> {
    fn conditions(&self, _: ExprType) -> Box<dyn Iterator<Item = CheckExpr>> {
        // literary expressions have no inner conditions to check
        Box::new(std::iter::empty())
    }
}
impl Textual for LitExpr<String> {}

struct StrConcatExpr {
    vals: Vec<Box<dyn Textual>>,
}
impl Expr for StrConcatExpr {
    // exprtype not needed as `Self` is strictly `Textual`
    fn conditions(&self, _: ExprType) -> Box<dyn Iterator<Item = CheckExpr>> {
        let iterators: Vec<_> = self
            .vals
            .iter()
            .map(|elem| elem.conditions(ExprType::Textual)) // all inner vals must be textual
            .collect();
        Box::new(iterators.into_iter().flatten())
    }
}

struct LenExpr {
    inner: Box<dyn Textual>,
}
impl Expr for LenExpr {
    // exprtype not needed as `Self` is strictly `Textual`
    fn conditions(&self, _: ExprType) -> Box<dyn Iterator<Item = CheckExpr>> {
        // inner type must be textual
        self.inner.conditions(ExprType::Textual)
    }
}

struct Column {
    name: String,
}
impl Expr for Column {
    fn conditions(&self, ctx: ExprType) -> Box<dyn Iterator<Item = CheckExpr>> {
        let exist = format!("Column `{}` must exist in context", self.name);
        let kind = match ctx {
            ExprType::Numeric => "numeric",
            ExprType::Textual => "textual",
            ExprType::Boolean => "boolean",
            _ => "",
        };
        if !kind.is_empty() {
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

// there are expressions which have a comptime known expression type, e.g.:
// - LenExpr:       LENGTH(col)            => Numeric
// - StrConcatExpr: col || 'stuff'         => Textual
// - AndExpr:       col1 > 3 AND col2 = 2  => Boolean
// there are also expressions which dont have strict expression types, and can be of different
// types depend on specific cases e.g.:
// - Column:   col           => Any
// - CaseExpr: CASE ... END  => Any
// the "runtime" expressions need to have a context used to create/validate their inner
// representation. this is currently modelled by providing a context (`ctx`) to
// `Expr::conditions`

struct CaseExpr {
    branches: Vec<CaseBranchExpr>,
}
impl Expr for CaseExpr {
    fn conditions(&self, ctx: ExprType) -> Box<dyn Iterator<Item = CheckExpr>> {
        // Conditions:
        // - All of the values inside of case branches are of the same type,
        // do this by comparing B1 to B2, B2 to B3, etc.
        //
        todo!()
    }
}
impl Textual for CaseExpr {}
impl Numeric for CaseExpr {}
impl Boolean for CaseExpr {}

struct CaseBranchExpr {
    condition: Box<dyn Boolean>,
    value: Box<dyn Expr>,
}
impl Expr for CaseBranchExpr {
    fn conditions(&self, ctx: ExprType) -> Box<dyn Iterator<Item = CheckExpr>> {
        // Condition:
        // - The condition is Boolean
        todo!()
    }
}

// TODO:
// There is still the very large gap of how we pass off these objects into builders.
// Especially given all of the type erasure due to using trait objects within all of the
// expressions.
//
// We somehow need:
// - The concrete types given by generics (to add to the builder's state)
// - The dynamic nature of trait objects (used to have collections of different underlying types)
//
// It might be possible to get around the generics by extending the `Expr` trait to hold a method
// that creates the object needed for the builder. In a sense a "return the type & state of self"
// method that is called recursively. Although this would "clone" the entire expression into a
// type that should essentially not be any different from self.
// There is no easy way out by pre-constructing the SQL string (by having this method return a String)
// due to all of the post-processing logic that needs to be handled when sealing (such as virtual columns).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let str_concat = StrConcatExpr {
            vals: vec![
                Box::new(LitExpr {
                    val: String::from("hello"),
                }),
                Box::new(Column {
                    name: "id".to_owned(),
                }),
            ],
        };
        for cond in str_concat.conditions(ExprType::Any) {
            println!("CONDITION: {}", cond);
        }
        println!("---");
        let col = Column {
            name: "test".to_owned(),
        };
        for cond in col.conditions(ExprType::Any) {
            println!("CONDITION: {}", cond);
        }
    }
}

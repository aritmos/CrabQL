use super::{
    expr::{BaseExpr, BoolExpr, BoolOp::*, Expr, ExprErr, ExprResult, FuncExpr},
    value::Value,
};

/// Functions that can be applied to expressions
pub enum Functions {
    Len,
    Count,
    And,
}

// each of the methods here needs to have a 1-1 relation with an `Expr` subtype (not ideal)
//
// the `len()` method does not measure the length of the collection
#[allow(clippy::len_without_is_empty)]
pub trait Mappings: Into<ExprResult> {
    /// Returns the length of the expression
    fn len(self) -> ExprResult {
        let expr_res: ExprResult = self.into();
        if expr_res.error.is_some() {
            return expr_res;
        }

        if let Some(error) = match expr_res.expr {
            Expr::Bool(_) => Some("Attempt to apply LEN() to a boolean expression".to_owned()),
            Expr::Base(BaseExpr::Lit(ref val)) if !matches!(val, Value::Str(_)) => Some(
                "Attempt to apply `LEN()` function to a literal value not of type String"
                    .to_owned(),
            ),
            _ => None,
        } {
            let error = Some(ExprErr(error));
            return ExprResult { error, ..expr_res };
        }

        let expr = Expr::Func(FuncExpr {
            func: Functions::Len,
            args: vec![expr_res.expr],
        });

        ExprResult { expr, error: None }
    }

    fn count(self) -> ExprResult {
        let mut expr_res: ExprResult = self.into();

        if expr_res.error.is_some() {
            return expr_res;
        }

        if let Some(error) = match expr_res.expr {
            Expr::Base(BaseExpr::Col(_)) => None,
            _ => Some("Attempt to count rows of a non column type".to_string()),
        } {
            let error = Some(ExprErr(error));
            return ExprResult { error, ..expr_res };
        }

        expr_res.expr = Expr::Func(FuncExpr {
            func: Functions::Count,
            args: vec![expr_res.expr],
        });

        expr_res
    }

    fn and(self, rhs: impl Into<ExprResult>) -> ExprResult {
        let mut expr_res: ExprResult = self.into();
        let rhs: ExprResult = rhs.into();

        // Return any existing errors
        if expr_res.error.is_some() {
            return expr_res;
        }
        if rhs.error.is_some() {
            return rhs;
        }

        // Function verification
        if let Some(error) = match (&expr_res.expr, &rhs.expr) {
            (Expr::Bool(_), Expr::Bool(_)) => None,
            _ => Some("Attempt to `AND` non-boolean expressions".to_string()),
        } {
            let error = Some(ExprErr(error));
            return ExprResult { error, ..expr_res };
        }

        // Apply operation
        expr_res.expr = Expr::Bool(BoolExpr {
            lhs: Box::new(expr_res.expr),
            op: And,
            rhs: Box::new(rhs.expr),
        });

        expr_res
    }

    fn or(self, rhs: impl Into<ExprResult>) -> ExprResult {
        let mut expr_res: ExprResult = self.into();
        let rhs: ExprResult = rhs.into();

        // Return any existing errors
        if expr_res.error.is_some() {
            return expr_res;
        }
        if rhs.error.is_some() {
            return rhs;
        }

        // Function verification
        if let Some(error) = match (&expr_res.expr, &rhs.expr) {
            (Expr::Bool(_), Expr::Bool(_)) => None,
            _ => Some("Attempt to `AND` non-boolean expressions".to_string()),
        } {
            let error = Some(ExprErr(error));
            return ExprResult { error, ..expr_res };
        }

        // Apply operation
        expr_res.expr = Expr::Bool(BoolExpr {
            lhs: Box::new(expr_res.expr),
            op: Or,
            rhs: Box::new(rhs.expr),
        });

        expr_res
    }

    fn gt(self, rhs: impl Into<ExprResult>) -> ExprResult {
        let mut expr_res: ExprResult = self.into();
        let rhs: ExprResult = rhs.into();

        // Return any existing errors
        if expr_res.error.is_some() {
            return expr_res;
        }
        if rhs.error.is_some() {
            return rhs;
        }

        // No function verification

        // Apply operation
        expr_res.expr = Expr::Bool(BoolExpr {
            lhs: Box::new(expr_res.expr),
            op: Gt,
            rhs: Box::new(rhs.expr),
        });

        expr_res
    }
}

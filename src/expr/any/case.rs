use super::super::prelude::*;

// Does not implement IntoMulti as it should never exist outside of a CaseExpr
pub struct CaseBranchExpr {
    pattern: Option<Box<dyn Boolean>>,
    result: Box<dyn CoreExpression>,
}

impl CaseBranchExpr {
    pub fn new(pattern: Option<Box<dyn Boolean>>, result: Box<dyn CoreExpression>) -> Self {
        Self { pattern, result }
    }
}

impl Expression for CaseBranchExpr {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
        let pat_conds = match &self.pattern {
            Some(boolean) => boolean.conditions(ExprType::Bool),
            None => Box::new(std::iter::empty()),
        };
        let res_conds = self.result.conditions(coerce);

        Box::new(pat_conds.chain(res_conds))
    }

    fn display(&self, dialect: Dialect) -> String {
        match &self.pattern {
            Some(pattern) => format!(
                "WHEN {} THEN {}",
                pattern.display(dialect),
                self.result.display(dialect)
            ),
            None => format!("ELSE {}", self.result.display(dialect)),
        }
    }
}
// impl CoreExpression for Column {}
impl CoreExpression for CaseBranchExpr {
    fn eval_type(&self) -> ExprType {
        self.result.eval_type()
    }
}
impl Boolean for CaseBranchExpr {}
impl Numeric for CaseBranchExpr {}
impl Textual for CaseBranchExpr {}
impl Anything for CaseBranchExpr {}

#[derive(IntoMultiCore)]
pub struct CaseExpr {
    branches: Vec<CaseBranchExpr>,
}

impl ExprType {
    pub fn try_fold(
        expr_t1: ExprType,
        expr_t2: ExprType,
    ) -> Result<ExprType, (ExprType, ExprType)> {
        match (expr_t1, expr_t2) {
            (ExprType::Any, ExprType::Any) => Ok(ExprType::Any),
            (ExprType::Any, e) | (e, ExprType::Any) => Ok(e),
            (a, b) if a == b => Ok(a),
            x => Err(x),
        }
    }
}

impl CaseExpr {
    fn validate(&self) -> Result<ExprType, (ExprType, ExprType)> {
        self.branches
            .iter()
            .map(|b| b.eval_type())
            .try_fold(ExprType::Any, ExprType::try_fold)
    }
}

impl CaseExpr {
    pub fn new(branches: Vec<CaseBranchExpr>) -> Self {
        Self { branches }
    }
}

impl Expression for CaseExpr {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
        // restrict the coerce given the branches' return types
        let coerce = match ExprType::try_fold(self.eval_type(), coerce) {
            Ok(coerce) => coerce,
            Err(x) => return Box::new(std::iter::once(Condition::CaseRetErr(x))),
        };

        // if coerce == Any -> add conditions for all return types to link up

        Box::new(self.branches.iter().flat_map(move |e| e.conditions(coerce)))
    }

    fn display(&self, dialect: Dialect) -> String {
        todo!()
    }
}
impl CoreExpression for CaseExpr {
    fn eval_type(&self) -> ExprType {
        self.validate().unwrap_or(ExprType::Any)
    }
}
impl Boolean for CaseExpr {}
impl Numeric for CaseExpr {}
impl Textual for CaseExpr {}
impl Anything for CaseExpr {}

// for some reason im being forced to wrap the blocks in another block (??)
macro_rules! case {
    ($($pat:expr => $res:expr),+, $(,)?) => {{
        let arr = [
            $(CaseBranchExpr::new(Some(Box::new($pat)), Box::new($res)),)+
        ];
        CaseExpr::new(Vec::from(arr))
    }};

    ($($pat:expr => $res:expr),+, _ => $final_res:expr, $(,)?) => {{
        let arr = [
            $(CaseBranchExpr::new(Some(Box::new($pat)), Box::new($res)),)+
            CaseBranchExpr::new(None, Box::new($final_res))
        ];
        CaseExpr::new(Vec::from(arr))
    }};
}

#[cfg(test)]
mod tests {
    use super::super::col;
    use super::*;

    #[test]
    fn all_any() {
        let case = case! {
            col("a") => col("b"),
            _ => col("d"),
        };
        assert_eq!(case.validate(), Ok(ExprType::Any));
    }

    #[test]
    fn downcast_any() {
        let case = case! {
            col("a") => col("b"),
            col("c") => 0,
        };
        assert_eq!(case.validate(), Ok(ExprType::Num));
    }

    #[test]
    fn error() {
        let case = case! {
            col("a") => true,
            col("c") => 0,
        };
        assert!(case.validate().is_err());
    }

    #[test]
    fn conds() {
        let case = case! {
            col("a") => col("b"),
            col("c") => col("d"),
        };

        for cond in case.conditions(ExprType::Any) {
            println!("{cond:?}");
        }
    }
}

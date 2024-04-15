use super::prelude::*;

pub trait Anything: Boolean + Numeric + Textual {}
impl<T: Anything> Anything for CommonExpr<T> {}

mod col {
    use super::*;

    pub struct Col {
        name: String,
    }

    pub fn col(name: impl Into<String>) -> CommonExpr<Col> {
        CommonExpr(Col { name: name.into() })
    }

    impl Client for Col {
        type Ctx = ExprType;
        type Msg = Message;

        fn children(
            &self,
            ctx: Self::Ctx,
        ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
            Vec::new()
        }

        fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
            vec![Condition::ColExistsAndType(self.name.clone(), ctx).into()]
        }
    }
    impl Checkable for Col {}
    impl Expression for Col {
        fn eval_type(&self) -> ExprType {
            ExprType::Any
        }

        fn display(&self, dialect: Dialect) -> String {
            self.name.to_string()
        }
    }
    impl Common for Col {}
    impl Boolean for Col {}
    impl Numeric for Col {}
    impl Textual for Col {}
    impl Anything for Col {}
}
pub use col::*;

mod case_branch {
    use super::*;

    pub struct CaseBranch {
        pattern: Option<Box<dyn Expression>>, // Boolean
        result: Box<dyn Expression>,          // Common
    }

    impl CaseBranch {
        pub fn new(pattern: Option<Box<dyn Expression>>, result: Box<dyn Expression>) -> Self {
            Self { pattern, result }
        }
    }

    impl Client for CaseBranch {
        type Ctx = ExprType;
        type Msg = Message;

        fn children(
            &self,
            ctx: Self::Ctx,
        ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
            let mut out = Vec::new();
            if self.pattern.is_some() {
                out.push((self.pattern.as_deref().unwrap() as _, ExprType::Bool))
            }
            out.push((self.result.as_ref() as _, ctx));
            out
        }

        fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
            Vec::new()
        }
    }
    impl Checkable for CaseBranch {}
    impl Expression for CaseBranch {
        fn eval_type(&self) -> ExprType {
            self.result.eval_type()
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
    impl Common for CaseBranch {}
    impl Boolean for CaseBranch {}
    impl Numeric for CaseBranch {}
    impl Textual for CaseBranch {}
    impl Anything for CaseBranch {}
}
pub use case_branch::*;

#[macro_use]
mod case {
    use super::*;

    pub struct Case {
        branches: Vec<CaseBranch>,
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
                mismatch => Err(mismatch),
            }
        }
    }

    impl Case {
        pub(in super::super) fn validate(&self) -> Result<ExprType, (ExprType, ExprType)> {
            self.branches
                .iter()
                .map(|b| b.eval_type())
                .try_fold(ExprType::Any, ExprType::try_fold)
        }
    }

    impl Case {
        pub fn new(branches: Vec<CaseBranch>) -> Self {
            Self { branches }
        }
    }

    impl Client for Case {
        type Ctx = ExprType;
        type Msg = Message;

        fn children(
            &self,
            ctx: Self::Ctx,
        ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
            self.branches
                .iter()
                .map(|branch| (branch as _, ctx))
                .collect()
        }

        fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
            // No standard messages to be sent, instead we use a custom `send_all` implementation
            // to ensure the required ordering of messages.
            // The custom `send_all` implementation should never call this function.
            unreachable!();
        }

        fn send_all(&self, ctx: Self::Ctx, server: &mut dyn Server<Msg = Self::Msg>) {
            match self.validate() {
                Err(mismatch) => {
                    server.accept(Message::Sig(Signal::TypeMismatch(mismatch)));
                }
                Ok(ExprType::Any) => {
                    server.accept(Signal::StartLink.into());
                    for (child, ctx) in self.children(ctx) {
                        child.send_all(ctx, server);
                    }
                    server.accept(Signal::EndLink.into());
                }
                Ok(_) => {
                    for (child, ctx) in self.children(ctx) {
                        child.send_all(ctx, server);
                    }
                }
            }
        }
    }
    impl Checkable for Case {}
    impl Expression for Case {
        fn eval_type(&self) -> ExprType {
            self.validate().unwrap_or(ExprType::Any)
        }

        fn display(&self, dialect: Dialect) -> String {
            todo!()
        }
    }
    impl Common for Case {}
    impl Boolean for Case {}
    impl Numeric for Case {}
    impl Textual for Case {}
    impl Anything for Case {}

    /// Build `CommonExpr<Case>` expressions using `match` semantics.
    macro_rules! case {
    ($($pat:expr => $res:expr),+, $(,)?) => {{
        let arr = [
            $(CaseBranch::new(Some(Box::new($pat)), Box::new($res)),)+
        ];
        CommonExpr(Case::new(Vec::from(arr)))
    }};

    ($($pat:expr => $res:expr),+, _ => $final_res:expr, $(,)?) => {{
        let arr = [
            $(CaseBranch::new(Some(Box::new($pat)), Box::new($res)),)+
            CaseBranch::new(None, Box::new($final_res))
        ];
        CommonExpr(Case::new(Vec::from(arr)))
    }};
}
}
pub use case::*;

#[cfg(test)]
mod tests {
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

        for msg in case.messages(ExprType::Any) {
            println!("{msg:?}");
        }
    }
}

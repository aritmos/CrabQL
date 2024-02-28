// TODO: Remove Q and T generics once their types are settled
pub struct Writer<'c, C, Q, T> {
    pub(super) checker: &'c mut C,
    pub(super) state: T,
    pub(super) query: Q,
}

pub struct SealedWriter<T> {
    state: T,
}

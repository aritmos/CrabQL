use super::schema::Schema;

pub type WriterT<'s, S> = Writer<'s, S, usize, Result<(), String>>;

pub struct Writer<'s, S, I, T> {
    pub schema: &'s mut S, // impl Schema
    pub internals: I,
    pub state: T,
}

impl<'s, S, I, T> Writer<'s, S, I, T> {
    pub fn new(schema: &'s mut S, internals: I, state: T) -> Self
    where
        S: Schema,
    {
        Writer {
            internals,
            schema,
            state,
        }
    }
}

pub enum Message {}

pub struct Expr<T> {
    inner: T,
    children: Option<Vec<Box<dyn Expression>>>,
    messages: Option<Vec<Message>>,
}

impl<T> Expr<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            children: None,
            messages: None,
        }
    }
}

pub trait Expression {}
pub trait Core {}
pub trait Boolean: Core {}
pub trait Textual: Core {}
pub trait Numeric: Core {}
pub trait Anything: Boolean + Textual + Numeric {}

pub struct Len;
impl Core for Len {}
impl Numeric for Len {}

pub struct Col {
    id: String,
}
impl Expression for Col {}
impl Core for Col {}
impl Boolean for Col {}
impl Textual for Col {}
impl Numeric for Col {}
impl Anything for Col {}

impl<T> Expression for Expr<T> {}

impl<T: Textual + 'static> Expr<T> {
    pub fn len(self) -> Expr<Len> {
        Expr::<Len> {
            inner: Len {},
            children: Some(vec![Box::new(self)]),
            messages: None,
        }
    }
}

fn test() {
    let e = Expr::<Col>::new(Col {
        id: "name".to_string(),
    });
    let l = e.len();
}

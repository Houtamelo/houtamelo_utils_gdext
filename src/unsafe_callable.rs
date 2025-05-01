use crate::internal::*;

#[allow(clippy::type_complexity)]
pub struct UnsafeCallable(Box<dyn FnMut(&[&Variant]) + 'static>);

impl UnsafeCallable {
    pub fn new(f: impl FnMut(&[&Variant]) + 'static) -> Self { Self(Box::new(f)) }

    fn invoke(&mut self, args: &[&Variant]) { self.0(args); }
}

unsafe impl Send for UnsafeCallable {}

unsafe impl Sync for UnsafeCallable {}

impl From<UnsafeCallable> for Callable {
    fn from(mut value: UnsafeCallable) -> Self {
        Callable::from_sync_fn("lambda", move |args| {
            value.invoke(args);
            Ok(Variant::nil())
        })
    }
}

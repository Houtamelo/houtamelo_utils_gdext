use crate::prelude::*;

#[allow(clippy::type_complexity)]
struct UnsafeFn(Box<dyn FnMut(&[&Variant]) + 'static>);

impl UnsafeFn {
	fn invoke(&mut self, args: &[&Variant]) {
		self.0(args);
	}
}

unsafe impl Send for UnsafeFn {}

unsafe impl Sync for UnsafeFn {}

impl From<UnsafeFn> for Callable {
	fn from(mut value: UnsafeFn) -> Self {
		Callable::from_fn("lambda",
			move |args| {
				value.invoke(args);
				Ok(Variant::nil())
			})
	}
}

#[allow(private_bounds)]
pub trait ConnectDeferred {
	fn connect_deferred(
		&mut self,
		signal: impl Into<StringName>,
		f: impl FnMut(&[&Variant]) + 'static,
	);
}

#[allow(private_bounds)]
impl<T: GodotClass + Inherits<Object>> ConnectDeferred for Gd<T> {
	fn connect_deferred(
		&mut self,
		signal: impl Into<StringName>,
		f: impl FnMut(&[&Variant]) + 'static,
	) {
		let signal = signal.into();
		let unsafe_fn = UnsafeFn(Box::new(f));

		self.upcast_mut()
		    .connect_ex(signal, unsafe_fn.into())
		    .flags(ConnectFlags::DEFERRED.ord() as u32)
		    .done();
	}
}
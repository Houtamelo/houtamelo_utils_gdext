use godot::meta::AsArg;

use super::UnsafeCallable;
use crate::internal::*;

#[allow(private_bounds)]
pub trait ConnectDeferred {
	fn connect_deferred(
		&mut self,
		signal: impl AsArg<StringName>,
		f: impl FnMut(&[&Variant]) + 'static,
	);
}

#[allow(private_bounds)]
impl<T: GodotClass + Inherits<Object>> ConnectDeferred for Gd<T> {
	fn connect_deferred(
		&mut self,
		signal: impl AsArg<StringName>,
		f: impl FnMut(&[&Variant]) + 'static,
	) {
		let unsafe_fn = UnsafeCallable(Box::new(f));
		let callable = Callable::from(unsafe_fn);

		self.upcast_mut()
			.connect_ex(signal, &callable)
			.flags(ConnectFlags::DEFERRED.ord() as u32)
			.done();
	}
}

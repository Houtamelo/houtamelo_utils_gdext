use crate::prelude::*;

struct UnsafeFn<F>(F);

impl<F: FnMut(&[&Variant])> UnsafeFn<F> {
	fn invoke(&mut self, args: &[&Variant]) {
		self.0(args);
	}
}

unsafe impl<F> Send for UnsafeFn<F> {}
unsafe impl<F> Sync for UnsafeFn<F> {}

enum FlexCallable {
	Callable(Callable),
	#[allow(clippy::type_complexity)]
	Closure(UnsafeFn<Box<dyn FnMut(&[&Variant])>>),
}

impl From<FlexCallable> for Callable {
	fn from(value: FlexCallable) -> Self {
		match value {
			FlexCallable::Callable(callable) => callable,
			FlexCallable::Closure(mut unsafe_fn) => {
				Callable::from_fn("lambda", move |args| {
					unsafe_fn.invoke(args);
					Ok(Variant::nil())
				})
			}
		}
	}
}

impl<T: FnMut(&[&Variant]) + 'static> From<T> for FlexCallable {
	fn from(value: T) -> Self {
		FlexCallable::Closure(UnsafeFn(Box::new(value)))
	}
}

impl From<Callable> for FlexCallable {
	fn from(value: Callable) -> Self {
		FlexCallable::Callable(value)
	}
}

#[allow(private_bounds)]
pub trait ConnectDeferred {
	fn connect_deferred(&mut self, signal: impl Into<StringName>, callable: impl Into<FlexCallable>);
}

#[allow(private_bounds)]
impl<T: GodotClass + Inherits<Object>> ConnectDeferred for Gd<T> {
	fn connect_deferred(&mut self, signal: impl Into<StringName>, callable: impl Into<FlexCallable>) {
		let signal = signal.into();
		let callable = callable.into();

		self.upcast_mut()
		    .connect_ex(signal, callable.into())
		    .flags(ConnectFlags::DEFERRED.ord() as u32)
		    .done();
	}
}
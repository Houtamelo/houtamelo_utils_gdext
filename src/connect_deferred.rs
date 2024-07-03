use crate::prelude::*;

struct UnsafeFn<F: FnMut()>(F);

impl<F: FnMut()> UnsafeFn<F> {
	fn invoke(&mut self) {
		self.0();
	}
}

unsafe impl<F: FnMut()> Send for UnsafeFn<F> {}
unsafe impl<F: FnMut()> Sync for UnsafeFn<F> {}

enum FlexCallable {
	Callable(Callable),
	Closure(UnsafeFn<Box<dyn FnMut()>>),
}

impl From<FlexCallable> for Callable {
	fn from(value: FlexCallable) -> Self {
		match value {
			FlexCallable::Callable(callable) => callable,
			FlexCallable::Closure(mut unsafe_fn) => 
				Callable::from_fn("lambda", move |_| {
					unsafe_fn.invoke();
					Ok(Variant::nil())
				}),
		}
	}
}

impl<T: FnMut() + 'static> From<T> for FlexCallable {
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
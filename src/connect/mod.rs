pub mod connect_child;
pub mod connect_deferred;
pub mod connect_with_deferred;

use godot::obj::bounds::DeclUser;

use crate::internal::*;

struct UnsafeWrapper<T: GodotClass<Declarer = DeclUser>> {
	#[allow(clippy::type_complexity)]
	f: Box<dyn FnMut(&mut T, &[&Variant])>,
	node: Gd<T>,
}

unsafe impl<T: GodotClass<Declarer = DeclUser>> Send for UnsafeWrapper<T> {}

unsafe impl<T: GodotClass<Declarer = DeclUser>> Sync for UnsafeWrapper<T> {}

impl<T> UnsafeWrapper<T>
where T: GodotClass<Declarer = DeclUser>
{
	fn invoke(&mut self, args: &[&Variant]) {
		let mut bind = self.node.bind_mut();
		(self.f)(&mut bind, args);
	}
}

#[allow(clippy::type_complexity)]
struct UnsafeFn(Box<dyn FnMut(&[&Variant]) + 'static>);

impl UnsafeFn {
	fn invoke(&mut self, args: &[&Variant]) { self.0(args); }
}

unsafe impl Send for UnsafeFn {}

unsafe impl Sync for UnsafeFn {}

impl From<UnsafeFn> for Callable {
	fn from(mut value: UnsafeFn) -> Self {
		Callable::from_fn("lambda", move |args| {
			value.invoke(args);
			Ok(Variant::nil())
		})
	}
}

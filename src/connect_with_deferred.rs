use godot::obj::bounds::DeclUser;
use crate::prelude::*;

struct UnsafeWrapper<T: GodotClass<Declarer = DeclUser>> {
	#[allow(clippy::type_complexity)]
	f: Box<dyn FnMut(&mut T, &[&Variant])>,
	node: Gd<T>,
}

unsafe impl<T: GodotClass<Declarer = DeclUser>> Send for UnsafeWrapper<T> {}

unsafe impl<T: GodotClass<Declarer = DeclUser>> Sync for UnsafeWrapper<T> {}

impl<T> UnsafeWrapper<T>
	where
		T: GodotClass<Declarer = DeclUser>,
{
	fn invoke(&mut self, args: &[&Variant]) {
		let mut bind = self.node.bind_mut();
		(self.f)(&mut bind, args);
	}
}

#[allow(private_bounds)]
pub trait ConnectWithDeferred: Sized {
	fn connect_with_deferred<Other>(
		&mut self,
		other: Gd<Other>,
		signal: impl Into<StringName>,
		callable: impl FnMut(&mut Self, &[&Variant]) + 'static,
	) -> godot::global::Error
		where
			Other: GodotClass + Inherits<Object>;
}

#[allow(private_bounds)]
impl<T> ConnectWithDeferred for T
	where
		T: GodotClass<Declarer = DeclUser> + Inherits<Node> + godot::obj::WithBaseField,
{
	fn connect_with_deferred<Other>(
		&mut self,
		mut other: Gd<Other>,
		signal: impl Into<StringName>,
		f: impl FnMut(&mut T, &[&Variant]) + 'static,
	) -> godot::global::Error
		where
			Other: GodotClass + Inherits<Object>,
	{
		let signal = signal.into();

		let self_node = self.to_gd();

		let mut wrapper = UnsafeWrapper { f: Box::new(f), node: self_node.clone() };

		let callable = Callable::from_fn(
			"lambda",
			move |args| {
				wrapper.invoke(args);
				Ok(Variant::nil())
			});

		other.upcast_mut()
			 .connect_ex(signal, callable)
		     .flags(ConnectFlags::DEFERRED.ord() as u32)
		     .done()
	}
}
use godot::{
	meta::AsArg,
	obj::{WithBaseField, bounds::DeclUser},
};

use super::UnsafeWrapper;
use crate::internal::*;

#[allow(private_bounds)]
pub trait ConnectWithDeferred: Sized {
	fn connect_with_deferred<Other>(
		&mut self,
		other: &Gd<Other>,
		signal: impl AsArg<StringName>,
		callable: impl FnMut(&mut Self, &[&Variant]) + 'static,
	) -> GodotError
	where
		Other: GodotClass + Inherits<Object>;
}

#[allow(private_bounds)]
impl<T> ConnectWithDeferred for T
where T: GodotClass<Declarer = DeclUser> + Inherits<Node> + WithBaseField
{
	fn connect_with_deferred<Other>(
		&mut self,
		other: &Gd<Other>,
		signal: impl AsArg<StringName>,
		f: impl FnMut(&mut T, &[&Variant]) + 'static,
	) -> GodotError
	where
		Other: GodotClass + Inherits<Object>,
	{
		let self_node = self.to_gd();

		let mut wrapper = UnsafeWrapper {
			f: Box::new(f),
			node: self_node.clone(),
		};

		let callable = Callable::from_sync_fn("lambda", move |args| {
			wrapper.invoke(args);
			Ok(Variant::nil())
		});

		other
			.clone()
			.upcast_mut()
			.connect_ex(signal, &callable)
			.flags(ConnectFlags::DEFERRED.ord() as u32)
			.done()
	}
}

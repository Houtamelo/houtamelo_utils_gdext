use anyhow::Result;
use godot::{meta::AsArg, obj::bounds::DeclUser};

use super::UnsafeWrapper;
use crate::{internal::*, prelude::*};

#[allow(private_bounds)]
pub trait ConnectChild: Sized {
	fn connect_child(
		&mut self,
		child_path: impl AsArg<NodePath>,
		signal: impl AsArg<StringName>,
		callable: impl FnMut(&mut Self, &[&Variant]) + 'static,
	) -> Result<()>;
}

#[allow(private_bounds)]
impl<T> ConnectChild for T
where T: GodotClass<Declarer = DeclUser> + Inherits<Node> + godot::obj::WithBaseField
{
	fn connect_child(
		&mut self,
		child_path: impl AsArg<NodePath>,
		signal: impl AsArg<StringName>,
		f: impl FnMut(&mut T, &[&Variant]) + 'static,
	) -> Result<()> {
		let node = self.to_gd();

		let mut wrapper = UnsafeWrapper {
			f: Box::new(f),
			node: node.clone(),
		};

		let callable = Callable::from_fn("lambda", move |args| {
			wrapper.invoke(args);
			Ok(Variant::nil())
		});

		node.map_node(child_path, |child: &mut Gd<Node>| {
			child
				.connect_ex(signal, &callable)
				.flags(ConnectFlags::DEFERRED.ord() as u32)
				.done();
		})
	}
}

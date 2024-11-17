use godot::obj::bounds::DeclUser;
use crate::prelude::*;
use anyhow::Result;
use godot::meta::AsArg;
use crate::internal::*;

struct UnsafeWrapper<T: GodotClass<Declarer = DeclUser>> {
	#[allow(clippy::type_complexity)]
	f: Box<dyn FnMut(&mut T, &[&Variant])>,
	node: Gd<T>,
}

unsafe impl<T: GodotClass<Declarer = DeclUser>> Send for UnsafeWrapper<T> {}

unsafe impl<T: GodotClass<Declarer = DeclUser>> Sync for UnsafeWrapper<T> {}

impl<T> UnsafeWrapper<T>
	where
		T: GodotClass<Declarer = DeclUser>
{
	fn invoke(&mut self, args: &[&Variant]) {
		let mut bind = self.node.bind_mut();
		(self.f)(&mut bind, args);
	}
}

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
	where
		T: GodotClass<Declarer = DeclUser> + Inherits<Node> + godot::obj::WithBaseField,
{
	fn connect_child(
		&mut self,
		child_path: impl AsArg<NodePath>,
		signal: impl AsArg<StringName>,
		f: impl FnMut(&mut T, &[&Variant]) + 'static,
	) -> Result<()> {
		let node = self.to_gd();
		
		let mut wrapper = UnsafeWrapper { f: Box::new(f), node: node.clone() };
		
		let callable = Callable::from_fn(
			"lambda",
			move |args| {
				wrapper.invoke(args);
				Ok(Variant::nil())
			});

		node.map_node(
			child_path,
			|child: &mut Gd<Node>| {
				child.connect_ex(signal, &callable)
				     .flags(ConnectFlags::DEFERRED.ord() as u32)
				     .done();
			})
	}
}
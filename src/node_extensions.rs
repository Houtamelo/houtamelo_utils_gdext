use anyhow::{Result, anyhow};
use godot::meta::AsArg;

use crate::internal::*;

pub trait MapNode {
	fn map_node<TNode, TMap>(
		&self,
		node_path: impl AsArg<NodePath>,
		f: impl FnOnce(&mut Gd<TNode>) -> TMap,
	) -> Result<TMap>
	where
		TNode: Inherits<Node>;
}

impl<TSelf> MapNode for Gd<TSelf>
where TSelf: Inherits<Node>
{
	fn map_node<TNode, TMap>(
		&self,
		path: impl AsArg<NodePath>,
		f: impl FnOnce(&mut Gd<TNode>) -> TMap,
	) -> Result<TMap>
	where
		TNode: Inherits<Node>,
	{
		let this = self.clone().upcast();

		let path = path.into_arg();
		let path_ref = path.cow_as_ref();

		this.get_node_or_null(path_ref)
			.ok_or_else(|| {
				let self_name = this.get_name();
				anyhow!("Node \"{self_name}\" does not have child at path \"{path_ref}\"")
			})
			.and_then(|node| {
				node.try_cast::<TNode>().map_err(|err| {
					anyhow!(
						"Could not cast node to type `{ty}`\n\
						     Path: \"{path_ref}\"\n\
						     Error: {err}",
						ty = std::any::type_name::<TNode>()
					)
				})
			})
			.map(|mut node| f(&mut node))
	}
}

pub trait GetChild {
	fn get_child<T: Inherits<Node>>(&self, node_path: impl AsArg<NodePath>) -> Result<Gd<T>>;
}

impl<TSelf> GetChild for Gd<TSelf>
where TSelf: Inherits<Node>
{
	fn get_child<T: Inherits<Node>>(&self, node_path: impl AsArg<NodePath>) -> Result<Gd<T>> {
		let this = self.clone().upcast();

		let node_path = node_path.into_arg();
		let node_path_ref = node_path.cow_as_ref();

		this.get_node_or_null(node_path_ref)
			.ok_or_else(|| {
				let self_name = this.get_name();
				anyhow!("Node \"{self_name}\" does not have child at path \"{node_path_ref}\"")
			})
			.and_then(|node| {
				node.try_cast::<T>().map_err(|err| {
					anyhow!(
						"Could not cast node to type `{ty}`\n\
						     Path: \"{node_path_ref}\"\n\
						     Error: {err}",
						ty = std::any::type_name::<T>()
					)
				})
			})
	}
}

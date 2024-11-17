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
			TNode: GodotClass + Inherits<Node>;
}

impl<TSelf> MapNode for Gd<TSelf>
	where
		TSelf: GodotClass + Inherits<Node>
{
	fn map_node<TNode, TMap>(
		&self,
		path: impl AsArg<NodePath>,
		f: impl FnOnce(&mut Gd<TNode>) -> TMap,
	) -> Result<TMap>
		where
			TNode: GodotClass + Inherits<Node> 
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
			    node.try_cast::<TNode>()
			        .map_err(|err| {
				        anyhow!(
						    "Could not cast node to type `{ty}`\n\
						     Path: \"{path_ref}\"\n\
						     Error: {err}", ty = std::any::type_name::<TNode>())
			        })
		    })
		    .map(|mut node| {
			    f(&mut node)
		    })
	}
}
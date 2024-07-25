use anyhow::{Result, anyhow};
use crate::prelude::*;

pub trait MapNode {
	fn map_node<TNode, TMap>(
		&self,
		node_path: impl Into<NodePath>,
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
		node_path: impl Into<NodePath>,
		f: impl FnOnce(&mut Gd<TNode>) -> TMap,
	) -> Result<TMap>
		where
			TNode: GodotClass + Inherits<Node> 
	{
		let this = self.clone().upcast();
		let path = node_path.into();
		this.get_node_or_null(path.clone())
		    .ok_or_else(|| {
			    let self_name = this.get_name();
			    anyhow!("Node \"{self_name}\" does not have child at path \"{path}\"")
		    })
		    .and_then(|node| {
			    node.try_cast::<TNode>()
			        .map_err(|err| {
				        anyhow!(
						    "Could not cast node to type `{ty}`\n\
						     Path: \"{path}\"\n\
						     Error: {err}", ty = std::any::type_name::<TNode>())
			        })
		    })
		    .map(|mut node| {
			    f(&mut node)
		    })
	}
}
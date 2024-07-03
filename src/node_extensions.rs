use util::prelude::*;

use crate::prelude::*;

pub trait InspectNode {
	fn touch_node<T: GodotClass + Inherits<Node>>(
		&self, 
		node_path: impl Into<NodePath>, 
		f: impl FnOnce(Gd<T>),
	) -> Result<()>;
}

impl<TSelf: GodotClass + Inherits<Node>> InspectNode for Gd<TSelf> {
	fn touch_node<TNode: GodotClass + Inherits<Node>>(
		&self, 
		node_path: impl Into<NodePath>, 
		f: impl FnOnce(Gd<TNode>),
	) -> Result<()> {
		self.map_node(node_path, f)
	}
}

pub trait MapNode {
	fn map_node<T: GodotClass + Inherits<Node>, TMap>(
		&self,
		node_path: impl Into<NodePath>,
		f: impl FnOnce(Gd<T>) -> TMap,
	) -> Result<TMap>;
}

impl<TSelf: GodotClass + Inherits<Node>> MapNode for Gd<TSelf> {
	fn map_node<TNode: GodotClass + Inherits<Node>, TMap>(
		&self,
		node_path: impl Into<NodePath>,
		f: impl FnOnce(Gd<TNode>) -> TMap,
	) -> Result<TMap> {
		let this = self.clone().upcast();
		let path = node_path.into();
		this.get_node_or_null(path.clone())
		    .ok_or_else(|| {
			    let self_name = this.get_name();
			    anyhow!("Node \"{self_name}\" does not have child at path \"{path}\"")
		    })
		    .and_then(|node| {
			    node.try_cast()
				    .map_err(|err| { 
					    anyhow!(
						    "Could not cast node to type `{ty}`\n\
						     Path: \"{path}\"\n\
						     Error: {err}", ty = type_name::<TNode>())
				    })
		    })
		    .map(f)
	}
}
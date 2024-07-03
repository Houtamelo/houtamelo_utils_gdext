use godot::classes::resource_loader::CacheMode;
use util::prelude::*;

use crate::prelude::*;

pub fn load_prefab(path: impl Into<GString>) -> Result<Gd<PackedScene>> {
	let path = path.into();

	ResourceLoader::singleton()
		.load_ex(path.clone())
		.type_hint("PackedScene".into())
		.cache_mode(CacheMode::REUSE)
		.done()
		.ok_or_else(|| anyhow!(
			"Resource could not be loaded.\n\
			 Path: {path}"
		))
		.and_then(|res| {
			res.try_cast::<PackedScene>()
			   .map_err(|err| anyhow!(
				   "Loaded resource is not a PackedScene.\n\
				    Path \"{path}\"\n\
				    Error: {err}"
			   ))
		})
}

pub fn spawn_prefab_as<T: GodotClass + Inherits<Node>>(path: impl Into<GString>) -> Result<Gd<T>> {
	load_prefab(path)?.spawn_as()
}

pub fn load_resource_as<T: GodotClass + Inherits<Resource>>(path: impl Into<GString>) -> Result<Gd<T>> {
	let path = path.into();

	ResourceLoader::singleton()
		.load_ex(path.clone())
		.type_hint(T::class_name().to_gstring())
		.cache_mode(CacheMode::REUSE)
		.done()
		.ok_or_else(|| anyhow!(
			"Resource could not be loaded.\n\
			 Path: {path}"
		))
		.and_then(|res| {
			res.try_cast::<T>()
			   .map_err(|err| {
				   let type_name = type_name::<T>();
				   anyhow!(
					   "Loaded resource is not of type {type_name}.\n\
					    Path \"{path}\"\n\
					    Error: {err}"
				   )
			   })
		})
}

pub trait SpawnAs {
	fn spawn_as<T: Inherits<Node> + GodotClass>(&self) -> Result<Gd<T>>;
}

impl SpawnAs for Gd<PackedScene> {
	fn spawn_as<T: Inherits<Node> + GodotClass>(&self) -> Result<Gd<T>> {
		self.try_instantiate_as()
		    .ok_or_else(|| {
			    let type_name = type_name::<T>();
			    let self_name = self.get_name();
			    anyhow!(
				    "Could not instantiate prefab, scene is not of type {type_name}. \n\
				     Scene name: {self_name}"
			    )
		    })
	}
}
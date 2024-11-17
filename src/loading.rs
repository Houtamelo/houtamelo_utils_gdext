use godot::classes::resource_loader::CacheMode;
use anyhow::{Result, anyhow};
use godot::meta::AsArg;
use crate::internal::*;

pub fn load_prefab(path: impl AsArg<GString>) -> Result<Gd<PackedScene>> {
	let path = path.into_arg();
	let path_ref = path.cow_as_ref();
	
	ResourceLoader::singleton()
		.load_ex(path_ref)
		.type_hint("PackedScene")
		.cache_mode(CacheMode::REUSE)
		.done()
		.ok_or_else(|| anyhow!(
			"Resource could not be loaded.\n\
			 Path: {path_ref}"
		))
		.and_then(|res| {
			res.try_cast::<PackedScene>()
			   .map_err(|err| anyhow!(
				   "Loaded resource is not a PackedScene.\n\
				    Path \"{path_ref}\"\n\
				    Error: {err}"
			   ))
		})
}

pub fn spawn_prefab_as<T: GodotClass + Inherits<Node>>(path: impl AsArg<GString>) -> Result<Gd<T>> {
	load_prefab(path)?.spawn_as::<T>()
}

pub fn load_resource_as<T: GodotClass + Inherits<Resource>>(path: impl AsArg<GString>) -> Result<Gd<T>> {
	let path = path.into_arg();
	let path_ref = path.cow_as_ref();

	ResourceLoader::singleton()
		.load_ex(path_ref)
		.type_hint(&T::class_name().to_gstring())
		.cache_mode(CacheMode::REUSE)
		.done()
		.ok_or_else(|| anyhow!(
			"Resource could not be loaded.\n\
			 Path: {path_ref}"
		))
		.and_then(|res| {
			res.try_cast::<T>()
			   .map_err(|err| {
				   let type_name = std::any::type_name::<T>();
				   anyhow!(
					   "Loaded resource is not of type {type_name}.\n\
					    Path \"{path_ref}\"\n\
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
		self.try_instantiate_as::<T>()
		    .ok_or_else(|| {
			    let type_name = std::any::type_name::<T>();
			    let self_name = self.get_name();
			    anyhow!(
				    "Could not instantiate prefab, scene is not of type {type_name}. \n\
				     Scene name: {self_name}"
			    )
		    })
	}
}
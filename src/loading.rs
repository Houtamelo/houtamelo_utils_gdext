use anyhow::{Result, anyhow, bail};
use godot::{classes::resource_loader::CacheMode, meta::AsArg};

use crate::internal::*;

pub fn load_prefab(path: impl AsArg<GString>) -> Result<Gd<PackedScene>> {
    let path = path.into_arg();
    let path_ref = path.cow_as_ref();

    ResourceLoader::singleton()
        .load_ex(path_ref)
        .type_hint("PackedScene")
        .cache_mode(CacheMode::REUSE)
        .done()
        .ok_or_else(|| anyhow!("Resource could not be loaded.\nPath: {path_ref}"))
        .and_then(|res| {
            res.try_cast::<PackedScene>()
                .map_err(|err| anyhow!("Loaded resource is not a PackedScene.\nPath \"{path_ref}\"\nError: {err}"))
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
        .type_hint(&T::class_id().to_gstring())
        .cache_mode(CacheMode::REUSE)
        .done()
        .ok_or_else(|| anyhow!("Resource could not be loaded.\nPath: {path_ref}"))
        .and_then(|res| {
            res.try_cast::<T>().map_err(|err| {
                let type_name = std::any::type_name::<T>();
                anyhow!("Loaded resource is not of type {type_name}.\nPath \"{path_ref}\"\nError: {err}")
            })
        })
}

pub trait SpawnAs {
    fn spawn_as<T: Inherits<Node> + GodotClass>(&self) -> Result<Gd<T>>;
}

impl SpawnAs for Gd<PackedScene> {
    fn spawn_as<T: Inherits<Node> + GodotClass>(&self) -> Result<Gd<T>> {
        let node = self
            .instantiate()
            .ok_or_else(|| anyhow!("Scene instantiation failed, path=\"{}\"", self.get_path()))?;

        match node.try_cast::<T>() {
            Ok(t) => Ok(t),
            Err(mut node) => {
                let class = node.get_class();
                node.queue_free();
                bail!(
                    "Scene instantiation succeed, but node is not of expected type=`{}`, actual type=`{}`, path=\"{}\"",
                    std::any::type_name::<T>(),
                    class,
                    self.get_path()
                )
            }
        }
    }
}

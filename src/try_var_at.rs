use anyhow::{Result, anyhow};

use crate::internal::*;

pub trait TryVarAt<Key> {
	fn try_var_at<Var: FromGodot>(&self, key: Key) -> Result<Var>;
}

impl<Key: ToGodot + Clone + std::fmt::Debug> TryVarAt<Key> for Dictionary {
	fn try_var_at<Var: FromGodot>(&self, key: Key) -> Result<Var> {
		self.get(key.clone())
			.ok_or_else(|| anyhow!("Dictionary does not contain key \"{key:?}\""))
			.and_then(|val| {
				val.try_to::<Var>().map_err(|err| {
					anyhow!(
						"Dictionary contains key \"{key:?}\", but value is not of expected type `{}`.\n\
						 Conversion Error: {err:?}",
						std::any::type_name::<Var>()
					)
				})
			})
	}
}

impl TryVarAt<usize> for VariantArray {
	fn try_var_at<Var: FromGodot>(&self, index: usize) -> Result<Var> {
		self.get(index)
			.ok_or_else(|| {
				anyhow!("Index `{index}` is out of bounds. Array length: `{}`", self.len())
			})
			.and_then(|val| {
				val.try_to::<Var>().map_err(|err| {
					anyhow!(
						"Index `{index}` is in bounds, but value is not of expected type `{}`.\n\
						 Conversion Error: {err:?}",
						std::any::type_name::<Var>()
					)
				})
			})
	}
}

impl TryVarAt<usize> for &[Variant] {
	fn try_var_at<Var: FromGodot>(&self, index: usize) -> Result<Var> {
		self.get(index)
			.ok_or_else(|| {
				anyhow!("Index `{index}` is out of bounds. Array length: `{}`", self.len())
			})
			.and_then(|val| {
				val.try_to::<Var>().map_err(|err| {
					anyhow!(
						"Index `{index}` is in bounds, but value is not of expected type `{}`.\n\
						 Conversion Error: {err:?}",
						std::any::type_name::<Var>()
					)
				})
			})
	}
}

impl TryVarAt<usize> for &[&Variant] {
	fn try_var_at<Var: FromGodot>(&self, index: usize) -> Result<Var> {
		self.get(index)
			.ok_or_else(|| {
				anyhow!("Index `{index}` is out of bounds. Array length: `{}`", self.len())
			})
			.and_then(|val| {
				val.try_to::<Var>().map_err(|err| {
					anyhow!(
						"Index `{index}` is in bounds, but value is not of expected type `{}`.\n\
						 Conversion Error: {err:?}",
						std::any::type_name::<Var>()
					)
				})
			})
	}
}

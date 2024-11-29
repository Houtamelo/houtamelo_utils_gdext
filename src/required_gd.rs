use crate::internal::*;

use std::{
	backtrace::Backtrace,
	fmt::{Debug, Formatter},
	hash::{Hash, Hasher},
	ops::{Deref, DerefMut},
};

use godot::{
	meta::PropertyHintInfo,
	obj::{Bounds, bounds},
};


pub struct RequiredGd<T: GodotClass> {
	gd: Option<Gd<T>>,
}

impl<T: GodotClass> RequiredGd<T> {
	pub fn new(gd: Gd<T>) -> Self { Self { gd: Some(gd) } }
}

impl<T: GodotClass> Clone for RequiredGd<T> {
	fn clone(&self) -> Self {
		Self {
			gd: self.gd.clone(),
		}
	}
}

impl<T: GodotClass> Debug for RequiredGd<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { self.gd.fmt(f) }
}

impl<T: GodotClass> PartialEq for RequiredGd<T> {
	fn eq(&self, other: &Self) -> bool { self.gd == other.gd }
}

impl<T: GodotClass> Eq for RequiredGd<T> {}

impl<T: GodotClass> Hash for RequiredGd<T> {
	fn hash<H: Hasher>(&self, state: &mut H) { self.gd.hash(state) }
}

impl<T: GodotClass> Default for RequiredGd<T> {
	fn default() -> Self { Self { gd: None } }
}

impl<T: GodotClass> GodotConvert for RequiredGd<T> {
	type Via = Option<Gd<T>>;
}

impl<T: GodotClass> ToGodot for RequiredGd<T> {
	type ToVia<'v> = Self::Via;

	fn to_godot(&self) -> Self::Via { self.gd.clone() }
}

impl<T: GodotClass> FromGodot for RequiredGd<T> {
	fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> { Ok(Self { gd: via }) }
}

impl<T: GodotClass> Var for RequiredGd<T> {
	fn get_property(&self) -> Self::Via { self.gd.clone() }

	fn set_property(&mut self, value: Self::Via) { self.gd = value }

	fn var_hint() -> PropertyHintInfo { <Option<Gd<T>> as Var>::var_hint() }
}

impl<T> Export for RequiredGd<T>
where T: GodotClass + Bounds<Exportable = bounds::Yes>
{
	fn export_hint() -> PropertyHintInfo { <Option<Gd<T>> as Export>::export_hint() }
}

impl<T: GodotClass> Deref for RequiredGd<T> {
	type Target = Gd<T>;

	fn deref(&self) -> &Self::Target {
		self.gd.as_ref().unwrap_or_else(|| {
			panic!(
				"RequiredExport has null field at runtime, this is not allowed.\n\
				 Stack Trace:\n {}",
				Backtrace::capture()
			)
		})
	}
}

impl<T: GodotClass> DerefMut for RequiredGd<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.gd.as_mut().unwrap_or_else(|| {
			panic!(
				"RequiredExport has null field at runtime, this is not allowed.\n\
				 Stack Trace:\n {}",
				Backtrace::capture()
			)
		})
	}
}

impl<T: GodotClass> std::panic::UnwindSafe for RequiredGd<T> {}

impl<T: GodotClass> std::panic::RefUnwindSafe for RequiredGd<T> {}
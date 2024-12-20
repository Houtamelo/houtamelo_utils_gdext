use godot::meta::AsArg;
use crate::internal::*;
use crate::internal::class_macros::meta::ParamType;

pub struct ConstGString {
	gstring: GString,
	str: &'static str,
}

unsafe impl Send for ConstGString {}

unsafe impl Sync for ConstGString {}

impl ConstGString {
	pub fn new(str: &'static str) -> Self {
		Self { gstring: GString::from(str), str }
	}

	pub fn get(&self) -> GString {
		self.gstring.clone()
	}
	
	pub fn as_str(&self) -> &'static str {
		self.str
	}
}

impl From<&'static str> for ConstGString {
	fn from(s: &'static str) -> Self {
		Self::new(s)
	}
}

impl AsArg<StringName> for ConstGString {
	fn into_arg<'r>(self) -> <StringName as ParamType>::Arg<'r>
		where Self: 'r
	{
		<&str as AsArg<StringName>>::into_arg(self.str)
	}
}

impl AsArg<StringName> for &ConstGString {
	fn into_arg<'r>(self) -> <StringName as ParamType>::Arg<'r>
		where Self: 'r
	{
		<&str as AsArg<StringName>>::into_arg(self.str)
	}
}

impl AsArg<GString> for ConstGString {
	fn into_arg<'r>(self) -> <GString as ParamType>::Arg<'r>
		where Self: 'r
	{
		<&str as AsArg<GString>>::into_arg(self.str)
	}
}

impl AsArg<GString> for &ConstGString {
	fn into_arg<'r>(self) -> <GString as ParamType>::Arg<'r>
		where Self: 'r
	{
		<&str as AsArg<GString>>::into_arg(self.str)
	}
}

impl AsArg<NodePath> for ConstGString {
	fn into_arg<'r>(self) -> <NodePath as ParamType>::Arg<'r>
		where Self: 'r
	{
		<&str as AsArg<NodePath>>::into_arg(self.str)
	}
}

impl AsArg<NodePath> for &ConstGString {
	fn into_arg<'r>(self) -> <NodePath as ParamType>::Arg<'r>
		where Self: 'r
	{
		<&str as AsArg<NodePath>>::into_arg(self.str)
	}
}

pub struct ConstStringName {
	string_name: StringName,
	str: &'static str,
}

impl AsArg<StringName> for ConstStringName {
	fn into_arg<'r>(self) -> <StringName as ParamType>::Arg<'r>
		where Self: 'r
	{
		<&str as AsArg<StringName>>::into_arg(self.str)
	}
}

impl AsArg<GString> for ConstStringName {
	fn into_arg<'r>(self) -> <GString as ParamType>::Arg<'r>
		where Self: 'r
	{
		<&str as AsArg<GString>>::into_arg(self.str)
	}
}

impl AsArg<NodePath> for ConstStringName {
	fn into_arg<'r>(self) -> <NodePath as ParamType>::Arg<'r>
		where Self: 'r
	{
		<&str as AsArg<NodePath>>::into_arg(self.str)
	}
}


impl AsArg<StringName> for &ConstStringName {
	fn into_arg<'r>(self) -> <StringName as ParamType>::Arg<'r>
		where Self: 'r
	{
		<&str as AsArg<StringName>>::into_arg(self.str)
	}
}

impl AsArg<GString> for &ConstStringName {
	fn into_arg<'r>(self) -> <GString as ParamType>::Arg<'r>
		where Self: 'r
	{
		<&str as AsArg<GString>>::into_arg(self.str)
	}
}

impl AsArg<NodePath> for &ConstStringName {
	fn into_arg<'r>(self) -> <NodePath as ParamType>::Arg<'r>
		where Self: 'r
	{
		<&str as AsArg<NodePath>>::into_arg(self.str)
	}
}

unsafe impl Send for ConstStringName {}

unsafe impl Sync for ConstStringName {}

impl ConstStringName {
	pub fn new(str: &'static str) -> Self {
		Self { string_name: StringName::from(str), str }
	}

	pub fn get(&self) -> StringName {
		self.string_name.clone()
	}

	pub fn as_str(&self) -> &'static str {
		self.str
	}
}

impl From<&'static str> for ConstStringName {
	fn from(s: &'static str) -> Self {
		Self::new(s)
	}
}

#[macro_export]
macro_rules! lazy_gstring {
    ($visibility: vis $var_name: ident = $value: expr) => {
	    $visibility static $var_name: std::sync::LazyLock<$crate::prelude::ConstGString> = 
	        std::sync::LazyLock::new(|| $crate::prelude::ConstGString::new($value)); 
    };
}

#[macro_export]
macro_rules! lazy_stringname {
    ($visibility: vis $var_name: ident = $value: expr) => {
	    $visibility static $var_name: std::sync::LazyLock< $crate::prelude::ConstStringName> = 
	        std::sync::LazyLock::new(|| $crate::prelude::ConstStringName::new($value));
    };
}
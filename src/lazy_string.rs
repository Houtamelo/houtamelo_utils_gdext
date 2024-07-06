use crate::prelude::*;

pub struct ConstGString(GString);

unsafe impl Send for ConstGString {}

unsafe impl Sync for ConstGString {}

impl ConstGString {
	pub fn new(s: &str) -> Self {
		Self(GString::from(s))
	}

	pub fn get(&self) -> GString {
		self.0.clone()
	}
}

impl From<&'static str> for ConstGString {
	fn from(s: &'static str) -> Self {
		Self::new(s)
	}
}

pub struct ConstStringName(StringName);

unsafe impl Send for ConstStringName {}

unsafe impl Sync for ConstStringName {}

impl ConstStringName {
	pub fn new(s: &str) -> Self {
		Self(StringName::from(s))
	}

	pub fn get(&self) -> StringName {
		self.0.clone()
	}
}

impl From<&'static str> for ConstStringName {
	fn from(s: &'static str) -> Self {
		Self::new(s)
	}
}

#[macro_export]
macro_rules! lazy_gstring {
    ($visibility: vis $var_name: ident = $value: literal) => {
	    $visibility static $var_name: std::sync::LazyLock<$crate::lazy_string::ConstGString> = 
	        std::sync::LazyLock::new(|| $crate::lazy_string::ConstGString::new($value)); 
    };
}

#[macro_export]
macro_rules! lazy_stringname {
    ($visibility: vis $var_name: ident = $value: literal) => {
	    $visibility static $var_name: std::sync::LazyLock< $crate::lazy_string::ConstStringName> = 
	        std::sync::LazyLock::new(|| $crate::lazy_string::ConstStringName::new($value));
    };
}
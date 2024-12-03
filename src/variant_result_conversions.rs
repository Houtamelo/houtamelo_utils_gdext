use anyhow::{Result, bail};

use crate::internal::*;

pub fn are_gds_equal<T: GodotClass>(a: &Gd<T>, b: &Gd<T>) -> bool {
	if a.is_instance_valid() && b.is_instance_valid() {
		a == b
	} else {
		false
	}
}

pub fn variant_as_result<T: FromGodot>(variant: Variant) -> Result<T> {
	if let Ok(ok) = variant.try_to::<T>() {
		Ok(ok)
	} else {
		bail!("{variant:?}")
	}
}

#[macro_export]
macro_rules! try_result_as_variant {
	($Expr:expr) => {
		match $Expr {
			Ok(ok) => ok,
			Err(err) => return format!("{err:?}").to_variant(),
		}
	};
}

#[macro_export]
macro_rules! try_variant {
	($Expr:expr) => {{
		let result = $Expr;
		if let Ok(ok) = result.try_to() {
			ok
		} else {
			return format!("{result:?}").to_variant();
		}
	}};
}

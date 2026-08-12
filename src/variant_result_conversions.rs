use anyhow::{Result, bail};

use crate::internal::*;

pub fn are_gds_equal<A: GodotClass, B: GodotClass>(a: &Gd<A>, b: &Gd<B>) -> bool {
    a.instance_id_unchecked() == b.instance_id_unchecked()
}

pub fn variant_as_result<T: FromGodot>(variant: Variant) -> Result<T> {
    if let Ok(ok) = variant.try_to::<T>() { Ok(ok) } else { bail!("{variant:?}") }
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

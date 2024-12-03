#![feature(macro_metavar_expr_concat)]

mod audio;
mod auto_text_resize;
mod connect;
mod disallow_click_focus;
mod enums;
mod lazy_string;
mod loading;
mod log_if_err;
mod modulate_on_hover;
mod node_extensions;
mod required_gd;
mod try_var_at;
mod variant_result_conversions;

pub mod prelude {
	pub use crate::{
		audio::*,
		auto_text_resize::*,
		connect::{
			UnsafeCallable,
			connect_child::ConnectChild,
			connect_deferred::ConnectDeferred,
			connect_with_deferred::ConnectWithDeferred,
		},
		define_gdscript_rust_enum,
		disallow_click_focus::*,
		export_rust_enum,
		lazy_gstring,
		lazy_string::{ConstGString, ConstStringName},
		lazy_stringname,
		loading::*,
		log_if_err::{LogIfErr, OkLogErr},
		modulate_on_hover::ModulateOnHover,
		node_extensions::*,
		required_gd::*,
		try_result_as_variant,
		try_var_at::*,
		try_variant,
		variant_result_conversions::{are_gds_equal, variant_as_result},
	};
}

mod internal {
	pub(crate) use godot::{
		classes::{object::ConnectFlags, *},
		global::Error as GodotError,
		prelude::*,
	};
}

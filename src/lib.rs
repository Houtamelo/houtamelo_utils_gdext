mod audio;
mod auto_text_resize;
mod connect;
mod disallow_click_focus;
mod lazy_string;
mod loading;
mod log_if_err;
mod modulate_on_hover;
mod node_extensions;
mod required_gd;
mod try_var_at;

pub mod prelude {
	pub use crate::{
		audio::*,
		auto_text_resize::*,
		connect::{
			connect_child::ConnectChild,
			connect_deferred::ConnectDeferred,
			connect_with_deferred::ConnectWithDeferred,
		},
		disallow_click_focus::*,
		lazy_gstring,
		lazy_string::{ConstGString, ConstStringName},
		lazy_stringname,
		loading::*,
		log_if_err::{LogIfErr, OkLogErr},
		modulate_on_hover::ModulateOnHover,
		node_extensions::*,
		required_gd::*,
		try_var_at::*,
	};
}

mod internal {
	pub(crate) use godot::{
		classes::{object::ConnectFlags, *},
		global::Error as GodotError,
		prelude::*,
	};
}

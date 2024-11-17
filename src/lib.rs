mod disallow_click_focus;
mod audio;
mod auto_text_resize;
mod loading;
mod connect_deferred;
mod connect_child;
mod connect_with_deferred;
mod node_extensions;
mod lazy_string;
mod modulate_on_hover;
mod log_if_err;

pub mod prelude {
	pub use crate::audio::*;
	pub use crate::auto_text_resize::*;
	pub use crate::disallow_click_focus::*;
	pub use crate::loading::*;
	pub use crate::node_extensions::*;
	pub use crate::connect_deferred::ConnectDeferred;
	pub use crate::connect_child::ConnectChild;
	pub use crate::connect_with_deferred::ConnectWithDeferred;
	pub use crate::lazy_string::{ConstStringName, ConstGString};
	pub use crate::{lazy_gstring, lazy_stringname};
	pub use crate::log_if_err::LogIfErr;
	pub use crate::modulate_on_hover::ModulateOnHover;
}

mod internal {
	pub(crate) use godot::classes::*;
	pub(crate) use godot::classes::object::ConnectFlags;
	pub(crate) use godot::prelude::*;
}
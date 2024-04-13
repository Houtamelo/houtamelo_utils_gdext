#![feature(let_chains)]
#![allow(unused_imports)]
#![feature(auto_traits)]
#![feature(negative_impls)]

mod disallow_click_focus;
mod audio;
mod auto_text_resize;

pub mod prelude {
	pub use godot::engine::*;
	pub use godot::prelude::*;

	pub use crate::audio::*;
	pub use crate::auto_text_resize::*;
	pub use crate::disallow_click_focus::*;
}
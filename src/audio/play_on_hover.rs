use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use util::prelude::*;

use crate::prelude::*;
use crate::prelude::object::ConnectFlags;

#[derive(GodotClass)]
#[class(init, base = AudioStreamPlayer2D)]
pub struct PlayOnHoverAndPitchRandomizer {
	base: Base<AudioStreamPlayer2D>,
	original_pitch: f32,
}

#[godot_api]
impl IAudioStreamPlayer2D for PlayOnHoverAndPitchRandomizer {
	fn ready(&mut self) {
		self.original_pitch = self.base().get_pitch_scale();
		
		let self_gd = self.to_gd();
		
		let Some(parent) = &mut self.base().get_parent()
			else { return godot_error!("{}(): Node `{}` has no parent", full_fn_name(&Self::ready), self.base().get_name()) };

		if parent.has_signal("mouse_entered".into()) {
			parent.connect_ex("mouse_entered".into(), Callable::from_object_method(&self_gd, fn_name(&Self::_play_custom)))
				  .flags(ConnectFlags::DEFERRED.ord() as u32)
				  .done();
		} else {
			godot_warn!("ready():\n\
			Node `{}` cannot connect to it's parent `{}`\n\
			Parent does not have signal `mouse_entered`.",
				self.base().get_name(), parent.get_name());
		}

		if parent.has_signal("focus_entered".into()) {
			parent.connect_ex("focus_entered".into(), Callable::from_object_method(&self_gd, fn_name(&Self::_play_custom)))
				  .flags(ConnectFlags::DEFERRED.ord() as u32)
			      .done();
		}
	}
}


#[godot_api]
impl PlayOnHoverAndPitchRandomizer {
	#[func]
	fn _play_custom(&mut self) {
		let mut rng = Xoshiro256PlusPlus::from_entropy();
		let pitch = self.original_pitch * (0.9 + rng.gen_range(0.0..=0.2));
		
		let mut base = self.base_mut();
		base.set_pitch_scale(pitch);
		base.play();
	}
}
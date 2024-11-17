use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use crate::internal::*;

#[derive(GodotClass)]
#[class(init, base=AudioStreamPlayer2D)]
pub struct PitchRandomizer {
	base: Base<AudioStreamPlayer2D>,
	original_pitch: f32,
}

#[godot_api]
impl IAudioStreamPlayer2D for PitchRandomizer {
	fn ready(&mut self) {
		self.original_pitch = self.base().get_pitch_scale();
	}
}

#[godot_api]
impl PitchRandomizer {
	#[func]
	pub fn _play_custom(&mut self) {
		let mut rng = Xoshiro256PlusPlus::from_entropy();
		let pitch = self.original_pitch * (0.9 + rng.gen_range(0.0..=0.2));
		
		let mut base = self.base_mut();
		base.set_pitch_scale(pitch);
		base.play();
	}
}
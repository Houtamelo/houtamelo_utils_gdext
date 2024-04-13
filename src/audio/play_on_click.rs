use godot::engine::object::ConnectFlags;
use godot::sys::Global;
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use util::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(init, base = AudioStreamPlayer2D)]
pub struct PlayOnClickAndPitchRandomizer {
	base: Base<AudioStreamPlayer2D>,
	original_pitch: f32,
}

#[godot_api]
impl IAudioStreamPlayer2D for PlayOnClickAndPitchRandomizer {
	fn ready(&mut self) {
		self.original_pitch = self.base().get_pitch_scale();

		let Some(parent) = &mut self.base().get_parent()
			else { return godot_error!("{}(): Node `{}` has no parent", full_fn_name(&Self::ready), self.base().get_name()) };

		let self_gd = self.to_gd();

		if parent.has_signal("pressed".into()) {
			parent.connect_ex("pressed".into(), Callable::from_object_method(&self_gd, fn_name(&Self::_on_pressed)))
				  .flags(ConnectFlags::DEFERRED.ord() as u32)
				  .done();
		} else if parent.has_signal("gui_input".into()) {
			parent.connect_ex("gui_input".into(), Callable::from_object_method(&self_gd, fn_name(&Self::_on_gui_input)))
				  .flags(ConnectFlags::DEFERRED.ord() as u32)
				  .done();
		} else if parent.has_signal("input_event".into()) {
			parent.connect_ex("input_event".into(), Callable::from_object_method(&self_gd, fn_name(&Self::_on_input_event)))
				  .flags(ConnectFlags::DEFERRED.ord() as u32)
				  .done();
		} else {
			godot_warn!("ready():\n\
			 Node `{}` cannot connect to it's parent `{}`\n\
			 Parent does not have any of these signals: `gui_input` | `pressed` | `input_event`",
				self.base().get_name(), parent.get_name());
		}
	}
}

fn is_confirm_input(event: Gd<InputEvent>) -> bool {
	if event.is_action_pressed("ui_accept".into()) {
		return true;
	}

	if event.try_cast::<InputEventMouseButton>().ok().is_some_and(|mouse_event|
		mouse_event.is_pressed() && mouse_event.get_button_index() == global::MouseButton::LEFT) {
		return true;
	}

	return false;
}

#[godot_api]
impl PlayOnClickAndPitchRandomizer {
	#[func]
	fn _on_gui_input(&mut self, event: Gd<InputEvent>) {
		if is_confirm_input(event) {
			self._play_custom();
		}
	}

	#[func]
	fn _on_input_event(&mut self, _viewport: Gd<Node>, event: Gd<InputEvent>, _shape_idx: i64) {
		if is_confirm_input(event) {
			self._play_custom();
		}
	}

	#[func]
	fn _on_pressed(&mut self) {
		self._play_custom();
	}

	#[func]
	fn _play_custom(&mut self) {
		let mut rng = Xoshiro256PlusPlus::from_entropy();
		let pitch = self.original_pitch * (0.9 + rng.gen_range(0.0..=0.2));
		
		let base_mut = &mut self.base_mut();
		base_mut.set_pitch_scale(pitch);
		base_mut.play();
	}
}

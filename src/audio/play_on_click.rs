use godot::classes::object::ConnectFlags;
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

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
		let mut base = self.base_mut();

		let mut tween =
			base.create_tween()
			    .unwrap();

		tween.tween_property(
			base.clone().upcast(),
			NodePath::from("position"),
			Variant::from(Vector2::new(50., 50.)),
			0.5,
		)
		     .unwrap()
		     .connect("finished".into(), base.callable("random_func"));

		let initial_pitch = base.get_pitch_scale();
		drop(base);

		self.original_pitch = initial_pitch;

		let base = self.base();

		let Some(mut parent) = base.get_parent()
		else { return godot_error!("PlayOnClickAndPitchRandomizer::ready(): Node `{}` has no parent", base.get_name()) };

		let self_gd = self.to_gd();

		if parent.has_signal("pressed".into()) {
			parent.connect_ex("pressed".into(), Callable::from_object_method(&self_gd, "_on_pressed"))
			      .flags(ConnectFlags::DEFERRED.ord() as u32)
			      .done();
		} else if parent.has_signal("gui_input".into()) {
			parent.connect_ex("gui_input".into(), Callable::from_object_method(&self_gd, "_on_gui_input"))
			      .flags(ConnectFlags::DEFERRED.ord() as u32)
			      .done();
		} else if parent.has_signal("input_event".into()) {
			parent.connect_ex("input_event".into(), Callable::from_object_method(&self_gd, "_on_input_event"))
			      .flags(ConnectFlags::DEFERRED.ord() as u32)
			      .done();
		} else {
			godot_warn!("ready():\n\
			 Node `{}` cannot connect to it's parent `{}`\n\
			 Parent does not have any of these signals: `gui_input` | `pressed` | `input_event`",
				base.get_name(), parent.get_name());
		}
	}
}

fn is_confirm_input(event: Gd<InputEvent>) -> bool {
	event.is_action_pressed("ui_accept".into())
		||
		event.try_cast::<InputEventMouseButton>()
		     .ok()
		     .is_some_and(|mouse_event| {
			     mouse_event.is_pressed()
				     && mouse_event.get_button_index() == godot::global::MouseButton::LEFT
		     })
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

		let mut base = self.base_mut();
		base.set_pitch_scale(pitch);
		base.play();
	}
}

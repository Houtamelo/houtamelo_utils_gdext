use std::any::type_name_of_val;

use godot::classes::object::ConnectFlags;
use rand::RngExt;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::internal::*;

#[derive(GodotClass)]
#[class(init, base = AudioStreamPlayer2D)]
pub struct PlayOnClickAndPitchRandomizer {
    base: Base<AudioStreamPlayer2D>,
    original_pitch: f32,
}

#[godot_api]
impl IAudioStreamPlayer2D for PlayOnClickAndPitchRandomizer {
    fn ready(&mut self) {
        let initial_pitch = self.get_pitch_scale();
        self.original_pitch = initial_pitch;
        let Some(mut parent) = self.get_parent() else {
            return godot_error!("PlayOnClickAndPitchRandomizer::ready(): Node `{}` has no parent", self.get_name())
        };

        let self_gd = self.to_gd();

        if parent.has_signal("pressed") {
            parent.connect_flags(
                "pressed",
                &Callable::from_object_method(&self_gd, type_name_of_val(&Self::_on_pressed)),
                ConnectFlags::DEFERRED,
            );
        } else if parent.has_signal("gui_input") {
            parent.connect_flags(
                "gui_input",
                &Callable::from_object_method(&self_gd, type_name_of_val(&Self::_on_gui_input)),
                ConnectFlags::DEFERRED,
            );
        } else if parent.has_signal("input_event") {
            parent.connect_flags(
                "input_event",
                &Callable::from_object_method(&self_gd, type_name_of_val(&Self::_on_input_event)),
                ConnectFlags::DEFERRED,
            );
        } else {
            godot_warn!(
                "ready():\nNode `{}` cannot connect to it's parent `{}`\nParent does not have any of these signals: \
                 `gui_input` | `pressed` | `input_event`",
                self.get_name(),
                parent.get_name()
            );
        }
    }
}

fn is_confirm_input(event: Gd<InputEvent>) -> bool {
    event.is_action_pressed("ui_accept")
        || event
            .try_cast::<InputEventMouseButton>()
            .ok()
            .is_some_and(|mouse_event| {
                mouse_event.is_pressed() && mouse_event.get_button_index() == godot::global::MouseButton::LEFT
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
    fn _on_pressed(&mut self) { self._play_custom(); }

    #[func]
    fn _play_custom(&mut self) {
        let mut rng = rand::make_rng::<Xoshiro256PlusPlus>();
        let pitch = self.original_pitch * (0.9 + rng.random_range(0.0..=0.2));

        let mut base = self.base_mut();
        base.set_pitch_scale(pitch);
        base.play();
    }
}

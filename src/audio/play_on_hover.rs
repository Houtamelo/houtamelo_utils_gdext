use std::any::type_name_of_val;

use rand::RngExt;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::internal::*;

#[derive(GodotClass)]
#[class(init, base = AudioStreamPlayer2D)]
pub struct PlayOnHoverAndPitchRandomizer {
    base: Base<AudioStreamPlayer2D>,
    original_pitch: f32,
}

#[godot_api]
impl IAudioStreamPlayer2D for PlayOnHoverAndPitchRandomizer {
    fn ready(&mut self) {
        self.original_pitch = self.get_pitch_scale();

        let self_gd = self.to_gd();

        let Some(mut parent) = self.get_parent() else {
            return godot_error!("PlayOnHoverAndPitchRandomizer::ready(): Node `{}` has no parent", self.get_name())
        };

        if parent.has_signal("mouse_entered") {
            parent.connect_flags(
                "mouse_entered",
                &Callable::from_object_method(&self_gd, type_name_of_val(&Self::_play_custom)),
                ConnectFlags::DEFERRED,
            );
        } else {
            godot_warn!(
                "ready():\nNode `{}` cannot connect to it's parent `{}`\nParent does not have signal `mouse_entered`.",
                self.get_name(),
                parent.get_name()
            );
        }

        if parent.has_signal("focus_entered") {
            parent.connect_flags(
                "focus_entered",
                &Callable::from_object_method(&self_gd, type_name_of_val(&Self::_play_custom)),
                ConnectFlags::DEFERRED,
            );
        }
    }
}

#[godot_api]
impl PlayOnHoverAndPitchRandomizer {
    #[func]
    fn _play_custom(&mut self) {
        let mut rng = rand::make_rng::<Xoshiro256PlusPlus>();
        let pitch = self.original_pitch * (0.9 + rng.random_range(0.0..=0.2));

        let mut base = self.base_mut();
        base.set_pitch_scale(pitch);
        base.play();
    }
}

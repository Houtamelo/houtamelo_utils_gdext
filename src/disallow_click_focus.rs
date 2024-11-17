use crate::prelude::*;
use crate::prelude::object::ConnectFlags;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct DisallowClickFocusOnParent {
	base: Base<Node>,
}

fn is_mouse_pressed(event: Gd<InputEvent>) -> bool {
	event.try_cast::<InputEventMouseButton>()
	     .ok()
	     .is_some_and(|event| {
		     event.is_pressed()
	     })
}

#[godot_api]
impl INode for DisallowClickFocusOnParent {
	fn ready(&mut self) {
		let base = self.base();
		let Some(mut parent) = base.get_parent()
		else { return godot_error!("DisallowClickFocusOnParent::ready(): Node `{}` has no parent", base.get_name()) };

		let self_gd = self.to_gd();

		if parent.has_signal("pressed") {
			parent.connect_ex("pressed", &Callable::from_object_method(&self_gd, "_on_pressed"))
			      .flags(ConnectFlags::DEFERRED.ord() as u32)
			      .done();
		} else if parent.has_signal("gui_input") {
			parent.connect_ex("gui_input", &Callable::from_object_method(&self_gd, "_on_gui_input"))
			      .flags(ConnectFlags::DEFERRED.ord() as u32)
			      .done();
		} else if parent.has_signal("input_event") {
			parent.connect_ex("input_event", &Callable::from_object_method(&self_gd, "_on_input_event"))
			      .flags(ConnectFlags::DEFERRED.ord() as u32)
			      .done();
		} else {
			godot_warn!("DisallowClickFocusOnParent::ready():\n\
			 Node `{}` cannot connect to it's parent `{}`\n\
			 Parent does not have any of these signals: `gui_input` | `pressed` | `input_event`",
				base.get_name(), parent.get_name());
		}
	}
}

#[godot_api]
impl DisallowClickFocusOnParent {
	fn release_parent_focus(&self) {
		let base = self.base();
		let Some(mut parent) = base.get_parent()
		else { return };

		if parent.has_method("release_focus") {
			parent.call_deferred("release_focus", &[]);
		} else {
			godot_warn!("DisallowClickFocusOnParent::release_parent_focus():\n\
			 Node `{}` cannot release focus from it's parent `{}`\n\
			 Parent does not have method `release_focus`",
				base.get_name(), parent.get_name());
		}
	}

	#[func]
	fn _on_gui_input(&self, event: Gd<InputEvent>) {
		if is_mouse_pressed(event) {
			self.release_parent_focus();
		}
	}

	#[func]
	fn _on_input_event(&self, _viewport: Gd<Node>, event: Gd<InputEvent>, _shape_idx: i64) {
		if is_mouse_pressed(event) {
			self.release_parent_focus();
		}
	}

	#[func]
	fn _on_pressed(&self) {
		self.release_parent_focus();
	}
}


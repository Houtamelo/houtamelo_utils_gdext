use util::prelude::*;

use crate::prelude::*;
use crate::prelude::text_server::AutowrapMode;

#[derive(GodotClass)]
#[class(init, base = Label)]
pub struct AutoTextResize {
	base: Base<Label>,
	#[export]
	#[var(set = set_min_size)]
	min_size: i32,
	#[export]
	#[var(set = set_max_size)]
	max_size: i32,
}

#[godot_api]
impl ILabel for AutoTextResize {
	fn ready(&mut self) {
		let mut base = self.base_mut();
		
		if base.get_max_lines_visible() <= 0 {
			base.set_max_lines_visible(1);
			godot_warn!(
				"{}: max_lines_visible is not set to a positive value. Defaulting to 1.\n\
				 Object: {}", fn_name(&AutoTextResize::ready), base.get_name());
		}

		if base.get_autowrap_mode() == AutowrapMode::OFF {
			base.set_autowrap_mode(AutowrapMode::WORD);
			godot_warn!(
				"{}: autowrap is set to OFF. Overriding to WORD.\n\
				 Object: {}", fn_name(&AutoTextResize::ready), base.get_name());
		}
		
		drop(base);
		self.update_font_size();
	}

	fn set_property(&mut self, property: StringName, value: Variant) -> bool {
		return match property.to_string().as_str() {
			"min_size" => {
				self.set_min_size(value);
				true
			},
			"max_size" => {
				self.set_max_size(value);
				true
			},
			"text" => {
				self.set_text(value);
				true
			},
			_ => false,
		};
	}
}

#[godot_api]
impl AutoTextResize {
	#[func]
	fn set_min_size(&mut self, value: Variant) {
		if let Ok(size) = value.try_to::<i32>() {
			self.min_size = size;
			self.update_font_size();
		} else {
			godot_warn!(
				"{}: Failed to convert value to i64.\n\
				 Value: {}\n\
				 Object: {}", fn_name(&AutoTextResize::set_min_size), value.to_string(), self.base().get_name());
		}
	}

	#[func]
	fn set_max_size(&mut self, value: Variant) {
		if let Ok(size) = value.try_to::<i32>() {
			self.max_size = size;
			self.update_font_size();
		} else {
			godot_warn!(
				"{}: Failed to convert value to i64.\n\
				 Value: {}\n\
				 Object: {}", fn_name(&AutoTextResize::set_max_size), value.to_string(), self.base().get_name());
		}
	}

	#[func]
	fn set_text(&mut self, value: Variant) {
		if let Ok(text) = value.try_to::<GString>() {
			let mut base = self.base_mut();
			base.set_text(text);
			base.set_clip_text(true);
			drop(base);
			
			self.update_font_size();
		} else {
			godot_warn!(
				"{}: Failed to convert value to GodotString.\n\
				 Value: {}\n\
				 Object: {}", fn_name(&AutoTextResize::set_text), value.to_string(), self.base().get_name());
		}
	}
	
	fn update_font_size(&mut self) {
		let min_size = self.min_size;
		let max_size = self.max_size;
		
		let base = &mut self.base_mut();
		base.add_theme_font_size_override("font_size".into(), max_size);

		let mut font_size = max_size;

		while base.get_visible_line_count() < base.get_line_count()
			&& font_size > min_size {
			font_size -= 1;
			base.add_theme_font_size_override("font_size".into(), font_size);
		}
	}
}
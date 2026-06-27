use text_server::AutowrapMode;

use crate::internal::*;

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
                "AutoTextResize::ready(): max_lines_visible is not set to a positive value. Defaulting to 1.\nObject: \
                 {}",
                base.get_name()
            );
        }

        if base.get_autowrap_mode() == AutowrapMode::OFF {
            base.set_autowrap_mode(AutowrapMode::WORD);
            godot_warn!(
                "AutoTextResize::ready(): autowrap is set to OFF. Overriding to WORD.\nObject: {}",
                base.get_name()
            );
        }

        drop(base);
        self.update_font_size();
    }

    fn on_set(&mut self, property: StringName, value: Variant) -> bool {
        match property.to_string().as_str() {
            "min_size" => {
                if let Ok(value) = value.try_to::<i32>() {
                    self.set_min_size(value);
                }

                true
            }
            "max_size" => {
                if let Ok(value) = value.try_to::<i32>() {
                    self.set_max_size(value);
                }

                true
            }
            "text" => {
                if let Ok(value) = value.try_to::<GString>() {
                    self.set_text(value);
                }

                true
            }
            _ => false,
        }
    }
}

#[godot_api]
impl AutoTextResize {
    #[func]
    fn set_min_size(&mut self, size: i32) {
        self.min_size = size;
        self.update_font_size();
    }

    #[func]
    fn set_max_size(&mut self, size: i32) {
        self.max_size = size;
        self.update_font_size();
    }

    #[func]
    fn set_text(&mut self, text: GString) {
        let mut base = self.base_mut();
        base.set_text(&text);
        base.set_clip_text(true);
        drop(base);

        self.update_font_size();
    }

    fn update_font_size(&mut self) {
        let min_size = self.min_size;
        let max_size = self.max_size;

        let mut base = self.base_mut();
        base.add_theme_font_size_override("font_size", max_size);

        let mut font_size = max_size;

        while base.get_visible_line_count() < base.get_line_count() && font_size > min_size {
            font_size -= 1;
            base.add_theme_font_size_override("font_size", font_size);
        }
    }
}

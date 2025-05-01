use crate::internal::*;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct ModulateOnHover {
    base: Base<Control>,
    #[export]
    #[init(val = Color::from_rgb(0.5, 0.5, 0.5))]
    normal_color: Color,
    #[export]
    #[init(val = Color::from_rgb(1.0, 1.0, 1.0))]
    hover_color: Color,
}

#[godot_api]
impl IControl for ModulateOnHover {
    fn ready(&mut self) {
        self.signals()
            .mouse_entered()
            .connect_self(|this: &mut ModulateOnHover| {
                let color = this.hover_color;
                this.base_mut().set_modulate(color);
            });

        self.signals()
            .mouse_exited()
            .connect_self(|this: &mut ModulateOnHover| {
                let color = this.normal_color;
                this.base_mut().set_modulate(color);
            });
    }
}

#[godot_api]
impl ModulateOnHover {}

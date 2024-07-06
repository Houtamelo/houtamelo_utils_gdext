pub trait LogIfErr {
	fn log_if_err(self);
}

impl<T, E: std::fmt::Debug> LogIfErr for Result<T, E> {
	fn log_if_err(self) {
		if let Err(e) = self {
			godot::prelude::godot_error!("{e:?}");
		}
	}
}

impl LogIfErr for godot::global::Error {
	fn log_if_err(self) {
		match self {
			godot::global::Error::OK => {}
			err => {
				godot::prelude::godot_error!("{err:?}");
			}
		}
	}
}
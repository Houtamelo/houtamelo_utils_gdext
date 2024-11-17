pub trait LogIfErr {
	fn log_if_err(self) -> bool;
}

impl<T, E: std::fmt::Debug> LogIfErr for Result<T, E> {
	fn log_if_err(self) -> bool {
		if let Err(e) = self {
			godot::prelude::godot_error!("{e:?}");
			true
		} else {
			false
		}
	}
}

impl LogIfErr for godot::global::Error {
	fn log_if_err(self) -> bool {
		match self {
			godot::global::Error::OK => false,
			err => {
				godot::prelude::godot_error!("{err:?}");
				true
			}
		}
	}
}
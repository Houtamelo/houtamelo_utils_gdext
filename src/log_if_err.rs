use crate::internal::*;
use std::fmt::Debug;

pub trait LogIfErr {
	fn log_if_err(self) -> bool;
}

impl<T, E: Debug> LogIfErr for Result<T, E> {
	fn log_if_err(self) -> bool {
		if let Err(e) = self {
			godot_error!("{e:?}");
			true
		} else {
			false
		}
	}
}

impl LogIfErr for GodotError {
	fn log_if_err(self) -> bool {
		match self {
			GodotError::OK => false,
			err => {
				godot_error!("{err:?}");
				true
			}
		}
	}
}

pub trait OkLogErr<T> {
	fn ok_log_err(self) -> Option<T>;
}

impl<T, E: Debug> OkLogErr<T> for Result<T, E> {
	fn ok_log_err(self) -> Option<T> {
		match self {
			Ok(ok) => Some(ok),
			Err(err) => {
				godot_error!("{err:?}");
				None
			}
		}
	}
}
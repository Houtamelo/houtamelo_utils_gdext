#[macro_export]
macro_rules! match_gd {
	($gd: expr,
		$V1: ident @ $T1: ty => $F1: expr
		$(, $( $left: tt )* )?
	) => { '__match_return: {
		let __gd = $gd;

		if let Ok(mut $V1) = __gd.clone().try_cast::<$T1>() {
			break '__match_return ($F1)
		}

		$( $crate::munch_gd!(@MUNCH '__match_return, __gd, $( $left )* ); )?
	}};
}

#[macro_export]
#[doc(hidden)]
macro_rules! munch_gd {
    (@MUNCH
		$label: lifetime,
		$gd: expr,
		$V: ident @ $T: ty => $F: expr
		$(, $( $left: tt )* )?
	) => {
		if let Ok(mut $V) = $gd.clone().try_cast::<$T>() {
			break $label ($F)
		}

		$( $crate::munch_gd!(@MUNCH $label, $gd, $( $left )*); )?
	};

	(@MUNCH
		$label: lifetime,
		$gd: expr,
		$V_else: ident => $F_else: expr $(,)?
	) => {
		let $V_else = $gd;
		break $label ($F_else)
	};

	(@MUNCH $label: lifetime, $gd: expr, ) => {};
}

#[macro_export]
macro_rules! match_var {
	($var: expr,
		$V1: ident @ $T1: ty => $F1: expr
		$(, $( $left: tt )* )?
	) => { '__match_return: {
		let __var = $var;

		if let Ok(mut $V1) = __var.try_to::<$T1>() {
			break '__match_return ($F1)
		}

		$( $crate::munch_var!(@MUNCH '__match_return, __var, $( $left )* ); )?
	}};
}

#[macro_export]
#[doc(hidden)]
macro_rules! munch_var {
    (@MUNCH
		$label: lifetime,
		$var: expr,
		$V: ident @ $T: ty => $F: expr
		$(, $( $left: tt )* )?
	) => {
		if let Ok(mut $V) = $var.try_to::<$T>() {
			break $label ($F)
		}

		$( $crate::munch_var!(@MUNCH $label, $var, $( $left )*); )?
	};

	(@MUNCH
		$label: lifetime,
		$var: expr,
		$V_else: ident => $F_else: expr $(,)?
	) => {
		let $V_else = $var;
		break $label ($F_else)
	};

	(@MUNCH $label: lifetime, $gd: expr, ) => {};
}

#[allow(unused)]
#[cfg(test)]
mod tests {
	use crate::internal::*;

	fn test(gd: Gd<Node>) {
		match_gd!(gd.clone(),
			node2d @ Node2D => {
				godot_print!("{:?}", node2d.get_position())
			},
			node3d @ Node3D => {
				godot_print!("{:?}", node3d.get_rotation())
			},
			sprite @ Sprite2D => godot_print!("{}", sprite.get_name()),
			node => godot_print!("{:?}", node.get_name()),
		);

		let value = match_gd!(gd.clone(),
			node2d @ Node2D => node2d.get_position(),
			node3d @ Node3D => {
				let Vector3 { x, y, .. } = node3d.get_position();
				Vector2 { x, y }
			},
			sprite @ Sprite2D => sprite.get_position(),
			node => {
				godot_print!("{:?}", node.get_name());
				Vector2::ZERO
			},
		);

		let var = gd.to_variant();
		match_var!(var.clone(),
			node2d @ Gd<Node2D> => {
				godot_print!("{:?}", node2d.get_position());
			},
			node3d @ Gd<Node3D> => {
				godot_print!("{:?}", node3d.get_rotation());
			},
			sprite @ Gd<Sprite2D> => godot_print!("{}", sprite.get_name()),
			other => godot_print!("{other:?}"),
		);

		let value = match_var!(var,
			node2d @ Gd<Node2D> => node2d.get_position(),
			node3d @ Gd<Node3D> => {
				let Vector3 { x, y, .. } = node3d.get_position();
				Vector2 { x, y }
			},
			other => Vector2::ZERO,
		);
	}
}

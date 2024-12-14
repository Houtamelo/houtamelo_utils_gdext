#[macro_export]
macro_rules! match_gd {
	($gd: expr,
		$T: ty: $P: pat => $F: expr
		$(, $( $left: tt )* )?
	) => {
		'__match_return: {
			let __gd = $gd;

			if let Ok($P) = __gd.clone().try_cast::<$T>() {
				break '__match_return ($F)
			}

			$( $crate::munch_gd!(@MUNCH '__match_return, __gd, $( $left )* ); )?
		}
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! munch_gd {
	(@MUNCH
		$label: lifetime,
		$gd: ident,
		$T: ty: $P: pat => $F: expr
		$(, $( $left: tt )* )?
	) => {
		if let Ok($P) = $gd.clone().try_cast::<$T>() {
			break $label ($F)
		}

		$( $crate::munch_gd!(@MUNCH $label, $gd, $( $left )*); )?
	};

	(@MUNCH
		$label: lifetime,
		$gd: ident,
		$P: pat => $F_else: expr $(,)?
	) => {
		let $P = $gd;
		break $label ($F_else)
	};

	(@MUNCH $label: lifetime, $gd: expr, ) => {};
}

#[macro_export]
macro_rules! match_var {
	($var: expr,
	$T: ident { $($P: tt)* } => $F: expr
	$(, $( $left: tt )* )?
	) => {
		'__match_return: {
			let __var = $var;

			if let Ok($T { $($P)* }) = __var.try_to::<$T>() {
				break '__match_return ($F)
			}

			$( $crate::munch_var!(@MUNCH '__match_return, __var, $( $left )* ); )?
		}
	};

	($var: expr,
		$T: ty: $P: pat => $F: expr
		$(, $( $left: tt )* )?
	) => {
		'__match_return: {
			let __var = $var;

			if let Ok($P) = __var.try_to::<$T>() {
				break '__match_return ($F)
			}

			$( $crate::munch_var!(@MUNCH '__match_return, __var, $( $left )* ); )?
		}
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! munch_var {
	(@MUNCH
		$label: lifetime,
		$var: ident,
		$T: ident { $($P: tt)* } => $F: expr
		$(, $( $left: tt )* )?
	) => {
		if let Ok($T { $($P)* }) = $var.try_to::<$T>() {
			break $label ($F)
		}

		$( $crate::munch_var!(@MUNCH $label, $var, $( $left )*); )?
	};

	(@MUNCH
		$label: lifetime,
		$var: ident,
		$T: ty: $P: pat => $F: expr
		$(, $( $left: tt )* )?
	) => {
		if let Ok($P) = $var.try_to::<$T>() {
			break $label ($F)
		}

		$( $crate::munch_var!(@MUNCH $label, $var, $( $left )*); )?
	};

	(@MUNCH
		$label: lifetime,
		$var: ident,
		$P: pat => $F_else: expr $(,)?
	) => {
		let $P = $var;
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
			Node2D: node2d => {
				godot_print!("{:?}", node2d.get_position())
			},
			Node3D: mut node3d => {
				godot_print!("{:?}", node3d.get_rotation())
			},
			Sprite2D: sprite => godot_print!("{}", sprite.get_name()),
			AudioStreamPlayer2D: _ => {},
			mut my_node => godot_print!("{:?}", my_node.get_name()),
		);

		match_gd!(gd.clone(),
			Node2D: node2d => {
				godot_print!("{:?}", node2d.get_position())
			},
			Node3D: node3d => {
				godot_print!("{:?}", node3d.get_rotation())
			},
			Sprite2D: sprite => godot_print!("{}", sprite.get_name()),
			my_node => godot_print!("{:?}", my_node.get_name()),
		);

		let value = match_gd!(gd.clone(),
			Node2D: mut node2d => node2d.get_position(),
			Node3D: node3d => {
				let Vector3 { x, y, .. } = node3d.get_position();
				Vector2 { x, y }
			},
			Sprite2D: sprite => sprite.get_position(),
			mut node => {
				godot_print!("{:?}", node.get_name());
				Vector2::ZERO
			},
		);

		let var = gd.to_variant();
		match_var!(var.clone(),
			Gd<Node2D>: mut node2d => {
				godot_print!("{:?}", node2d.get_position());
			},
			Gd<Node3D>: node3d => {
				godot_print!("{:?}", node3d.get_rotation());
			},
			Gd<Sprite2D>: sprite => godot_print!("{}", sprite.get_name()),
			bool: true => godot_print!("{}", true),
			other => godot_print!("{other:?}"),
		);

		let value = match_var!(var,
			i32: ..0 => Vector2::ZERO,
			real: mut my_var => Vector2 { x: my_var, y: my_var },
			Vector3 { x, y, .. } => Vector2 { x, y },
			Gd<Node2D>: node2d => node2d.get_position(),
			Gd<Node3D>: mut node3d => {
				let Vector3 { x, y, .. } = node3d.get_position();
				Vector2 { x, y }
			},
			Vector2 { x, y } => Vector2 { x, y },
			_ => Vector2::ZERO,
		);
	}
}

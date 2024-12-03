#[macro_export]
macro_rules! define_gdscript_rust_enum {
	(
		GDScriptName:
		$gd_ident:ident $(;)? $(#[$enum_meta:meta])*
		$enum_vis:vis enum
		$enum_ident:ident { $($var_ident:ident = $var_value:literal),* $(,)? }
	) => {
		$(#[$enum_meta])*
		#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
		#[repr(i32)]
		$enum_vis enum $enum_ident {
			#[default]
			$( $var_ident = $var_value),*
		}

		$crate::export_rust_enum! {
			GDScriptName: $gd_ident
			$enum_ident { $($var_ident = $var_value),* }
		}
	};
}

#[macro_export]
macro_rules! export_rust_enum {
    (
        GDScriptName: $gd_ident: ident $(;)?

        $enum_ident: ident {
            $( $var_ident: ident = $var_value: literal ),*
            $(,)?
        }
    ) => {
        impl $enum_ident {
            pub const fn as_str(&self) -> &'static str {
	            match self {
                    $( $enum_ident::$var_ident => stringify!($var_ident), )*
                }
            }

	        pub fn iter_variants() -> impl Iterator<Item = $enum_ident> {
                [ $( $enum_ident::$var_ident ),* ].into_iter()
            }
        }

        impl godot::prelude::GodotConvert for $enum_ident {
            type Via = i32;
        }

        impl godot::prelude::ToGodot for $enum_ident {
            type ToVia<'v> = Self::Via;

            fn to_godot(&self) -> i32 {
                match self {
	                $( $enum_ident::$var_ident => $var_value, )*
                }
            }
        }

        impl godot::prelude::FromGodot for $enum_ident {
            fn try_from_godot(v: i32) -> Result<Self, godot::prelude::ConvertError> {
                match v {
                    $( $var_value => Ok($enum_ident::$var_ident), )*
                    _ => Err(godot::prelude::ConvertError::new(format!("Invalid enum repr: {v}"))),
                }
            }
        }

        #[doc(hidden)]
        #[allow(unused)]
        #[allow(non_upper_case_globals)]
        #[allow(non_snake_case)]
        mod ${concat($enum_ident, "_gd_class")} {
            use super::$enum_ident;
            use godot::prelude::*;

            #[derive(GodotClass)]
            #[class(no_init, base = RefCounted, rename = $gd_ident)]
            struct GdEnum;

            const ENUM_MAP: &[(&str, $enum_ident, i32)] = &[
	            $( (stringify!($var_ident), $enum_ident::$var_ident, $var_value), )*
            ];

            #[godot_api]
            impl GdEnum {
                $(
                    #[constant]
                    const $var_ident: i32 = $var_value;
                )*

	            #[func]
	            fn find_key(value: i32) -> Variant {
	                ENUM_MAP.iter().find_map(|(key, _, int)| {
		                if value == *int {
			                Some(key.to_variant())
		                } else {
			                None
		                }
	                }).unwrap_or_else(|| Variant::nil())
                }

	            #[func]
	            fn get_value(key: StringName) -> Variant {
	                let str = key.to_string();

	                ENUM_MAP.iter().find_map(|(enum_key, _, int)| {
		                if str == *enum_key {
			                Some(int.to_variant())
		                } else {
			                None
		                }
	                }).unwrap_or_else(|| Variant::nil())
                }

	            #[func]
	            fn has(key: StringName) -> bool {
	                let str = key.to_string();

	                ENUM_MAP.iter().any(|(enum_key, _, _)| {
		                str == *enum_key
	                })
                }

	            #[func]
	            fn has_value(value: i32) -> bool {
	                ENUM_MAP.iter().any(|(_, _, int)| {
		                value == *int
	                })
                }

	            #[func]
	            fn has_all(keys: Array<StringName>) -> bool {
	                keys.iter_shared().all(Self::has)
                }

	            #[func]
	            fn is_empty() -> bool {
	                ENUM_MAP.is_empty()
                }

	            #[func]
	            fn is_read_only() -> bool {
	                true
                }

	            #[func]
	            fn keys() -> Array<StringName> {
	                Array::from_iter(ENUM_MAP.iter().map(|(key, _, _)| StringName::from(*key)))
                }

	            #[func]
	            fn values() -> Array<i32> {
	                Array::from_iter(ENUM_MAP.iter().map(|(_, _, int)| *int))
                }
            }
        }
    }
}

#[allow(unused)]
#[cfg(test)]
mod must_compile {
	pub enum ExportTest {
		Giraffe = 5,
		Fish = 2,
	}

	export_rust_enum! {
		GDScriptName: TestEnum;

		ExportTest {
			Giraffe = 5,
			Fish = 2,
		}
	}

	define_gdscript_rust_enum! {
		GDScriptName: TestEnum;

		pub enum Test {
			Dog = 0,
			Cat = 2,
		}
	}
}

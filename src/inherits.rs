/*
use crate::prelude::*;

pub trait Inherits<T: GodotObject> {
	unsafe fn base<Base: GodotObject>(&self) -> Ref<Base> where T: SubClass<Base>;
}

impl<TSelf, Origin, Inherited> Inherits<Inherited> for TSelf
	where TSelf: FlexCast<Inner = Origin>,
	      Origin: GodotObject + SubClass<Inherited>,
	      Inherited: GodotObject {
	unsafe fn base<Base: GodotObject>(&self) -> Ref<Base> where Inherited: SubClass<Base> {
		self.flex_cast::<Inherited>().flex_cast::<Base>()
	}
}

pub trait FlexCast {
	type Inner: GodotObject;

	fn flex_cast<'a, TDom: GodotObject>(&'a self) -> Ref<TDom> where Self::Inner: 'a + SubClass<TDom> {
		unsafe {
			let inner = self.inner();
			Ref::<TDom>::from_sys(std::ptr::NonNull::new_unchecked(inner.as_ptr()))
		}
	}

	fn inner(&self) -> Ref<Self::Inner>;
}

auto trait NotRef {}
impl<T> !NotRef for Ref<T> {}
impl<T> !NotRef for TRef<'_, T> {}
impl<T> !NotRef for Instance<T> {}
impl<T> !NotRef for TInstance<'_, T> {}

impl<T: GodotObject + NotRef> FlexCast for T {
	type Inner = T;

	fn inner(&self) -> Ref<Self::Inner> {
		unsafe { self.assume_shared() }
	}
}

impl<T: GodotObject> FlexCast for Ref<T> {
	type Inner = T;

	fn inner(&self) -> Ref<Self::Inner> {
		self.clone()
	}
}

impl<T: GodotObject> FlexCast for TRef<'_, T> {
	type Inner = T;

	fn inner(&self) -> Ref<Self::Inner> {
		unsafe { self.assume_shared() }
	}
}

impl<TUser: NativeClass<Base: GodotObject>> FlexCast for Instance<TUser> {
	type Inner = TUser::Base;

	fn inner(&self) -> Ref<Self::Inner> {
		self.clone().into_base()
	}
}

impl<TUser: NativeClass<Base: GodotObject>> FlexCast for TInstance<'_, TUser> {
	type Inner = TUser::Base;

	fn inner(&self) -> Ref<Self::Inner> {
		unsafe { self.base().assume_shared() }
	}
}

#[allow(unused)]
unsafe fn please_compile<T: NativeClass<Base: SubClass<Node>>>(
	node: &CanvasItem,
	node_tref: TRef<CanvasItem>,
	node_ref: Ref<Node>,
	instance: &Instance<T>,
	t_instance: TInstance<T>) {
	let n = node.flex_cast::<Node>();
	let n_tref = node_tref.flex_cast::<Node>();
	let n_ref = node_ref.flex_cast::<Node>();
	
	test(instance);
	test(&t_instance);
}

#[allow(unused)]
unsafe fn test(obj: &impl Inherits<Node>) {
	let obj_ref: Ref<Object> = obj.base();
}

#[allow(unused)]
unsafe fn test_1(obj: &PathFollow2D, obj_ref: Ref<Node2D>, obj_tref: TRef<'_, Sprite>) {
	test(obj);
	test(&obj_ref);
	test(&obj_tref);
}

#[allow(unused)]
unsafe fn test_2(obj: &impl Inherits<Resource>) {
	let obj_ref: Ref<Object> = obj.base();
}

#[allow(unused)]
unsafe fn test_3(obj: &PackedScene, obj_ref: Ref<Texture>, obj_tref: TRef<'_, DynamicFontData>) {
	test_2(obj);
	test_2(&obj_ref);
	test_2(&obj_tref);
}
*/

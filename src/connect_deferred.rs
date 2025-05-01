use godot::{
    meta,
    meta::InParamTuple,
    obj::{WithBaseField, WithSignals, bounds::DeclUser},
    register::{SignalReceiver, TypedSignal},
};

use crate::internal::*;

pub trait ConnectDeferred<'a, C, Ps: meta::ParamTuple + InParamTuple + 'static> {
    fn connect_deferred<F: SignalReceiver<(), Ps>>(&'a mut self, f: F);

    fn connect_self_deferred<F: for<'b> SignalReceiver<&'b mut C, Ps>>(&'a mut self, f: F)
    where C: GodotClass<Declarer = DeclUser>;

    fn connect_user_deferred<
        Listener: GodotClass<Declarer = DeclUser> + WithBaseField,
        F: for<'b> SignalReceiver<&'b mut Listener, Ps>,
    >(
        &'a mut self,
        listener: &Listener,
        f: F,
    );

    fn connect_gd_deferred<
        Listener: GodotClass<Declarer = DeclUser>,
        F: for<'b> SignalReceiver<&'b mut Listener, Ps>,
    >(
        &'a mut self,
        listener: &Gd<Listener>,
        f: F,
    );

    fn connect_gd_immut_deferred<
        Listener: GodotClass<Declarer = DeclUser>,
        F: for<'b> SignalReceiver<&'b Listener, Ps>,
    >(
        &'a mut self,
        listener: &Gd<Listener>,
        f: F,
    );
}

impl<'a, C: WithSignals, Ps: meta::ParamTuple + InParamTuple + 'static> ConnectDeferred<'a, C, Ps>
    for TypedSignal<'a, C, Ps>
{
    fn connect_deferred<F: SignalReceiver<(), Ps>>(&'a mut self, f: F) {
        self.connect_builder()
            .function(f)
            .flags(ConnectFlags::DEFERRED)
            .done();
    }

    fn connect_self_deferred<F: for<'b> SignalReceiver<&'b mut C, Ps>>(&'a mut self, f: F)
    where C: GodotClass<Declarer = DeclUser> {
        self.connect_builder()
            .object_self()
            .method_mut(f)
            .flags(ConnectFlags::DEFERRED)
            .done();
    }

    fn connect_user_deferred<
        Listener: GodotClass<Declarer = DeclUser> + WithBaseField,
        F: for<'b> SignalReceiver<&'b mut Listener, Ps>,
    >(
        &'a mut self,
        listener: &Listener,
        f: F,
    ) {
        let gd = listener.to_gd();

        self.connect_builder()
            .object(&gd)
            .method_mut(f)
            .flags(ConnectFlags::DEFERRED)
            .done();
    }

    fn connect_gd_deferred<
        Listener: GodotClass<Declarer = DeclUser>,
        F: for<'b> SignalReceiver<&'b mut Listener, Ps>,
    >(
        &'a mut self,
        listener: &Gd<Listener>,
        f: F,
    ) {
        self.connect_builder()
            .object(listener)
            .method_mut(f)
            .flags(ConnectFlags::DEFERRED)
            .done();
    }

    fn connect_gd_immut_deferred<
        Listener: GodotClass<Declarer = DeclUser>,
        F: for<'b> SignalReceiver<&'b Listener, Ps>,
    >(
        &'a mut self,
        listener: &Gd<Listener>,
        f: F,
    ) {
        self.connect_builder()
            .object(listener)
            .method_immut(f)
            .flags(ConnectFlags::DEFERRED)
            .done();
    }
}

#[allow(unused)]
#[cfg(test)]
mod must_compile {
    use godot::{meta::AsObjectArg, obj::bounds::DeclEngine};

    use super::*;

    #[derive(GodotClass)]
    #[class(base = Node, init)]
    pub struct MyNode {
        base: Base<Node>,
    }

    #[godot_api]
    impl MyNode {}

    fn test1(my_node: &MyNode) {}

    fn test2(mut my_node: &mut MyNode) {
        let mut gd = my_node.to_gd();
        my_node
            .base_mut()
            .signals()
            .child_entered_tree()
            .connect_obj(&gd, |node: &mut MyNode, child| {});

        my_node
            .signals()
            .child_entered_tree()
            .connect_self_deferred(|this: &mut MyNode, child| {});

        my_node
            .signals()
            .child_entered_tree()
            .connect_deferred(|child| {});

        let mut base_gd = gd.clone().upcast::<Node>();
        base_gd
            .signals()
            .child_entered_tree()
            .connect_user_deferred(&*my_node, |me: &mut MyNode, node| {});

        base_gd
            .signals()
            .child_entered_tree()
            .connect_gd_immut_deferred(&gd, |gd: &MyNode, node| {});

        base_gd
            .signals()
            .child_entered_tree()
            .connect_gd_deferred(&gd, |gd: &mut MyNode, node| {});
    }

    fn test<T: GodotClass<Declarer = DeclEngine>>(arg: impl AsObjectArg<T>) {
        let obj = arg.as_object_arg();
        let var = obj.to_variant();
    }
}

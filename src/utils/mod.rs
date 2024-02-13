mod focus_trap;
pub use focus_trap::*;

use leptos::*;
use std::rc::Rc;

/// Position of an element.
#[derive(Clone, Copy, Default)]
pub enum Position {
    #[default]
    Left,
    Right,
    Top,
    Bottom,
}

/// Generates a newtype for a `Clone,Copy` SignalSetter with a generic marker to
/// allow multiple hooks.
///
///
/// # Usage
/// The below code:
/// ```no_run
/// generate_marker_signal_setter(
///     /// Some docs
///     #[derive(Debug)]
///     MySignalSetter, ReturnType
/// )
/// ```
/// generates this:
/// ```
/// /// Some docs
/// #[derive(Debug)]
/// #[derive(Deref, DerefMut)]
/// pub struct MySignalSetter<M: 'static> {
///     #[deref]
///     setter: ::leptos::SignalSetter<ReturnType>,
///     _phant: std::marker::PhantomData<M>,
/// }
///
/// impl<M: 'static> Clone for MySignalSetter<M> {
///     fn clone(&self) -> Self {
///         *self
///     }
/// }
///
/// impl<M: 'static> Copy for MySignalSetter<M> {}
///
/// impl<M: 'static> MySignalSetter<M> {
///     pub fn new(setter: impl Fn($ty) + 'static) -> Self {
///         Self { setter: ::leptos::SignalSetter::<$ty>::map(setter), _phant: Default::default() }
///     }
/// }
/// ```
///
/// Use the newly generated type as you would a normal `SignalSetter`.
#[macro_export]
macro_rules! generate_marker_signal_setter {
    ($(#[$outer:meta])* $name:ident, $ty:ty) => {
        $(#[$outer])*
        #[derive(Deref, DerefMut)]
        pub struct $name <M: 'static> {
            #[deref]
            setter: ::leptos::SignalSetter<$ty>,
            _phant: std::marker::PhantomData<M>,
        }

        impl<M: 'static> Clone for $name <M> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<M: 'static> Copy for $name <M> {}

        impl<M: 'static> $name <M> {
            pub fn new(setter: impl Fn($ty) + 'static) -> Self {
                Self { setter: ::leptos::SignalSetter::<$ty>::map(setter), _phant: Default::default() }
            }
        }
    }
}


/// Generates a newtype for a `Clone,Copy` SignalSetter with a generic marker
/// and active type to allow multiple hooks.
///
///
/// # Usage
/// The below code:
/// ```no_run
/// generate_generic_marker_signal_setter(
///     /// Some docs
///     #[derive(Debug)]
///     MySignalSetter, T
/// )
/// ```
///
/// Use the newly generated type as you would a normal `SignalSetter`.
#[macro_export]
macro_rules! generate_generic_marker_signal_setter {
    ($(#[$outer:meta])* $name:ident, $map_ty:ty, $ty:ident) => {
        $(#[$outer])*
        #[derive(Deref, DerefMut)]
        pub struct $name <M: 'static, $ty: 'static> {
            #[deref]
            setter: ::leptos::SignalSetter<$map_ty>,
            _phant: std::marker::PhantomData<(M, T)>,
        }

        impl<M: 'static, $ty> Clone for $name <M, $ty> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<M: 'static, $ty> Copy for $name <M, $ty> {}

        impl<M: 'static, $ty> $name <M, $ty> {
            pub fn new(setter: impl Fn($map_ty) + 'static) -> Self {
                Self { setter: ::leptos::SignalSetter::<$map_ty>::map(setter), _phant: Default::default() }
            }
        }
    }
}

/// Generates a newtype for a `Clone` read signal with a generic marker to
/// allow multiple hooks.
///
///
/// # Usage
/// The below code:
/// ```no_run
/// generate_marker_bootleg_read_signal(
///     /// Some docs
///     #[derive(Debug)]
///     MyReadSignal, ReturnType
/// )
/// ```
/// generates this:
/// ```
/// /// Some docs
/// #[derive(Debug)]
/// #[derive(Deref, DerefMut)]
/// pub struct MyReadSignal<M: 'static> {
///     #[deref]
///     reader: ::std::rc::Rc<dyn Fn() -> ReturnType>,
///     _phant: std::marker::PhantomData<M>,
/// }
///
/// impl<M: 'static> Clone for MyReadSignal<M> {
///     fn clone(&self) -> Self {
///         Self { reader: self.reader.clone(), _phant: Default::default() }
///     }
/// }
///
/// impl<M: 'static> MyReadSignal<M> {
///     pub fn new(reader: ::std::rc::Rc<dyn Fn() -> ReturnType>) -> Self {
///         Self { reader, _phant: Default::default() }
///     }
/// }
/// ```
///
/// Use the newly generated type as you would a normal read signal.
#[macro_export]
macro_rules! generate_marker_bootleg_read_signal {
    ($(#[$outer:meta])* $name:ident, $ty:ty) => {
        $(#[$outer])*
        #[derive(Deref, DerefMut)]
        pub struct $name <M: 'static> {
            #[deref]
            reader: ::std::rc::Rc<dyn Fn() -> $ty>,
            _phant: std::marker::PhantomData<M>,
        }

        impl<M: 'static> Clone for $name <M> {
            fn clone(&self) -> Self {
                Self { reader: self.reader.clone(), _phant: Default::default() }
            }
        }

        impl<M: 'static> $name <M> {
            pub fn new(reader: ::std::rc::Rc<dyn Fn() -> $ty>) -> Self {
                Self { reader, _phant: Default::default() }
            }
        }
    };
}

/// Generates a unit struct that acts as a marker type.
#[macro_export]
macro_rules! generate_marker_type {
    ($(#[$outer:meta])* $name:ident) => {
        $(#[$outer])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name;
    };
}

/// Generates a newtype for a `Clone,Copy` WriteSignal with a generic marker to
/// allow multiple hooks.
///
///
/// # Usage
/// The below code:
/// ```no_run
/// generate_marker_arbitrary_write_signal(
///     /// Some docs
///     #[derive(Debug)]
///     MyWriteSignal, ReturnType
/// )
/// ```
/// generates this:
/// ```
/// /// Some docs
/// #[derive(Debug)]
/// #[derive(Deref, DerefMut)]
/// pub struct MyWriteSignal<M: 'static> {
///     #[deref]
///     writer: ::leptos::WriteSignal<ReturnType>,
///     _phant: std::marker::PhantomData<M>,
/// }
///
/// impl<M: 'static> Clone for MyWriteSignal<M> {
///     fn clone(&self) -> Self {
///         *self
///     }
/// }
///
/// impl<M: 'static> Copy for MyWriteSignal<M> {}
///
/// impl<M: 'static> MyWriteSignal<M> {
///     pub fn new(writer: ::leptos::WriteSignal<ReturnType>) -> Self {
///         Self { writer, _phant: Default::default() }
///     }
/// }
///
/// impl<M: 'static> ::leptos::SignalUpdate<ReturnType> for MyWriteSignal<M> {
///     type Value = ReturnType;
///
///     fn update(&self, f: impl FnOnce(&mut Self::Value)) {
///         self.writer.update(f)
///     }
///
///     fn try_update<O>(&self, f: impl FnOnce(&mut Self::Value) -> O) -> Option<O> {
///         self.writer.try_update(f)
///     }
/// }
/// ```
///
/// Use the newly generated type as you would a normal `WriteSignal`.
#[macro_export]
macro_rules! generate_marker_arbitrary_write_signal {
    ($(#[$outer:meta])* $name:ident, $ty:ty) => {
        $(#[$outer])*
        #[derive(Deref, DerefMut)]
        pub struct $name <M: 'static> {
            #[deref]
            writer: ::leptos::WriteSignal<$ty>,
            _phant: std::marker::PhantomData<M>,
        }

        impl<M: 'static> Clone for $name <M> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<M: 'static> Copy for $name <M> {}

        impl<M: 'static> $name <M> {
            pub fn new(writer: ::leptos::WriteSignal<$ty>) -> Self {
                Self { writer, _phant: Default::default() }
            }
        }

        impl<M: 'static> ::leptos::SignalUpdate for $name <M> {
            type Value = $ty;

            fn update(&self, f: impl FnOnce(&mut Self::Value)) {
                self.writer.update(f)
            }

            fn try_update<O>(&self, f: impl FnOnce(&mut Self::Value) -> O) -> Option<O> {
                self.writer.try_update(f)
            }
        }
    };
}

/// New-type wrapper for the a function that returns a view with `From` and `Default` traits implemented
/// to enable optional props in for example `<Show>` and `<Suspense>`.
pub struct ParamViewFn<T>(Rc<dyn Fn(T) -> View>);

impl<T> Default for ParamViewFn<T> {
    fn default() -> Self {
        Self(Rc::new(|_| ().into_view()))
    }
}

impl<T, F, IV> From<F> for ParamViewFn<T>
where
    F: Fn(T) -> IV + 'static,
    IV: IntoView,
{
    fn from(value: F) -> Self {
        Self(Rc::new(move |val| value(val).into_view()))
    }
}

impl<T> ParamViewFn<T> {
    /// Execute the wrapped function
    pub fn run(&self, val: T) -> View {
        (self.0)(val)
    }
}

impl<T> Clone for ParamViewFn<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

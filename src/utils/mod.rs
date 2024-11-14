mod focus_trap;
mod errors;
pub use focus_trap::*;
pub use errors::{Errors, ReactiveErrors};

use std::rc::Rc;
use leptos::*;

/// Position of an element.
#[derive(Clone, Copy, Default)]
pub enum Position {
	Left,
	#[default]
	Right,
	Top,
	Bottom,
}

/// Generates a newtype for a `Clone,Copy` SignalSetter with a generic marker to
/// allow multiple hooks.
///
/// # Usage
/// ```no_run
/// #[derive(Debug)]
/// pub struct ReturnType;
/// wu::generate_marker_signal_setter!(
///     /// Some docs
///     #[derive(Debug)]
///     MySignalSetter, ReturnType
/// );
/// ```
///
/// Use the newly generated type as you would a normal `SignalSetter`.
#[macro_export]
macro_rules! generate_marker_signal_setter {
    ($(#[$outer:meta])* $name:ident, $ty:ty) => {
        $(#[$outer])*
        pub struct $name <M: 'static> {
            setter: ::leptos::SignalSetter<$ty>,
            _phant: std::marker::PhantomData<M>,
        }

        impl<M: 'static> Clone for $name <M> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<M: 'static> Copy for $name <M> {}

        impl<M: 'static> std::ops::Deref for $name <M> {
            type Target = ::leptos::SignalSetter<$ty>;

            fn deref(&self) -> &Self::Target {
                &self.setter
            }
        }

        impl<M: 'static> std::ops::DerefMut for $name <M> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.setter
            }
        }

        impl<M: 'static> $name <M> {
            pub fn new(setter: impl Fn($ty) + 'static) -> Self {
                Self { setter: ::leptos::SignalSetter::<$ty>::map(setter), _phant: Default::default() }
            }
        }

        impl<M: 'static> std::convert::From<::leptos::SignalSetter<$ty>> for $name <M> {
            fn from(value: ::leptos::SignalSetter<$ty>) -> Self {
                Self { setter: value, _phant: Default::default() }
            }
        }
    }
}

/// Generates a newtype for a `Clone,Copy` SignalSetter with a generic marker
/// and active type to allow multiple hooks.
///
/// # Usage
/// ```no_run
/// wu::generate_generic_marker_signal_setter!(
///     /// Some docs
///     #[derive(Debug)]
///     MySignalSetter, T, T
/// );
/// ```
///
/// Use the newly generated type as you would a normal `SignalSetter`.
#[macro_export]
macro_rules! generate_generic_marker_signal_setter {
    ($(#[$outer:meta])* $name:ident, $map_ty:ty, $ty:ident) => {
        $(#[$outer])*
        pub struct $name <M: 'static, $ty: 'static> {
            setter: ::leptos::SignalSetter<$map_ty>,
            _phant: std::marker::PhantomData<(M, T)>,
        }

        impl<M: 'static, $ty> Clone for $name <M, $ty> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<M: 'static, $ty> Copy for $name <M, $ty> {}

        impl<M: 'static, $ty> std::ops::Deref for $name <M, $ty> {
            type Target = ::leptos::SignalSetter<$map_ty>;

            fn deref(&self) -> &Self::Target {
                &self.setter
            }
        }

        impl<M: 'static, $ty> std::ops::DerefMut for $name <M, $ty> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.setter
            }
        }

        impl<M: 'static, $ty> $name <M, $ty> {
            pub fn new(setter: impl Fn($map_ty) + 'static) -> Self {
                Self { setter: ::leptos::SignalSetter::<$map_ty>::map(setter), _phant: Default::default() }
            }
        }

        impl<M: 'static, $ty> std::convert::From<::leptos::SignalSetter<$map_ty>> for $name <M, $ty> {
            fn from(value: ::leptos::SignalSetter<$map_ty>) -> Self {
                Self { setter: value, _phant: Default::default() }
            }
        }
    }
}

/// Generates a newtype for a `Clone` read signal with a generic marker to
/// allow multiple hooks.
///
/// # Usage
/// ```no_run
/// pub struct ReturnType;
/// wu::generate_marker_bootleg_read_signal!(
///     /// Some docs
///     MyReadSignal, ReturnType
/// );
/// ```
///
/// Use the newly generated type as you would a normal read signal.
#[macro_export]
macro_rules! generate_marker_bootleg_read_signal {
    ($(#[$outer:meta])* $name:ident, $ty:ty) => {
        $(#[$outer])*
        pub struct $name <M: 'static> {
            reader: ::std::rc::Rc<dyn Fn() -> $ty>,
            _phant: std::marker::PhantomData<M>,
        }

        impl<M: 'static> Clone for $name <M> {
            fn clone(&self) -> Self {
                Self { reader: self.reader.clone(), _phant: Default::default() }
            }
        }

        impl<M: 'static> std::ops::Deref for $name <M> {
            type Target = ::std::rc::Rc<dyn Fn() -> $ty>;

            fn deref(&self) -> &Self::Target {
                &self.reader
            }
        }

        impl<M: 'static> std::ops::DerefMut for $name <M> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.reader
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
/// ```no_run
/// #[derive(Debug)]
/// pub struct ReturnType;
/// wu::generate_marker_arbitrary_write_signal!(
///     /// Some docs
///     #[derive(Debug)]
///     MyWriteSignal, ReturnType
/// );
/// ```
///
/// Use the newly generated type as you would a normal `WriteSignal`.
#[macro_export]
macro_rules! generate_marker_arbitrary_write_signal {
    ($(#[$outer:meta])* $name:ident, $ty:ty) => {
        $(#[$outer])*
        pub struct $name <M: 'static> {
            writer: ::leptos::WriteSignal<$ty>,
            _phant: std::marker::PhantomData<M>,
        }

        impl<M: 'static> Clone for $name <M> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<M: 'static> Copy for $name <M> {}

        impl<M: 'static> std::ops::Deref for $name <M> {
            type Target = ::leptos::WriteSignal<$ty>;

            fn deref(&self) -> &Self::Target {
                &self.writer
            }
        }

        impl<M: 'static> std::ops::DerefMut for $name <M> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.writer
            }
        }

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

use leptos::prelude::*;

/// A utility signal type that provides easy manipulation of a string.
///
/// Aims to replace `TextProp`.
#[derive(Clone, Copy)]
pub struct Text(Signal<std::borrow::Cow<'static, str>>);

impl Text {
	/// Gets the underlying string.
	pub fn get(&self) -> std::borrow::Cow<'static, str> {
		self.0.get()
	}
}

impl std::convert::From<String> for Text {
	fn from(value: String) -> Self {
		Self(Signal::stored(value.into()))
	}
}

impl std::convert::From<&'static str> for Text {
	fn from(value: &'static str) -> Self {
		Self(Signal::stored(value.into()))
	}
}

impl std::convert::From<std::borrow::Cow<'static, str>> for Text {
	fn from(value: std::borrow::Cow<'static, str>) -> Self {
		Self(Signal::stored(value))
	}
}

impl<S> std::convert::From<Signal<S>> for Text
where
	S: std::convert::Into<std::borrow::Cow<'static, str>> + Clone + Send + Sync + 'static,
{
	fn from(value: Signal<S>) -> Self {
		Self(Signal::derive(move || value.get().into()))
	}
}

impl<S> std::convert::From<ArcSignal<S>> for Text
where
	S: std::convert::Into<std::borrow::Cow<'static, str>> + Clone + Send + Sync + 'static,
{
	fn from(value: ArcSignal<S>) -> Self {
		Self(Signal::derive(move || value.get().into()))
	}
}

impl<F, S> std::convert::From<F> for Text
where
	F: Fn() -> S + Send + Sync + 'static,
	S: Into<std::borrow::Cow<'static, str>>,
{
	fn from(value: F) -> Self {
		Self(Signal::derive(move || value().into()))
	}
}

impl Default for Text {
	fn default() -> Self {
		Self(Signal::stored("".into()))
	}
}

impl IntoRender for Text {
	type Output = Signal<std::borrow::Cow<'static, str>>;

	fn into_render(self) -> Self::Output {
		self.0
	}
}

impl IntoAttributeValue for Text {
	type Output = Signal<String>;

	fn into_attribute_value(self) -> Self::Output {
		Signal::derive(move || self.0.get().into_owned())
	}
}

impl std::fmt::Display for Text {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.0.get())
	}
}

impl std::fmt::Debug for Text {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.0.get())
	}
}

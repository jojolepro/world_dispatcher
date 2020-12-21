pub use crate::*;

/// The type of `Resource`s.
/// All types having a 'static lifetime automatically implement this.
pub(crate) trait Resource: 'static + Downcast {}
impl<T> Resource for T where T: 'static {}
impl_downcast!(Resource);

/// Hacky trait to extend the lifetime of a Ref<'a, T>, which is used
/// internally in the `Dispatcher`'s logic.
/// Import this if you get errors where RefLifetime is not implemented for
/// your systems.
pub(crate) trait RefLifetime {}
impl<'a, T> RefLifetime for Ref<'a, T> {}
impl<'a, T> RefLifetime for RefMut<'a, T> {}
impl<'a, T> RefLifetime for &'a T {}
impl<'a, T> RefLifetime for &'a mut T {}
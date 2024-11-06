// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
use crate::Collation;
use crate::{ffi, Expression, Sorter};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkStringSorter")]
    pub struct StringSorter(Object<ffi::GtkStringSorter, ffi::GtkStringSorterClass>) @extends Sorter;

    match fn {
        type_ => || ffi::gtk_string_sorter_get_type(),
    }
}

impl StringSorter {
    #[doc(alias = "gtk_string_sorter_new")]
    pub fn new(expression: Option<impl AsRef<Expression>>) -> StringSorter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_string_sorter_new(
                expression
                    .map(|p| p.as_ref().clone().upcast())
                    .into_glib_ptr(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`StringSorter`] objects.
    ///
    /// This method returns an instance of [`StringSorterBuilder`](crate::builders::StringSorterBuilder) which can be used to create [`StringSorter`] objects.
    pub fn builder() -> StringSorterBuilder {
        StringSorterBuilder::new()
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_string_sorter_get_collation")]
    #[doc(alias = "get_collation")]
    pub fn collation(&self) -> Collation {
        unsafe { from_glib(ffi::gtk_string_sorter_get_collation(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_string_sorter_get_expression")]
    #[doc(alias = "get_expression")]
    pub fn expression(&self) -> Option<Expression> {
        unsafe { from_glib_none(ffi::gtk_string_sorter_get_expression(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_string_sorter_get_ignore_case")]
    #[doc(alias = "get_ignore_case")]
    #[doc(alias = "ignore-case")]
    pub fn ignores_case(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_string_sorter_get_ignore_case(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_string_sorter_set_collation")]
    #[doc(alias = "collation")]
    pub fn set_collation(&self, collation: Collation) {
        unsafe {
            ffi::gtk_string_sorter_set_collation(self.to_glib_none().0, collation.into_glib());
        }
    }

    #[doc(alias = "gtk_string_sorter_set_expression")]
    #[doc(alias = "expression")]
    pub fn set_expression(&self, expression: Option<impl AsRef<Expression>>) {
        unsafe {
            ffi::gtk_string_sorter_set_expression(
                self.to_glib_none().0,
                expression.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_string_sorter_set_ignore_case")]
    #[doc(alias = "ignore-case")]
    pub fn set_ignore_case(&self, ignore_case: bool) {
        unsafe {
            ffi::gtk_string_sorter_set_ignore_case(self.to_glib_none().0, ignore_case.into_glib());
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "collation")]
    pub fn connect_collation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_collation_trampoline<F: Fn(&StringSorter) + 'static>(
            this: *mut ffi::GtkStringSorter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::collation\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_collation_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "expression")]
    pub fn connect_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<F: Fn(&StringSorter) + 'static>(
            this: *mut ffi::GtkStringSorter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expression\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_expression_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "ignore-case")]
    pub fn connect_ignore_case_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ignore_case_trampoline<F: Fn(&StringSorter) + 'static>(
            this: *mut ffi::GtkStringSorter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ignore-case\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_ignore_case_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for StringSorter {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`StringSorter`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct StringSorterBuilder {
    builder: glib::object::ObjectBuilder<'static, StringSorter>,
}

impl StringSorterBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn collation(self, collation: Collation) -> Self {
        Self {
            builder: self.builder.property("collation", collation),
        }
    }

    pub fn expression(self, expression: impl AsRef<Expression>) -> Self {
        Self {
            builder: self
                .builder
                .property("expression", expression.as_ref().clone()),
        }
    }

    pub fn ignore_case(self, ignore_case: bool) -> Self {
        Self {
            builder: self.builder.property("ignore-case", ignore_case),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`StringSorter`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> StringSorter {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}

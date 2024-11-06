// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, EventController, IMContext, PropagationLimit, PropagationPhase, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkEventControllerKey")]
    pub struct EventControllerKey(Object<ffi::GtkEventControllerKey, ffi::GtkEventControllerKeyClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_event_controller_key_get_type(),
    }
}

impl EventControllerKey {
    #[doc(alias = "gtk_event_controller_key_new")]
    pub fn new() -> EventControllerKey {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_key_new()).unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`EventControllerKey`] objects.
    ///
    /// This method returns an instance of [`EventControllerKeyBuilder`](crate::builders::EventControllerKeyBuilder) which can be used to create [`EventControllerKey`] objects.
    pub fn builder() -> EventControllerKeyBuilder {
        EventControllerKeyBuilder::new()
    }

    #[doc(alias = "gtk_event_controller_key_forward")]
    pub fn forward(&self, widget: &impl IsA<Widget>) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_key_forward(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_event_controller_key_get_group")]
    #[doc(alias = "get_group")]
    pub fn group(&self) -> u32 {
        unsafe { ffi::gtk_event_controller_key_get_group(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_event_controller_key_get_im_context")]
    #[doc(alias = "get_im_context")]
    pub fn im_context(&self) -> Option<IMContext> {
        unsafe {
            from_glib_none(ffi::gtk_event_controller_key_get_im_context(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_event_controller_key_set_im_context")]
    pub fn set_im_context(&self, im_context: Option<&impl IsA<IMContext>>) {
        unsafe {
            ffi::gtk_event_controller_key_set_im_context(
                self.to_glib_none().0,
                im_context.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "im-update")]
    pub fn connect_im_update<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn im_update_trampoline<F: Fn(&EventControllerKey) + 'static>(
            this: *mut ffi::GtkEventControllerKey,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"im-update\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    im_update_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "modifiers")]
    pub fn connect_modifiers<F: Fn(&Self, gdk::ModifierType) -> glib::Propagation + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn modifiers_trampoline<
            F: Fn(&EventControllerKey, gdk::ModifierType) -> glib::Propagation + 'static,
        >(
            this: *mut ffi::GtkEventControllerKey,
            state: gdk::ffi::GdkModifierType,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(state)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"modifiers\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    modifiers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for EventControllerKey {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`EventControllerKey`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct EventControllerKeyBuilder {
    builder: glib::object::ObjectBuilder<'static, EventControllerKey>,
}

impl EventControllerKeyBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn propagation_limit(self, propagation_limit: PropagationLimit) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-limit", propagation_limit),
        }
    }

    pub fn propagation_phase(self, propagation_phase: PropagationPhase) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-phase", propagation_phase),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`EventControllerKey`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> EventControllerKey {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}

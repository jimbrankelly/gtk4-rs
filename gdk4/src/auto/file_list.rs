// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FileList(Boxed<ffi::GdkFileList>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::gdk_file_list_get_type(), ptr as *mut _) as *mut ffi::GdkFileList,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::gdk_file_list_get_type(), ptr as *mut _),
        type_ => || ffi::gdk_file_list_get_type(),
    }
}

impl FileList {
    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    #[doc(alias = "gdk_file_list_new_from_array")]
    #[doc(alias = "new_from_array")]
    pub fn from_array(files: &[gio::File]) -> FileList {
        assert_initialized_main_thread!();
        let n_files = files.len() as usize;
        unsafe {
            from_glib_full(ffi::gdk_file_list_new_from_array(
                files.to_glib_none().0,
                n_files,
            ))
        }
    }

    #[doc(alias = "gdk_file_list_get_files")]
    #[doc(alias = "get_files")]
    pub fn files(&self) -> Vec<gio::File> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_file_list_get_files(mut_override(
                self.to_glib_none().0,
            )))
        }
    }
}

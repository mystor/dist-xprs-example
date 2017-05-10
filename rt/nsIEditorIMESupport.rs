//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditorIMESupport.idl
//


#[repr(C)]
pub struct nsIEditorIMESupport {
    vtable: *const nsIEditorIMESupportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEditorIMESupport {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0ba7f490, 0xafb8, 0x46dd,
            [0x87, 0xfc, 0xbc, 0x61, 0x37, 0xfb, 0xc8, 0x99])
    }
}

unsafe impl RefCounted for nsIEditorIMESupport {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// Enable coercing to ourselves
pub trait nsIEditorIMESupportCoerce {
    fn coerce_from(v: &nsIEditorIMESupport) -> &Self;
}

impl nsIEditorIMESupportCoerce for nsIEditorIMESupport {
    #[inline]
    fn coerce_from(v: &nsIEditorIMESupport) -> &Self {
        v
    }
}

impl nsIEditorIMESupport {
    #[inline]
    pub fn coerce<T: nsIEditorIMESupportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEditorIMESupport {
    type Target = nsIEditor;
    #[inline]
    fn deref(&self) -> &nsIEditor {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIEditorCoerce> nsIEditorIMESupportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditorIMESupport) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEditorIMESupportVTable {
    pub __base: nsIEditorVTable,

}


impl nsIEditorIMESupport {
}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebVTTListener.idl
//


#[repr(C)]
pub struct nsIWebVTTListener {
    vtable: *const nsIWebVTTListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebVTTListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8a2d7780, 0x2045, 0x4a29,
            [0x99, 0xf4, 0xdf, 0x15, 0xca, 0xe5, 0xfc, 0x49])
    }
}

unsafe impl RefCounted for nsIWebVTTListener {
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
pub trait nsIWebVTTListenerCoerce {
    fn coerce_from(v: &nsIWebVTTListener) -> &Self;
}

impl nsIWebVTTListenerCoerce for nsIWebVTTListener {
    #[inline]
    fn coerce_from(v: &nsIWebVTTListener) -> &Self {
        v
    }
}

impl nsIWebVTTListener {
    #[inline]
    pub fn coerce<T: nsIWebVTTListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebVTTListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebVTTListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebVTTListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebVTTListenerVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void onCue (in jsval cue); */
    /// Unable to call function as its signature contains a non-rust type
    pub onCue: *const ::libc::c_void,

    /* [implicit_jscontext] void onRegion (in jsval region); */
    /// Unable to call function as its signature contains a non-rust type
    pub onRegion: *const ::libc::c_void,

    /* [implicit_jscontext] void onParsingError (in long errorCode); */
    /// Unable to call function as its signature contains a non-rust type
    pub onParsingError: *const ::libc::c_void,

}


impl nsIWebVTTListener {
    /* [implicit_jscontext] void onCue (in jsval cue); */


    /* [implicit_jscontext] void onRegion (in jsval region); */


    /* [implicit_jscontext] void onParsingError (in long errorCode); */


}



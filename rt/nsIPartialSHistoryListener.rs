//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPartialSHistoryListener.idl
//


#[repr(C)]
pub struct nsIPartialSHistoryListener {
    vtable: *const nsIPartialSHistoryListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPartialSHistoryListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbe0cd2b6, 0x6f03, 0x4366,
            [0x9f, 0xe2, 0x18, 0x4c, 0x91, 0x4f, 0xf3, 0xdf])
    }
}

unsafe impl RefCounted for nsIPartialSHistoryListener {
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
pub trait nsIPartialSHistoryListenerCoerce {
    fn coerce_from(v: &nsIPartialSHistoryListener) -> &Self;
}

impl nsIPartialSHistoryListenerCoerce for nsIPartialSHistoryListener {
    #[inline]
    fn coerce_from(v: &nsIPartialSHistoryListener) -> &Self {
        v
    }
}

impl nsIPartialSHistoryListener {
    #[inline]
    pub fn coerce<T: nsIPartialSHistoryListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPartialSHistoryListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPartialSHistoryListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPartialSHistoryListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPartialSHistoryListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onRequestCrossBrowserNavigation (in unsigned long aIndex); */
    pub onRequestCrossBrowserNavigation: unsafe extern "C" fn (this: *const nsIPartialSHistoryListener, aIndex: libc::uint32_t) -> nsresult,

}


impl nsIPartialSHistoryListener {
    /* void onRequestCrossBrowserNavigation (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn onRequestCrossBrowserNavigation(&self, aIndex: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onRequestCrossBrowserNavigation)(self as *const _, aIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



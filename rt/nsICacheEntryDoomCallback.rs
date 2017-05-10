//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheEntryDoomCallback.idl
//


#[repr(C)]
pub struct nsICacheEntryDoomCallback {
    vtable: *const nsICacheEntryDoomCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheEntryDoomCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2f8896be, 0x232f, 0x4140,
            [0xaf, 0xb3, 0x1f, 0xaf, 0xfb, 0x56, 0xf3, 0xc6])
    }
}

unsafe impl RefCounted for nsICacheEntryDoomCallback {
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
pub trait nsICacheEntryDoomCallbackCoerce {
    fn coerce_from(v: &nsICacheEntryDoomCallback) -> &Self;
}

impl nsICacheEntryDoomCallbackCoerce for nsICacheEntryDoomCallback {
    #[inline]
    fn coerce_from(v: &nsICacheEntryDoomCallback) -> &Self {
        v
    }
}

impl nsICacheEntryDoomCallback {
    #[inline]
    pub fn coerce<T: nsICacheEntryDoomCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheEntryDoomCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheEntryDoomCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheEntryDoomCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheEntryDoomCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onCacheEntryDoomed (in nsresult aResult); */
    pub onCacheEntryDoomed: unsafe extern "C" fn (this: *const nsICacheEntryDoomCallback, aResult: nsresult) -> nsresult,

}


impl nsICacheEntryDoomCallback {
    /* void onCacheEntryDoomed (in nsresult aResult); */
    #[inline]
    pub unsafe fn onCacheEntryDoomed(&self, aResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onCacheEntryDoomed)(self as *const _, aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



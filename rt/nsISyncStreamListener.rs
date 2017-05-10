//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISyncStreamListener.idl
//


#[repr(C)]
pub struct nsISyncStreamListener {
    vtable: *const nsISyncStreamListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISyncStreamListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7e1aa658, 0x6e3f, 0x4521,
            [0x99, 0x46, 0x96, 0x85, 0xa1, 0x69, 0xf7, 0x64])
    }
}

unsafe impl RefCounted for nsISyncStreamListener {
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
pub trait nsISyncStreamListenerCoerce {
    fn coerce_from(v: &nsISyncStreamListener) -> &Self;
}

impl nsISyncStreamListenerCoerce for nsISyncStreamListener {
    #[inline]
    fn coerce_from(v: &nsISyncStreamListener) -> &Self {
        v
    }
}

impl nsISyncStreamListener {
    #[inline]
    pub fn coerce<T: nsISyncStreamListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISyncStreamListener {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamListenerCoerce> nsISyncStreamListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISyncStreamListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISyncStreamListenerVTable {
    pub __base: nsIStreamListenerVTable,

    /* readonly attribute nsIInputStream inputStream; */
    pub get_inputStream: unsafe extern "C" fn (this: *const nsISyncStreamListener, aInputStream: *mut *const nsIInputStream) -> nsresult,

}


impl nsISyncStreamListener {
    /* readonly attribute nsIInputStream inputStream; */
    #[inline]
    pub unsafe fn get_inputStream(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_inputStream)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



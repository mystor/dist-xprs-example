//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowlessBrowser.idl
//


#[repr(C)]
pub struct nsIWindowlessBrowser {
    vtable: *const nsIWindowlessBrowserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWindowlessBrowser {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xabb46f48, 0xabfc, 0x41bf,
            [0xaa, 0x9a, 0x7f, 0xec, 0xce, 0xfc, 0xf9, 0x77])
    }
}

unsafe impl RefCounted for nsIWindowlessBrowser {
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
pub trait nsIWindowlessBrowserCoerce {
    fn coerce_from(v: &nsIWindowlessBrowser) -> &Self;
}

impl nsIWindowlessBrowserCoerce for nsIWindowlessBrowser {
    #[inline]
    fn coerce_from(v: &nsIWindowlessBrowser) -> &Self {
        v
    }
}

impl nsIWindowlessBrowser {
    #[inline]
    pub fn coerce<T: nsIWindowlessBrowserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWindowlessBrowser {
    type Target = nsIWebNavigation;
    #[inline]
    fn deref(&self) -> &nsIWebNavigation {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIWebNavigationCoerce> nsIWindowlessBrowserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowlessBrowser) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWindowlessBrowserVTable {
    pub __base: nsIWebNavigationVTable,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIWindowlessBrowser) -> nsresult,

}


impl nsIWindowlessBrowser {
    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



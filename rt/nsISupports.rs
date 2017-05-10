//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISupports.idl
//


#[repr(C)]
pub struct nsISupports {
    vtable: *const nsISupportsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupports {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x00000000, 0x0000, 0x0000,
            [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46])
    }
}

unsafe impl RefCounted for nsISupports {
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
pub trait nsISupportsCoerce {
    fn coerce_from(v: &nsISupports) -> &Self;
}

impl nsISupportsCoerce for nsISupports {
    #[inline]
    fn coerce_from(v: &nsISupports) -> &Self {
        v
    }
}

impl nsISupports {
    #[inline]
    pub fn coerce<T: nsISupportsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

#[repr(C)]
pub struct nsISupportsVTable {/* void QueryInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    pub QueryInterface: unsafe extern "C" fn (this: *const nsISupports, uuid: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

    /* [noscript,notxpcom] nsrefcnt AddRef (); */
    pub AddRef: unsafe extern "C" fn (this: *const nsISupports) -> nsrefcnt,

    /* [noscript,notxpcom] nsrefcnt Release (); */
    pub Release: unsafe extern "C" fn (this: *const nsISupports) -> nsrefcnt,

}


impl nsISupports {
    /* void QueryInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn QueryInterface<T: XpCom>(&self, ) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).QueryInterface)(self as *const _, &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

    /* [noscript,notxpcom] nsrefcnt AddRef (); */
    #[inline]
    pub unsafe fn AddRef(&self, ) -> nsrefcnt {

        let _retval = ((*self.vtable).AddRef)(self as *const _, );
        _retval
    }

    /* [noscript,notxpcom] nsrefcnt Release (); */
    #[inline]
    pub unsafe fn Release(&self, ) -> nsrefcnt {

        let _retval = ((*self.vtable).Release)(self as *const _, );
        _retval
    }

}



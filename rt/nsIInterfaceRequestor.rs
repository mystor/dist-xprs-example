//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInterfaceRequestor.idl
//


#[repr(C)]
pub struct nsIInterfaceRequestor {
    vtable: *const nsIInterfaceRequestorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInterfaceRequestor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x033a1470, 0x8b2a, 0x11d3,
            [0xaf, 0x88, 0x00, 0xa0, 0x24, 0xff, 0xc0, 0x8c])
    }
}

unsafe impl RefCounted for nsIInterfaceRequestor {
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
pub trait nsIInterfaceRequestorCoerce {
    fn coerce_from(v: &nsIInterfaceRequestor) -> &Self;
}

impl nsIInterfaceRequestorCoerce for nsIInterfaceRequestor {
    #[inline]
    fn coerce_from(v: &nsIInterfaceRequestor) -> &Self {
        v
    }
}

impl nsIInterfaceRequestor {
    #[inline]
    pub fn coerce<T: nsIInterfaceRequestorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInterfaceRequestor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIInterfaceRequestorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInterfaceRequestor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInterfaceRequestorVTable {
    pub __base: nsISupportsVTable,

    /* void getInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    pub getInterface: unsafe extern "C" fn (this: *const nsIInterfaceRequestor, uuid: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

}


impl nsIInterfaceRequestor {
    /* void getInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn getInterface<T: XpCom>(&self, ) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).getInterface)(self as *const _, &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

}



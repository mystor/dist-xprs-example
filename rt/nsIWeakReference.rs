//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWeakReference.idl
//


#[repr(C)]
pub struct nsIWeakReference {
    vtable: *const nsIWeakReferenceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWeakReference {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9188bc85, 0xf92e, 0x11d2,
            [0x81, 0xef, 0x00, 0x60, 0x08, 0x3a, 0x0b, 0xcf])
    }
}

unsafe impl RefCounted for nsIWeakReference {
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
pub trait nsIWeakReferenceCoerce {
    fn coerce_from(v: &nsIWeakReference) -> &Self;
}

impl nsIWeakReferenceCoerce for nsIWeakReference {
    #[inline]
    fn coerce_from(v: &nsIWeakReference) -> &Self {
        v
    }
}

impl nsIWeakReference {
    #[inline]
    pub fn coerce<T: nsIWeakReferenceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWeakReference {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWeakReferenceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWeakReference) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWeakReferenceVTable {
    pub __base: nsISupportsVTable,

    /* void QueryReferent (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    pub QueryReferent: unsafe extern "C" fn (this: *const nsIWeakReference, uuid: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

}


impl nsIWeakReference {
    /* void QueryReferent (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn QueryReferent<T: XpCom>(&self, ) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).QueryReferent)(self as *const _, &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

}


#[repr(C)]
pub struct nsISupportsWeakReference {
    vtable: *const nsISupportsWeakReferenceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsWeakReference {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9188bc86, 0xf92e, 0x11d2,
            [0x81, 0xef, 0x00, 0x60, 0x08, 0x3a, 0x0b, 0xcf])
    }
}

unsafe impl RefCounted for nsISupportsWeakReference {
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
pub trait nsISupportsWeakReferenceCoerce {
    fn coerce_from(v: &nsISupportsWeakReference) -> &Self;
}

impl nsISupportsWeakReferenceCoerce for nsISupportsWeakReference {
    #[inline]
    fn coerce_from(v: &nsISupportsWeakReference) -> &Self {
        v
    }
}

impl nsISupportsWeakReference {
    #[inline]
    pub fn coerce<T: nsISupportsWeakReferenceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsWeakReference {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISupportsWeakReferenceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsWeakReference) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsWeakReferenceVTable {
    pub __base: nsISupportsVTable,

    /* nsIWeakReference GetWeakReference (); */
    pub GetWeakReference: unsafe extern "C" fn (this: *const nsISupportsWeakReference, _retval: *mut *const nsIWeakReference) -> nsresult,

}


impl nsISupportsWeakReference {
    /* nsIWeakReference GetWeakReference (); */
    #[inline]
    pub unsafe fn GetWeakReference(&self, ) -> Result<Option<RefPtr<nsIWeakReference>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetWeakReference)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFDelegateFactory.idl
//


#[repr(C)]
pub struct nsIRDFDelegateFactory {
    vtable: *const nsIRDFDelegateFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFDelegateFactory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa1b89470, 0xa124, 0x11d3,
            [0xbe, 0x59, 0x00, 0x20, 0xa6, 0x36, 0x16, 0x67])
    }
}

unsafe impl RefCounted for nsIRDFDelegateFactory {
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
pub trait nsIRDFDelegateFactoryCoerce {
    fn coerce_from(v: &nsIRDFDelegateFactory) -> &Self;
}

impl nsIRDFDelegateFactoryCoerce for nsIRDFDelegateFactory {
    #[inline]
    fn coerce_from(v: &nsIRDFDelegateFactory) -> &Self {
        v
    }
}

impl nsIRDFDelegateFactory {
    #[inline]
    pub fn coerce<T: nsIRDFDelegateFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFDelegateFactory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFDelegateFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFDelegateFactory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFDelegateFactoryVTable {
    pub __base: nsISupportsVTable,

    /* void CreateDelegate (in nsIRDFResource aOuter, in string aKey, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult aResult); */
    pub CreateDelegate: unsafe extern "C" fn (this: *const nsIRDFDelegateFactory, aOuter: *const nsIRDFResource, aKey: *const libc::c_char, aIID: *const nsIID, aResult: *mut *const libc::c_void) -> nsresult,

}


impl nsIRDFDelegateFactory {
    /* void CreateDelegate (in nsIRDFResource aOuter, in string aKey, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult aResult); */
    #[inline]
    pub unsafe fn CreateDelegate<T: XpCom>(&self, aOuter: Option<&nsIRDFResource>, aKey: *const libc::c_char) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut aResult : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).CreateDelegate)(self as *const _, aOuter.map_or(::std::ptr::null(), |x| x as *const _), aKey, &T::iid(), aResult.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aResult.refptr())
    }

}



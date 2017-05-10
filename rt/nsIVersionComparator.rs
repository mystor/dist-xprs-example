//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIVersionComparator.idl
//


#[repr(C)]
pub struct nsIVersionComparator {
    vtable: *const nsIVersionComparatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIVersionComparator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe6cd620a, 0xedbb, 0x41d2,
            [0x9e, 0x42, 0x9a, 0x2f, 0xfc, 0x81, 0x07, 0xf3])
    }
}

unsafe impl RefCounted for nsIVersionComparator {
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
pub trait nsIVersionComparatorCoerce {
    fn coerce_from(v: &nsIVersionComparator) -> &Self;
}

impl nsIVersionComparatorCoerce for nsIVersionComparator {
    #[inline]
    fn coerce_from(v: &nsIVersionComparator) -> &Self {
        v
    }
}

impl nsIVersionComparator {
    #[inline]
    pub fn coerce<T: nsIVersionComparatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIVersionComparator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIVersionComparatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIVersionComparator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIVersionComparatorVTable {
    pub __base: nsISupportsVTable,

    /* long compare (in ACString A, in ACString B); */
    pub compare: unsafe extern "C" fn (this: *const nsIVersionComparator, A: *const nsACString, B: *const nsACString, _retval: *mut libc::int32_t) -> nsresult,

}


impl nsIVersionComparator {
    /* long compare (in ACString A, in ACString B); */
    #[inline]
    pub unsafe fn compare(&self, A: &[u8], B: &[u8]) -> Result<libc::int32_t, nsresult> {
        let A = nsCString::from(A);
        let B = nsCString::from(B);
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).compare)(self as *const _, &*A, &*B, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



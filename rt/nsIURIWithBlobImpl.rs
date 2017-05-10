//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURIWithBlobImpl.idl
//


#[repr(C)]
pub struct nsIURIWithBlobImpl {
    vtable: *const nsIURIWithBlobImplVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIURIWithBlobImpl {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x331b41d3, 0x3506, 0x4ab5,
            [0xbe, 0xf9, 0xaa, 0xb4, 0x1e, 0x32, 0x02, 0xa3])
    }
}

unsafe impl RefCounted for nsIURIWithBlobImpl {
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
pub trait nsIURIWithBlobImplCoerce {
    fn coerce_from(v: &nsIURIWithBlobImpl) -> &Self;
}

impl nsIURIWithBlobImplCoerce for nsIURIWithBlobImpl {
    #[inline]
    fn coerce_from(v: &nsIURIWithBlobImpl) -> &Self {
        v
    }
}

impl nsIURIWithBlobImpl {
    #[inline]
    pub fn coerce<T: nsIURIWithBlobImplCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIURIWithBlobImpl {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIURIWithBlobImplCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIWithBlobImpl) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIURIWithBlobImplVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsISupports blobImpl; */
    pub get_blobImpl: unsafe extern "C" fn (this: *const nsIURIWithBlobImpl, aBlobImpl: *mut *const nsISupports) -> nsresult,

}


impl nsIURIWithBlobImpl {
    /* readonly attribute nsISupports blobImpl; */
    #[inline]
    pub unsafe fn get_blobImpl(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_blobImpl)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICloneableInputStream.idl
//


#[repr(C)]
pub struct nsICloneableInputStream {
    vtable: *const nsICloneableInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICloneableInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8149be1f, 0x44d3, 0x4f14,
            [0x8b, 0x65, 0xa5, 0x7a, 0x5f, 0xbb, 0xeb, 0x97])
    }
}

unsafe impl RefCounted for nsICloneableInputStream {
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
pub trait nsICloneableInputStreamCoerce {
    fn coerce_from(v: &nsICloneableInputStream) -> &Self;
}

impl nsICloneableInputStreamCoerce for nsICloneableInputStream {
    #[inline]
    fn coerce_from(v: &nsICloneableInputStream) -> &Self {
        v
    }
}

impl nsICloneableInputStream {
    #[inline]
    pub fn coerce<T: nsICloneableInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICloneableInputStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICloneableInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICloneableInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICloneableInputStreamVTable {
    pub __base: nsISupportsVTable,

    /* [infallible] readonly attribute boolean cloneable; */
    pub get_cloneable: unsafe extern "C" fn (this: *const nsICloneableInputStream, aCloneable: *mut bool) -> nsresult,

    /* nsIInputStream clone (); */
    pub clone: unsafe extern "C" fn (this: *const nsICloneableInputStream, _retval: *mut *const nsIInputStream) -> nsresult,

}


impl nsICloneableInputStream {
    /* [infallible] readonly attribute boolean cloneable; */
    #[inline]
    pub unsafe fn get_cloneable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_cloneable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIInputStream clone (); */
    #[inline]
    pub unsafe fn clone(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



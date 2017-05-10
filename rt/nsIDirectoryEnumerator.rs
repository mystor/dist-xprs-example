//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDirectoryEnumerator.idl
//


#[repr(C)]
pub struct nsIDirectoryEnumerator {
    vtable: *const nsIDirectoryEnumeratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDirectoryEnumerator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x31f7f4ae, 0x6916, 0x4f2d,
            [0xa8, 0x1e, 0x92, 0x6a, 0x4e, 0x30, 0x22, 0xee])
    }
}

unsafe impl RefCounted for nsIDirectoryEnumerator {
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
pub trait nsIDirectoryEnumeratorCoerce {
    fn coerce_from(v: &nsIDirectoryEnumerator) -> &Self;
}

impl nsIDirectoryEnumeratorCoerce for nsIDirectoryEnumerator {
    #[inline]
    fn coerce_from(v: &nsIDirectoryEnumerator) -> &Self {
        v
    }
}

impl nsIDirectoryEnumerator {
    #[inline]
    pub fn coerce<T: nsIDirectoryEnumeratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDirectoryEnumerator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDirectoryEnumeratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirectoryEnumerator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDirectoryEnumeratorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIFile nextFile; */
    pub get_nextFile: unsafe extern "C" fn (this: *const nsIDirectoryEnumerator, aNextFile: *mut *const nsIFile) -> nsresult,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIDirectoryEnumerator) -> nsresult,

}


impl nsIDirectoryEnumerator {
    /* readonly attribute nsIFile nextFile; */
    #[inline]
    pub unsafe fn get_nextFile(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_nextFile)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

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



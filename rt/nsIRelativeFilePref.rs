//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRelativeFilePref.idl
//


#[repr(C)]
pub struct nsIRelativeFilePref {
    vtable: *const nsIRelativeFilePrefVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRelativeFilePref {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2f977d4e, 0x5485, 0x11d4,
            [0x87, 0xe2, 0x00, 0x10, 0xa4, 0xe7, 0x5e, 0xf2])
    }
}

unsafe impl RefCounted for nsIRelativeFilePref {
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
pub trait nsIRelativeFilePrefCoerce {
    fn coerce_from(v: &nsIRelativeFilePref) -> &Self;
}

impl nsIRelativeFilePrefCoerce for nsIRelativeFilePref {
    #[inline]
    fn coerce_from(v: &nsIRelativeFilePref) -> &Self {
        v
    }
}

impl nsIRelativeFilePref {
    #[inline]
    pub fn coerce<T: nsIRelativeFilePrefCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRelativeFilePref {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRelativeFilePrefCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRelativeFilePref) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRelativeFilePrefVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIFile file; */
    pub get_file: unsafe extern "C" fn (this: *const nsIRelativeFilePref, aFile: *mut *const nsIFile) -> nsresult,
    pub set_file: unsafe extern "C" fn (this: *const nsIRelativeFilePref, aFile: *const nsIFile) -> nsresult,

    /* attribute ACString relativeToKey; */
    pub get_relativeToKey: unsafe extern "C" fn (this: *const nsIRelativeFilePref, aRelativeToKey: *mut nsACString) -> nsresult,
    pub set_relativeToKey: unsafe extern "C" fn (this: *const nsIRelativeFilePref, aRelativeToKey: *const nsACString) -> nsresult,

}


impl nsIRelativeFilePref {
    /* attribute nsIFile file; */
    #[inline]
    pub unsafe fn get_file(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_file)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_file(&self, aFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).set_file)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute ACString relativeToKey; */
    #[inline]
    pub unsafe fn get_relativeToKey(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_relativeToKey)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_relativeToKey(&self, aRelativeToKey: &[u8]) -> Result<(), nsresult> {
        let aRelativeToKey = nsCString::from(aRelativeToKey);
        match ((*self.vtable).set_relativeToKey)(self as *const _, &*aRelativeToKey) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



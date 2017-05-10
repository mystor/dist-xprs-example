//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFileURL.idl
//


#[repr(C)]
pub struct nsIFileURL {
    vtable: *const nsIFileURLVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFileURL {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe91ac988, 0x27c2, 0x448b,
            [0xb1, 0xa1, 0x38, 0x22, 0xe1, 0xef, 0x19, 0x87])
    }
}

unsafe impl RefCounted for nsIFileURL {
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
pub trait nsIFileURLCoerce {
    fn coerce_from(v: &nsIFileURL) -> &Self;
}

impl nsIFileURLCoerce for nsIFileURL {
    #[inline]
    fn coerce_from(v: &nsIFileURL) -> &Self {
        v
    }
}

impl nsIFileURL {
    #[inline]
    pub fn coerce<T: nsIFileURLCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFileURL {
    type Target = nsIURL;
    #[inline]
    fn deref(&self) -> &nsIURL {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIURLCoerce> nsIFileURLCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileURL) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFileURLVTable {
    pub __base: nsIURLVTable,

    /* attribute nsIFile file; */
    pub get_file: unsafe extern "C" fn (this: *const nsIFileURL, aFile: *mut *const nsIFile) -> nsresult,
    pub set_file: unsafe extern "C" fn (this: *const nsIFileURL, aFile: *const nsIFile) -> nsresult,

}


impl nsIFileURL {
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

}



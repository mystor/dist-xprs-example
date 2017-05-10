//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentViewerFile.idl
//


#[repr(C)]
pub struct nsIContentViewerFile {
    vtable: *const nsIContentViewerFileVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentViewerFile {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x564a3276, 0x6228, 0x401e,
            [0x9b, 0x5c, 0xd8, 0x2c, 0xb3, 0x82, 0xa6, 0x0f])
    }
}

unsafe impl RefCounted for nsIContentViewerFile {
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
pub trait nsIContentViewerFileCoerce {
    fn coerce_from(v: &nsIContentViewerFile) -> &Self;
}

impl nsIContentViewerFileCoerce for nsIContentViewerFile {
    #[inline]
    fn coerce_from(v: &nsIContentViewerFile) -> &Self {
        v
    }
}

impl nsIContentViewerFile {
    #[inline]
    pub fn coerce<T: nsIContentViewerFileCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentViewerFile {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentViewerFileCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentViewerFile) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentViewerFileVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean printable; */
    pub get_printable: unsafe extern "C" fn (this: *const nsIContentViewerFile, aPrintable: *mut bool) -> nsresult,

    /* [noscript] void print (in boolean aSilent, in FILE aDebugFile, in nsIPrintSettings aPrintSettings); */
    /// Unable to call function as its signature contains a non-rust type
    pub print: *const ::libc::c_void,

}


impl nsIContentViewerFile {
    /* readonly attribute boolean printable; */
    #[inline]
    pub unsafe fn get_printable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_printable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void print (in boolean aSilent, in FILE aDebugFile, in nsIPrintSettings aPrintSettings); */


}



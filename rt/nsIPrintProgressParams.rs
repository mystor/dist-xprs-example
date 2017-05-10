//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrintProgressParams.idl
//


#[repr(C)]
pub struct nsIPrintProgressParams {
    vtable: *const nsIPrintProgressParamsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrintProgressParams {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xca89b55b, 0x6faf, 0x4051,
            [0x96, 0x45, 0x1c, 0x03, 0xef, 0x51, 0x08, 0xf8])
    }
}

unsafe impl RefCounted for nsIPrintProgressParams {
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
pub trait nsIPrintProgressParamsCoerce {
    fn coerce_from(v: &nsIPrintProgressParams) -> &Self;
}

impl nsIPrintProgressParamsCoerce for nsIPrintProgressParams {
    #[inline]
    fn coerce_from(v: &nsIPrintProgressParams) -> &Self {
        v
    }
}

impl nsIPrintProgressParams {
    #[inline]
    pub fn coerce<T: nsIPrintProgressParamsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrintProgressParams {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrintProgressParamsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrintProgressParams) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrintProgressParamsVTable {
    pub __base: nsISupportsVTable,

    /* attribute wstring docTitle; */
    pub get_docTitle: unsafe extern "C" fn (this: *const nsIPrintProgressParams, aDocTitle: *mut *const libc::int16_t) -> nsresult,
    pub set_docTitle: unsafe extern "C" fn (this: *const nsIPrintProgressParams, aDocTitle: *const libc::int16_t) -> nsresult,

    /* attribute wstring docURL; */
    pub get_docURL: unsafe extern "C" fn (this: *const nsIPrintProgressParams, aDocURL: *mut *const libc::int16_t) -> nsresult,
    pub set_docURL: unsafe extern "C" fn (this: *const nsIPrintProgressParams, aDocURL: *const libc::int16_t) -> nsresult,

}


impl nsIPrintProgressParams {
    /* attribute wstring docTitle; */
    #[inline]
    pub unsafe fn get_docTitle(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_docTitle)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_docTitle(&self, aDocTitle: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_docTitle)(self as *const _, aDocTitle) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring docURL; */
    #[inline]
    pub unsafe fn get_docURL(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_docURL)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_docURL(&self, aDocURL: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_docURL)(self as *const _, aDocURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



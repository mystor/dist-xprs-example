//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUnicharOutputStream.idl
//


#[repr(C)]
pub struct nsIUnicharOutputStream {
    vtable: *const nsIUnicharOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUnicharOutputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2d00b1bb, 0x8b21, 0x4a63,
            [0xbc, 0xc6, 0x72, 0x13, 0xf5, 0x13, 0xac, 0x2e])
    }
}

unsafe impl RefCounted for nsIUnicharOutputStream {
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
pub trait nsIUnicharOutputStreamCoerce {
    fn coerce_from(v: &nsIUnicharOutputStream) -> &Self;
}

impl nsIUnicharOutputStreamCoerce for nsIUnicharOutputStream {
    #[inline]
    fn coerce_from(v: &nsIUnicharOutputStream) -> &Self {
        v
    }
}

impl nsIUnicharOutputStream {
    #[inline]
    pub fn coerce<T: nsIUnicharOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUnicharOutputStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUnicharOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUnicharOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUnicharOutputStreamVTable {
    pub __base: nsISupportsVTable,

    /* boolean write (in unsigned long aCount, [array, size_is (aCount), const] in char16_t c); */
    /// Unable to call function as its signature contains a non-rust type
    pub write: *const ::libc::c_void,

    /* boolean writeString (in AString str); */
    pub writeString: unsafe extern "C" fn (this: *const nsIUnicharOutputStream, str: *const nsAString, _retval: *mut bool) -> nsresult,

    /* void flush (); */
    pub flush: unsafe extern "C" fn (this: *const nsIUnicharOutputStream) -> nsresult,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIUnicharOutputStream) -> nsresult,

}


impl nsIUnicharOutputStream {
    /* boolean write (in unsigned long aCount, [array, size_is (aCount), const] in char16_t c); */


    /* boolean writeString (in AString str); */
    #[inline]
    pub unsafe fn writeString(&self, str: &[u16]) -> Result<bool, nsresult> {
        let str = nsString::from(str);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).writeString)(self as *const _, &*str, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void flush (); */
    #[inline]
    pub unsafe fn flush(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).flush)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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



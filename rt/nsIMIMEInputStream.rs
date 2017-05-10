//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMIMEInputStream.idl
//


#[repr(C)]
pub struct nsIMIMEInputStream {
    vtable: *const nsIMIMEInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMIMEInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdcbce63c, 0x1dd1, 0x11b2,
            [0xb9, 0x4d, 0x91, 0xf6, 0xd4, 0x9a, 0x31, 0x61])
    }
}

unsafe impl RefCounted for nsIMIMEInputStream {
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
pub trait nsIMIMEInputStreamCoerce {
    fn coerce_from(v: &nsIMIMEInputStream) -> &Self;
}

impl nsIMIMEInputStreamCoerce for nsIMIMEInputStream {
    #[inline]
    fn coerce_from(v: &nsIMIMEInputStream) -> &Self {
        v
    }
}

impl nsIMIMEInputStream {
    #[inline]
    pub fn coerce<T: nsIMIMEInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMIMEInputStream {
    type Target = nsIInputStream;
    #[inline]
    fn deref(&self) -> &nsIInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIInputStreamCoerce> nsIMIMEInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMIMEInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMIMEInputStreamVTable {
    pub __base: nsIInputStreamVTable,

    /* attribute boolean addContentLength; */
    pub get_addContentLength: unsafe extern "C" fn (this: *const nsIMIMEInputStream, aAddContentLength: *mut bool) -> nsresult,
    pub set_addContentLength: unsafe extern "C" fn (this: *const nsIMIMEInputStream, aAddContentLength: bool) -> nsresult,

    /* void addHeader (in string name, in string value); */
    pub addHeader: unsafe extern "C" fn (this: *const nsIMIMEInputStream, name: *const libc::c_char, value: *const libc::c_char) -> nsresult,

    /* void visitHeaders (in nsIHttpHeaderVisitor visitor); */
    pub visitHeaders: unsafe extern "C" fn (this: *const nsIMIMEInputStream, visitor: *const nsIHttpHeaderVisitor) -> nsresult,

    /* void setData (in nsIInputStream stream); */
    pub setData: unsafe extern "C" fn (this: *const nsIMIMEInputStream, stream: *const nsIInputStream) -> nsresult,

    /* readonly attribute nsIInputStream data; */
    pub get_data: unsafe extern "C" fn (this: *const nsIMIMEInputStream, aData: *mut *const nsIInputStream) -> nsresult,

}


impl nsIMIMEInputStream {
    /* attribute boolean addContentLength; */
    #[inline]
    pub unsafe fn get_addContentLength(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_addContentLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_addContentLength(&self, aAddContentLength: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_addContentLength)(self as *const _, aAddContentLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addHeader (in string name, in string value); */
    #[inline]
    pub unsafe fn addHeader(&self, name: *const libc::c_char, value: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).addHeader)(self as *const _, name, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void visitHeaders (in nsIHttpHeaderVisitor visitor); */
    #[inline]
    pub unsafe fn visitHeaders(&self, visitor: Option<&nsIHttpHeaderVisitor>) -> Result<(), nsresult> {

        match ((*self.vtable).visitHeaders)(self as *const _, visitor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setData (in nsIInputStream stream); */
    #[inline]
    pub unsafe fn setData(&self, stream: Option<&nsIInputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).setData)(self as *const _, stream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIInputStream data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_data)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



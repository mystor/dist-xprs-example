//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptableInputStream.idl
//


#[repr(C)]
pub struct nsIScriptableInputStream {
    vtable: *const nsIScriptableInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScriptableInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3fce9015, 0x472a, 0x4080,
            [0xac, 0x3e, 0xcd, 0x87, 0x5d, 0xbe, 0x36, 0x1e])
    }
}

unsafe impl RefCounted for nsIScriptableInputStream {
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
pub trait nsIScriptableInputStreamCoerce {
    fn coerce_from(v: &nsIScriptableInputStream) -> &Self;
}

impl nsIScriptableInputStreamCoerce for nsIScriptableInputStream {
    #[inline]
    fn coerce_from(v: &nsIScriptableInputStream) -> &Self {
        v
    }
}

impl nsIScriptableInputStream {
    #[inline]
    pub fn coerce<T: nsIScriptableInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScriptableInputStream {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScriptableInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptableInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScriptableInputStreamVTable {
    pub __base: nsISupportsVTable,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIScriptableInputStream) -> nsresult,

    /* void init (in nsIInputStream aInputStream); */
    pub init: unsafe extern "C" fn (this: *const nsIScriptableInputStream, aInputStream: *const nsIInputStream) -> nsresult,

    /* unsigned long long available (); */
    pub available: unsafe extern "C" fn (this: *const nsIScriptableInputStream, _retval: *mut libc::uint64_t) -> nsresult,

    /* string read (in unsigned long aCount); */
    pub read: unsafe extern "C" fn (this: *const nsIScriptableInputStream, aCount: libc::uint32_t, _retval: *mut *const libc::c_char) -> nsresult,

    /* ACString readBytes (in unsigned long aCount); */
    pub readBytes: unsafe extern "C" fn (this: *const nsIScriptableInputStream, aCount: libc::uint32_t, _retval: *mut nsACString) -> nsresult,

}


impl nsIScriptableInputStream {
    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void init (in nsIInputStream aInputStream); */
    #[inline]
    pub unsafe fn init(&self, aInputStream: Option<&nsIInputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aInputStream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* unsigned long long available (); */
    #[inline]
    pub unsafe fn available(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).available)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* string read (in unsigned long aCount); */
    #[inline]
    pub unsafe fn read(&self, aCount: libc::uint32_t) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).read)(self as *const _, aCount, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString readBytes (in unsigned long aCount); */
    #[inline]
    pub unsafe fn readBytes(&self, aCount: libc::uint32_t) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).readBytes)(self as *const _, aCount, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



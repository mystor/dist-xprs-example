//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHTTPHeaderListener.idl
//


#[repr(C)]
pub struct nsIHTTPHeaderListener {
    vtable: *const nsIHTTPHeaderListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHTTPHeaderListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xea51e0b8, 0x871c, 0x4b85,
            [0x92, 0xda, 0x6f, 0x40, 0x03, 0x94, 0xc5, 0xec])
    }
}

unsafe impl RefCounted for nsIHTTPHeaderListener {
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
pub trait nsIHTTPHeaderListenerCoerce {
    fn coerce_from(v: &nsIHTTPHeaderListener) -> &Self;
}

impl nsIHTTPHeaderListenerCoerce for nsIHTTPHeaderListener {
    #[inline]
    fn coerce_from(v: &nsIHTTPHeaderListener) -> &Self {
        v
    }
}

impl nsIHTTPHeaderListener {
    #[inline]
    pub fn coerce<T: nsIHTTPHeaderListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHTTPHeaderListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHTTPHeaderListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHTTPHeaderListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHTTPHeaderListenerVTable {
    pub __base: nsISupportsVTable,

    /* void newResponseHeader (in string headerName, in string headerValue); */
    pub newResponseHeader: unsafe extern "C" fn (this: *const nsIHTTPHeaderListener, headerName: *const libc::c_char, headerValue: *const libc::c_char) -> nsresult,

    /* void statusLine (in string line); */
    pub statusLine: unsafe extern "C" fn (this: *const nsIHTTPHeaderListener, line: *const libc::c_char) -> nsresult,

}


impl nsIHTTPHeaderListener {
    /* void newResponseHeader (in string headerName, in string headerValue); */
    #[inline]
    pub unsafe fn newResponseHeader(&self, headerName: *const libc::c_char, headerValue: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).newResponseHeader)(self as *const _, headerName, headerValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void statusLine (in string line); */
    #[inline]
    pub unsafe fn statusLine(&self, line: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).statusLine)(self as *const _, line) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



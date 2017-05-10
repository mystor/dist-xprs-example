//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamBufferAccess.idl
//


#[repr(C)]
pub struct nsIStreamBufferAccess {
    vtable: *const nsIStreamBufferAccessVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStreamBufferAccess {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xac923b72, 0xac87, 0x4892,
            [0xac, 0x7a, 0xca, 0x38, 0x5d, 0x42, 0x94, 0x35])
    }
}

unsafe impl RefCounted for nsIStreamBufferAccess {
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
pub trait nsIStreamBufferAccessCoerce {
    fn coerce_from(v: &nsIStreamBufferAccess) -> &Self;
}

impl nsIStreamBufferAccessCoerce for nsIStreamBufferAccess {
    #[inline]
    fn coerce_from(v: &nsIStreamBufferAccess) -> &Self {
        v
    }
}

impl nsIStreamBufferAccess {
    #[inline]
    pub fn coerce<T: nsIStreamBufferAccessCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStreamBufferAccess {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStreamBufferAccessCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamBufferAccess) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStreamBufferAccessVTable {
    pub __base: nsISupportsVTable,

    /* [noscript,notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask); */
    pub getBuffer: unsafe extern "C" fn (this: *const nsIStreamBufferAccess, aLength: uint32_t, aAlignMask: uint32_t) -> *const u8,

    /* [noscript,notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength); */
    pub putBuffer: unsafe extern "C" fn (this: *const nsIStreamBufferAccess, aBuffer: *const u8, aLength: uint32_t) -> libc::c_void,

    /* void disableBuffering (); */
    pub disableBuffering: unsafe extern "C" fn (this: *const nsIStreamBufferAccess) -> nsresult,

    /* void enableBuffering (); */
    pub enableBuffering: unsafe extern "C" fn (this: *const nsIStreamBufferAccess) -> nsresult,

    /* readonly attribute nsISupports unbufferedStream; */
    pub get_unbufferedStream: unsafe extern "C" fn (this: *const nsIStreamBufferAccess, aUnbufferedStream: *mut *const nsISupports) -> nsresult,

}


impl nsIStreamBufferAccess {
    /* [noscript,notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask); */
    #[inline]
    pub unsafe fn getBuffer(&self, aLength: uint32_t, aAlignMask: uint32_t) -> *const u8 {

        let _retval = ((*self.vtable).getBuffer)(self as *const _, aLength, aAlignMask);
        _retval
    }

    /* [noscript,notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength); */
    #[inline]
    pub unsafe fn putBuffer(&self, aBuffer: *const u8, aLength: uint32_t) -> () {

        let _retval = ((*self.vtable).putBuffer)(self as *const _, aBuffer, aLength);
        ()
    }

    /* void disableBuffering (); */
    #[inline]
    pub unsafe fn disableBuffering(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).disableBuffering)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void enableBuffering (); */
    #[inline]
    pub unsafe fn enableBuffering(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).enableBuffering)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsISupports unbufferedStream; */
    #[inline]
    pub unsafe fn get_unbufferedStream(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_unbufferedStream)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



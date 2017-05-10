//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIExtendedExpatSink.idl
//


#[repr(C)]
pub struct nsIExtendedExpatSink {
    vtable: *const nsIExtendedExpatSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIExtendedExpatSink {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5e3e4f0c, 0x7b77, 0x47ca,
            [0xa7, 0xc5, 0xa3, 0xd8, 0x7f, 0x2a, 0x9c, 0x82])
    }
}

unsafe impl RefCounted for nsIExtendedExpatSink {
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
pub trait nsIExtendedExpatSinkCoerce {
    fn coerce_from(v: &nsIExtendedExpatSink) -> &Self;
}

impl nsIExtendedExpatSinkCoerce for nsIExtendedExpatSink {
    #[inline]
    fn coerce_from(v: &nsIExtendedExpatSink) -> &Self {
        v
    }
}

impl nsIExtendedExpatSink {
    #[inline]
    pub fn coerce<T: nsIExtendedExpatSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIExtendedExpatSink {
    type Target = nsIExpatSink;
    #[inline]
    fn deref(&self) -> &nsIExpatSink {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIExpatSinkCoerce> nsIExtendedExpatSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExtendedExpatSink) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIExtendedExpatSinkVTable {
    pub __base: nsIExpatSinkVTable,

    /* void handleStartDTD (in wstring aDoctypeName, in wstring aSysid, in wstring aPubid); */
    pub handleStartDTD: unsafe extern "C" fn (this: *const nsIExtendedExpatSink, aDoctypeName: *const libc::int16_t, aSysid: *const libc::int16_t, aPubid: *const libc::int16_t) -> nsresult,

    /* void handleStartNamespaceDecl (in wstring aPrefix, in wstring aUri); */
    pub handleStartNamespaceDecl: unsafe extern "C" fn (this: *const nsIExtendedExpatSink, aPrefix: *const libc::int16_t, aUri: *const libc::int16_t) -> nsresult,

    /* void handleEndNamespaceDecl (in wstring aPrefix); */
    pub handleEndNamespaceDecl: unsafe extern "C" fn (this: *const nsIExtendedExpatSink, aPrefix: *const libc::int16_t) -> nsresult,

    /* void handleNotationDecl (in wstring aNotationName, in wstring aSysid, in wstring aPubid); */
    pub handleNotationDecl: unsafe extern "C" fn (this: *const nsIExtendedExpatSink, aNotationName: *const libc::int16_t, aSysid: *const libc::int16_t, aPubid: *const libc::int16_t) -> nsresult,

    /* void handleUnparsedEntityDecl (in wstring aName, in wstring aSysid, in wstring aPubid, in wstring aNotationName); */
    pub handleUnparsedEntityDecl: unsafe extern "C" fn (this: *const nsIExtendedExpatSink, aName: *const libc::int16_t, aSysid: *const libc::int16_t, aPubid: *const libc::int16_t, aNotationName: *const libc::int16_t) -> nsresult,

}


impl nsIExtendedExpatSink {
    /* void handleStartDTD (in wstring aDoctypeName, in wstring aSysid, in wstring aPubid); */
    #[inline]
    pub unsafe fn handleStartDTD(&self, aDoctypeName: *const libc::int16_t, aSysid: *const libc::int16_t, aPubid: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).handleStartDTD)(self as *const _, aDoctypeName, aSysid, aPubid) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleStartNamespaceDecl (in wstring aPrefix, in wstring aUri); */
    #[inline]
    pub unsafe fn handleStartNamespaceDecl(&self, aPrefix: *const libc::int16_t, aUri: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).handleStartNamespaceDecl)(self as *const _, aPrefix, aUri) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleEndNamespaceDecl (in wstring aPrefix); */
    #[inline]
    pub unsafe fn handleEndNamespaceDecl(&self, aPrefix: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).handleEndNamespaceDecl)(self as *const _, aPrefix) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleNotationDecl (in wstring aNotationName, in wstring aSysid, in wstring aPubid); */
    #[inline]
    pub unsafe fn handleNotationDecl(&self, aNotationName: *const libc::int16_t, aSysid: *const libc::int16_t, aPubid: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).handleNotationDecl)(self as *const _, aNotationName, aSysid, aPubid) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleUnparsedEntityDecl (in wstring aName, in wstring aSysid, in wstring aPubid, in wstring aNotationName); */
    #[inline]
    pub unsafe fn handleUnparsedEntityDecl(&self, aName: *const libc::int16_t, aSysid: *const libc::int16_t, aPubid: *const libc::int16_t, aNotationName: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).handleUnparsedEntityDecl)(self as *const _, aName, aSysid, aPubid, aNotationName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



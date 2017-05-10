//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXAttributes.idl
//


#[repr(C)]
pub struct nsISAXAttributes {
    vtable: *const nsISAXAttributesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISAXAttributes {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe347005e, 0x6cd0, 0x11da,
            [0xbe, 0x43, 0x00, 0x14, 0x22, 0x10, 0x69, 0x90])
    }
}

unsafe impl RefCounted for nsISAXAttributes {
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
pub trait nsISAXAttributesCoerce {
    fn coerce_from(v: &nsISAXAttributes) -> &Self;
}

impl nsISAXAttributesCoerce for nsISAXAttributes {
    #[inline]
    fn coerce_from(v: &nsISAXAttributes) -> &Self {
        v
    }
}

impl nsISAXAttributes {
    #[inline]
    pub fn coerce<T: nsISAXAttributesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISAXAttributes {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISAXAttributesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISAXAttributes) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISAXAttributesVTable {
    pub __base: nsISupportsVTable,

    /* long getIndexFromName (in AString uri, in AString localName); */
    pub getIndexFromName: unsafe extern "C" fn (this: *const nsISAXAttributes, uri: *const nsAString, localName: *const nsAString, _retval: *mut libc::int32_t) -> nsresult,

    /* long getIndexFromQName (in AString qName); */
    pub getIndexFromQName: unsafe extern "C" fn (this: *const nsISAXAttributes, qName: *const nsAString, _retval: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsISAXAttributes, aLength: *mut libc::int32_t) -> nsresult,

    /* AString getLocalName (in unsigned long index); */
    pub getLocalName: unsafe extern "C" fn (this: *const nsISAXAttributes, index: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getQName (in unsigned long index); */
    pub getQName: unsafe extern "C" fn (this: *const nsISAXAttributes, index: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getType (in unsigned long index); */
    pub getType: unsafe extern "C" fn (this: *const nsISAXAttributes, index: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getTypeFromName (in AString uri, in AString localName); */
    pub getTypeFromName: unsafe extern "C" fn (this: *const nsISAXAttributes, uri: *const nsAString, localName: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* AString getTypeFromQName (in AString qName); */
    pub getTypeFromQName: unsafe extern "C" fn (this: *const nsISAXAttributes, qName: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* AString getURI (in unsigned long index); */
    pub getURI: unsafe extern "C" fn (this: *const nsISAXAttributes, index: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getValue (in unsigned long index); */
    pub getValue: unsafe extern "C" fn (this: *const nsISAXAttributes, index: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getValueFromName (in AString uri, in AString localName); */
    pub getValueFromName: unsafe extern "C" fn (this: *const nsISAXAttributes, uri: *const nsAString, localName: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* AString getValueFromQName (in AString qName); */
    pub getValueFromQName: unsafe extern "C" fn (this: *const nsISAXAttributes, qName: *const nsAString, _retval: *mut nsAString) -> nsresult,

}


impl nsISAXAttributes {
    /* long getIndexFromName (in AString uri, in AString localName); */
    #[inline]
    pub unsafe fn getIndexFromName(&self, uri: &[u16], localName: &[u16]) -> Result<libc::int32_t, nsresult> {
        let uri = nsString::from(uri);
        let localName = nsString::from(localName);
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getIndexFromName)(self as *const _, &*uri, &*localName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getIndexFromQName (in AString qName); */
    #[inline]
    pub unsafe fn getIndexFromQName(&self, qName: &[u16]) -> Result<libc::int32_t, nsresult> {
        let qName = nsString::from(qName);
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getIndexFromQName)(self as *const _, &*qName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getLocalName (in unsigned long index); */
    #[inline]
    pub unsafe fn getLocalName(&self, index: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getLocalName)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getQName (in unsigned long index); */
    #[inline]
    pub unsafe fn getQName(&self, index: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getQName)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getType (in unsigned long index); */
    #[inline]
    pub unsafe fn getType(&self, index: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getType)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getTypeFromName (in AString uri, in AString localName); */
    #[inline]
    pub unsafe fn getTypeFromName(&self, uri: &[u16], localName: &[u16]) -> Result<nsString, nsresult> {
        let uri = nsString::from(uri);
        let localName = nsString::from(localName);
        let mut _retval = nsString::new();
        match ((*self.vtable).getTypeFromName)(self as *const _, &*uri, &*localName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getTypeFromQName (in AString qName); */
    #[inline]
    pub unsafe fn getTypeFromQName(&self, qName: &[u16]) -> Result<nsString, nsresult> {
        let qName = nsString::from(qName);
        let mut _retval = nsString::new();
        match ((*self.vtable).getTypeFromQName)(self as *const _, &*qName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getURI (in unsigned long index); */
    #[inline]
    pub unsafe fn getURI(&self, index: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getURI)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getValue (in unsigned long index); */
    #[inline]
    pub unsafe fn getValue(&self, index: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getValue)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getValueFromName (in AString uri, in AString localName); */
    #[inline]
    pub unsafe fn getValueFromName(&self, uri: &[u16], localName: &[u16]) -> Result<nsString, nsresult> {
        let uri = nsString::from(uri);
        let localName = nsString::from(localName);
        let mut _retval = nsString::new();
        match ((*self.vtable).getValueFromName)(self as *const _, &*uri, &*localName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getValueFromQName (in AString qName); */
    #[inline]
    pub unsafe fn getValueFromQName(&self, qName: &[u16]) -> Result<nsString, nsresult> {
        let qName = nsString::from(qName);
        let mut _retval = nsString::new();
        match ((*self.vtable).getValueFromQName)(self as *const _, &*qName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



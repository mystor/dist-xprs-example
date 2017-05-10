//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXMutableAttributes.idl
//


#[repr(C)]
pub struct nsISAXMutableAttributes {
    vtable: *const nsISAXMutableAttributesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISAXMutableAttributes {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8b1de83d, 0xcebb, 0x49fa,
            [0x82, 0x45, 0xc0, 0xfe, 0x31, 0x9e, 0xb7, 0xb6])
    }
}

unsafe impl RefCounted for nsISAXMutableAttributes {
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
pub trait nsISAXMutableAttributesCoerce {
    fn coerce_from(v: &nsISAXMutableAttributes) -> &Self;
}

impl nsISAXMutableAttributesCoerce for nsISAXMutableAttributes {
    #[inline]
    fn coerce_from(v: &nsISAXMutableAttributes) -> &Self {
        v
    }
}

impl nsISAXMutableAttributes {
    #[inline]
    pub fn coerce<T: nsISAXMutableAttributesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISAXMutableAttributes {
    type Target = nsISAXAttributes;
    #[inline]
    fn deref(&self) -> &nsISAXAttributes {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISAXAttributesCoerce> nsISAXMutableAttributesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISAXMutableAttributes) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISAXMutableAttributesVTable {
    pub __base: nsISAXAttributesVTable,

    /* void addAttribute (in AString uri, in AString localName, in AString qName, in AString type, in AString value); */
    pub addAttribute: unsafe extern "C" fn (this: *const nsISAXMutableAttributes, uri: *const nsAString, localName: *const nsAString, qName: *const nsAString, type_: *const nsAString, value: *const nsAString) -> nsresult,

    /* void clear (); */
    pub clear: unsafe extern "C" fn (this: *const nsISAXMutableAttributes) -> nsresult,

    /* void removeAttribute (in unsigned long index); */
    pub removeAttribute: unsafe extern "C" fn (this: *const nsISAXMutableAttributes, index: libc::uint32_t) -> nsresult,

    /* void setAttributes (in nsISAXAttributes attributes); */
    pub setAttributes: unsafe extern "C" fn (this: *const nsISAXMutableAttributes, attributes: *const nsISAXAttributes) -> nsresult,

    /* void setAttribute (in unsigned long index, in AString uri, in AString localName, in AString qName, in AString type, in AString value); */
    pub setAttribute: unsafe extern "C" fn (this: *const nsISAXMutableAttributes, index: libc::uint32_t, uri: *const nsAString, localName: *const nsAString, qName: *const nsAString, type_: *const nsAString, value: *const nsAString) -> nsresult,

    /* void setLocalName (in unsigned long index, in AString localName); */
    pub setLocalName: unsafe extern "C" fn (this: *const nsISAXMutableAttributes, index: libc::uint32_t, localName: *const nsAString) -> nsresult,

    /* void setQName (in unsigned long index, in AString qName); */
    pub setQName: unsafe extern "C" fn (this: *const nsISAXMutableAttributes, index: libc::uint32_t, qName: *const nsAString) -> nsresult,

    /* void setType (in unsigned long index, in AString type); */
    pub setType: unsafe extern "C" fn (this: *const nsISAXMutableAttributes, index: libc::uint32_t, type_: *const nsAString) -> nsresult,

    /* void setURI (in unsigned long index, in AString uri); */
    pub setURI: unsafe extern "C" fn (this: *const nsISAXMutableAttributes, index: libc::uint32_t, uri: *const nsAString) -> nsresult,

    /* void setValue (in unsigned long index, in AString value); */
    pub setValue: unsafe extern "C" fn (this: *const nsISAXMutableAttributes, index: libc::uint32_t, value: *const nsAString) -> nsresult,

}


impl nsISAXMutableAttributes {
    /* void addAttribute (in AString uri, in AString localName, in AString qName, in AString type, in AString value); */
    #[inline]
    pub unsafe fn addAttribute(&self, uri: &[u16], localName: &[u16], qName: &[u16], type_: &[u16], value: &[u16]) -> Result<(), nsresult> {
        let uri = nsString::from(uri);
        let localName = nsString::from(localName);
        let qName = nsString::from(qName);
        let type_ = nsString::from(type_);
        let value = nsString::from(value);
        match ((*self.vtable).addAttribute)(self as *const _, &*uri, &*localName, &*qName, &*type_, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clear (); */
    #[inline]
    pub unsafe fn clear(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clear)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeAttribute (in unsigned long index); */
    #[inline]
    pub unsafe fn removeAttribute(&self, index: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeAttribute)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAttributes (in nsISAXAttributes attributes); */
    #[inline]
    pub unsafe fn setAttributes(&self, attributes: Option<&nsISAXAttributes>) -> Result<(), nsresult> {

        match ((*self.vtable).setAttributes)(self as *const _, attributes.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAttribute (in unsigned long index, in AString uri, in AString localName, in AString qName, in AString type, in AString value); */
    #[inline]
    pub unsafe fn setAttribute(&self, index: libc::uint32_t, uri: &[u16], localName: &[u16], qName: &[u16], type_: &[u16], value: &[u16]) -> Result<(), nsresult> {
        let uri = nsString::from(uri);
        let localName = nsString::from(localName);
        let qName = nsString::from(qName);
        let type_ = nsString::from(type_);
        let value = nsString::from(value);
        match ((*self.vtable).setAttribute)(self as *const _, index, &*uri, &*localName, &*qName, &*type_, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setLocalName (in unsigned long index, in AString localName); */
    #[inline]
    pub unsafe fn setLocalName(&self, index: libc::uint32_t, localName: &[u16]) -> Result<(), nsresult> {
        let localName = nsString::from(localName);
        match ((*self.vtable).setLocalName)(self as *const _, index, &*localName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setQName (in unsigned long index, in AString qName); */
    #[inline]
    pub unsafe fn setQName(&self, index: libc::uint32_t, qName: &[u16]) -> Result<(), nsresult> {
        let qName = nsString::from(qName);
        match ((*self.vtable).setQName)(self as *const _, index, &*qName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setType (in unsigned long index, in AString type); */
    #[inline]
    pub unsafe fn setType(&self, index: libc::uint32_t, type_: &[u16]) -> Result<(), nsresult> {
        let type_ = nsString::from(type_);
        match ((*self.vtable).setType)(self as *const _, index, &*type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setURI (in unsigned long index, in AString uri); */
    #[inline]
    pub unsafe fn setURI(&self, index: libc::uint32_t, uri: &[u16]) -> Result<(), nsresult> {
        let uri = nsString::from(uri);
        match ((*self.vtable).setURI)(self as *const _, index, &*uri) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setValue (in unsigned long index, in AString value); */
    #[inline]
    pub unsafe fn setValue(&self, index: libc::uint32_t, value: &[u16]) -> Result<(), nsresult> {
        let value = nsString::from(value);
        match ((*self.vtable).setValue)(self as *const _, index, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



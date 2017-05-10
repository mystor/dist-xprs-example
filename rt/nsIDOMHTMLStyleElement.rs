//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLStyleElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLStyleElement {
    vtable: *const nsIDOMHTMLStyleElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLStyleElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfa326d22, 0x8739, 0x4eef,
            [0xa8, 0x0e, 0x64, 0x49, 0xbd, 0xe6, 0x05, 0xd2])
    }
}

unsafe impl RefCounted for nsIDOMHTMLStyleElement {
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
pub trait nsIDOMHTMLStyleElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLStyleElement) -> &Self;
}

impl nsIDOMHTMLStyleElementCoerce for nsIDOMHTMLStyleElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLStyleElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLStyleElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLStyleElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLStyleElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLStyleElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLStyleElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLStyleElementVTable {
    pub __base: nsISupportsVTable,

    /* [binaryname(MozDisabled)] attribute boolean disabled; */
    pub get_MozDisabled: unsafe extern "C" fn (this: *const nsIDOMHTMLStyleElement, aDisabled: *mut bool) -> nsresult,
    pub set_MozDisabled: unsafe extern "C" fn (this: *const nsIDOMHTMLStyleElement, aDisabled: bool) -> nsresult,

    /* attribute DOMString media; */
    pub get_media: unsafe extern "C" fn (this: *const nsIDOMHTMLStyleElement, aMedia: *mut nsAString) -> nsresult,
    pub set_media: unsafe extern "C" fn (this: *const nsIDOMHTMLStyleElement, aMedia: *const nsAString) -> nsresult,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLStyleElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLStyleElement, aType: *const nsAString) -> nsresult,

    /* attribute boolean scoped; */
    pub get_scoped: unsafe extern "C" fn (this: *const nsIDOMHTMLStyleElement, aScoped: *mut bool) -> nsresult,
    pub set_scoped: unsafe extern "C" fn (this: *const nsIDOMHTMLStyleElement, aScoped: bool) -> nsresult,

}


impl nsIDOMHTMLStyleElement {
    /* [binaryname(MozDisabled)] attribute boolean disabled; */
    #[inline]
    pub unsafe fn get_MozDisabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_MozDisabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_MozDisabled(&self, aDisabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_MozDisabled)(self as *const _, aDisabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString media; */
    #[inline]
    pub unsafe fn get_media(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_media)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_media(&self, aMedia: &[u16]) -> Result<(), nsresult> {
        let aMedia = nsString::from(aMedia);
        match ((*self.vtable).set_media)(self as *const _, &*aMedia) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_type_(&self, aType: &[u16]) -> Result<(), nsresult> {
        let aType = nsString::from(aType);
        match ((*self.vtable).set_type_)(self as *const _, &*aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean scoped; */
    #[inline]
    pub unsafe fn get_scoped(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_scoped)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_scoped(&self, aScoped: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_scoped)(self as *const _, aScoped) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



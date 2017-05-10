//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSStyleDeclaration.idl
//


#[repr(C)]
pub struct nsIDOMCSSStyleDeclaration {
    vtable: *const nsIDOMCSSStyleDeclarationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSStyleDeclaration {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa6cf90be, 0x15b3, 0x11d2,
            [0x93, 0x2e, 0x00, 0x80, 0x5f, 0x8a, 0xdd, 0x32])
    }
}

unsafe impl RefCounted for nsIDOMCSSStyleDeclaration {
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
pub trait nsIDOMCSSStyleDeclarationCoerce {
    fn coerce_from(v: &nsIDOMCSSStyleDeclaration) -> &Self;
}

impl nsIDOMCSSStyleDeclarationCoerce for nsIDOMCSSStyleDeclaration {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSStyleDeclaration) -> &Self {
        v
    }
}

impl nsIDOMCSSStyleDeclaration {
    #[inline]
    pub fn coerce<T: nsIDOMCSSStyleDeclarationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSStyleDeclaration {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSStyleDeclarationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSStyleDeclaration) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSStyleDeclarationVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString cssText; */
    pub get_cssText: unsafe extern "C" fn (this: *const nsIDOMCSSStyleDeclaration, aCssText: *mut nsAString) -> nsresult,
    pub set_cssText: unsafe extern "C" fn (this: *const nsIDOMCSSStyleDeclaration, aCssText: *const nsAString) -> nsresult,

    /* DOMString getPropertyValue (in DOMString propertyName); */
    pub getPropertyValue: unsafe extern "C" fn (this: *const nsIDOMCSSStyleDeclaration, propertyName: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* nsIDOMCSSValue getPropertyCSSValue (in DOMString propertyName); */
    pub getPropertyCSSValue: unsafe extern "C" fn (this: *const nsIDOMCSSStyleDeclaration, propertyName: *const nsAString, _retval: *mut *const nsIDOMCSSValue) -> nsresult,

    /* DOMString removeProperty (in DOMString propertyName) raises (DOMException); */
    pub removeProperty: unsafe extern "C" fn (this: *const nsIDOMCSSStyleDeclaration, propertyName: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* DOMString getPropertyPriority (in DOMString propertyName); */
    pub getPropertyPriority: unsafe extern "C" fn (this: *const nsIDOMCSSStyleDeclaration, propertyName: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* void setProperty (in DOMString propertyName, in DOMString value, [optional] in DOMString priority) raises (DOMException); */
    pub setProperty: unsafe extern "C" fn (this: *const nsIDOMCSSStyleDeclaration, propertyName: *const nsAString, value: *const nsAString, priority: *const nsAString) -> nsresult,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMCSSStyleDeclaration, aLength: *mut libc::uint32_t) -> nsresult,

    /* DOMString item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const nsIDOMCSSStyleDeclaration, index: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* readonly attribute nsIDOMCSSRule parentRule; */
    pub get_parentRule: unsafe extern "C" fn (this: *const nsIDOMCSSStyleDeclaration, aParentRule: *mut *const nsIDOMCSSRule) -> nsresult,

}


impl nsIDOMCSSStyleDeclaration {
    /* attribute DOMString cssText; */
    #[inline]
    pub unsafe fn get_cssText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_cssText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_cssText(&self, aCssText: &[u16]) -> Result<(), nsresult> {
        let aCssText = nsString::from(aCssText);
        match ((*self.vtable).set_cssText)(self as *const _, &*aCssText) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* DOMString getPropertyValue (in DOMString propertyName); */
    #[inline]
    pub unsafe fn getPropertyValue(&self, propertyName: &[u16]) -> Result<nsString, nsresult> {
        let propertyName = nsString::from(propertyName);
        let mut _retval = nsString::new();
        match ((*self.vtable).getPropertyValue)(self as *const _, &*propertyName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMCSSValue getPropertyCSSValue (in DOMString propertyName); */
    #[inline]
    pub unsafe fn getPropertyCSSValue(&self, propertyName: &[u16]) -> Result<Option<RefPtr<nsIDOMCSSValue>>, nsresult> {
        let propertyName = nsString::from(propertyName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getPropertyCSSValue)(self as *const _, &*propertyName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* DOMString removeProperty (in DOMString propertyName) raises (DOMException); */
    #[inline]
    pub unsafe fn removeProperty(&self, propertyName: &[u16]) -> Result<nsString, nsresult> {
        let propertyName = nsString::from(propertyName);
        let mut _retval = nsString::new();
        match ((*self.vtable).removeProperty)(self as *const _, &*propertyName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* DOMString getPropertyPriority (in DOMString propertyName); */
    #[inline]
    pub unsafe fn getPropertyPriority(&self, propertyName: &[u16]) -> Result<nsString, nsresult> {
        let propertyName = nsString::from(propertyName);
        let mut _retval = nsString::new();
        match ((*self.vtable).getPropertyPriority)(self as *const _, &*propertyName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setProperty (in DOMString propertyName, in DOMString value, [optional] in DOMString priority) raises (DOMException); */
    #[inline]
    pub unsafe fn setProperty(&self, propertyName: &[u16], value: &[u16], priority: &[u16]) -> Result<(), nsresult> {
        let propertyName = nsString::from(propertyName);
        let value = nsString::from(value);
        let priority = nsString::from(priority);
        match ((*self.vtable).setProperty)(self as *const _, &*propertyName, &*value, &*priority) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* DOMString item (in unsigned long index); */
    #[inline]
    pub unsafe fn item(&self, index: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).item)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMCSSRule parentRule; */
    #[inline]
    pub unsafe fn get_parentRule(&self, ) -> Result<Option<RefPtr<nsIDOMCSSRule>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parentRule)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNavigator.idl
//


#[repr(C)]
pub struct nsIDOMNavigator {
    vtable: *const nsIDOMNavigatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMNavigator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf1101fbb, 0xd119, 0x4cb8,
            [0x84, 0x5b, 0x6b, 0xba, 0xe8, 0xa1, 0x51, 0xc7])
    }
}

unsafe impl RefCounted for nsIDOMNavigator {
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
pub trait nsIDOMNavigatorCoerce {
    fn coerce_from(v: &nsIDOMNavigator) -> &Self;
}

impl nsIDOMNavigatorCoerce for nsIDOMNavigator {
    #[inline]
    fn coerce_from(v: &nsIDOMNavigator) -> &Self {
        v
    }
}

impl nsIDOMNavigator {
    #[inline]
    pub fn coerce<T: nsIDOMNavigatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMNavigator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMNavigatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMNavigator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMNavigatorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString appCodeName; */
    pub get_appCodeName: unsafe extern "C" fn (this: *const nsIDOMNavigator, aAppCodeName: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString language; */
    pub get_language: unsafe extern "C" fn (this: *const nsIDOMNavigator, aLanguage: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString vendor; */
    pub get_vendor: unsafe extern "C" fn (this: *const nsIDOMNavigator, aVendor: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString vendorSub; */
    pub get_vendorSub: unsafe extern "C" fn (this: *const nsIDOMNavigator, aVendorSub: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString product; */
    pub get_product: unsafe extern "C" fn (this: *const nsIDOMNavigator, aProduct: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString productSub; */
    pub get_productSub: unsafe extern "C" fn (this: *const nsIDOMNavigator, aProductSub: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString doNotTrack; */
    pub get_doNotTrack: unsafe extern "C" fn (this: *const nsIDOMNavigator, aDoNotTrack: *mut nsAString) -> nsresult,

}


impl nsIDOMNavigator {
    /* readonly attribute DOMString appCodeName; */
    #[inline]
    pub unsafe fn get_appCodeName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_appCodeName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString language; */
    #[inline]
    pub unsafe fn get_language(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_language)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString vendor; */
    #[inline]
    pub unsafe fn get_vendor(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_vendor)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString vendorSub; */
    #[inline]
    pub unsafe fn get_vendorSub(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_vendorSub)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString product; */
    #[inline]
    pub unsafe fn get_product(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_product)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString productSub; */
    #[inline]
    pub unsafe fn get_productSub(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_productSub)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString doNotTrack; */
    #[inline]
    pub unsafe fn get_doNotTrack(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_doNotTrack)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



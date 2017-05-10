//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAboutNewTabService.idl
//


#[repr(C)]
pub struct nsIAboutNewTabService {
    vtable: *const nsIAboutNewTabServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAboutNewTabService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdfcd2adc, 0x7867, 0x4d3a,
            [0xba, 0x70, 0x17, 0x50, 0x1f, 0x20, 0x81, 0x42])
    }
}

unsafe impl RefCounted for nsIAboutNewTabService {
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
pub trait nsIAboutNewTabServiceCoerce {
    fn coerce_from(v: &nsIAboutNewTabService) -> &Self;
}

impl nsIAboutNewTabServiceCoerce for nsIAboutNewTabService {
    #[inline]
    fn coerce_from(v: &nsIAboutNewTabService) -> &Self {
        v
    }
}

impl nsIAboutNewTabService {
    #[inline]
    pub fn coerce<T: nsIAboutNewTabServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAboutNewTabService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAboutNewTabServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAboutNewTabService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAboutNewTabServiceVTable {
    pub __base: nsISupportsVTable,

    /* attribute ACString newTabURL; */
    pub get_newTabURL: unsafe extern "C" fn (this: *const nsIAboutNewTabService, aNewTabURL: *mut nsACString) -> nsresult,
    pub set_newTabURL: unsafe extern "C" fn (this: *const nsIAboutNewTabService, aNewTabURL: *const nsACString) -> nsresult,

    /* attribute ACString defaultURL; */
    pub get_defaultURL: unsafe extern "C" fn (this: *const nsIAboutNewTabService, aDefaultURL: *mut nsACString) -> nsresult,
    pub set_defaultURL: unsafe extern "C" fn (this: *const nsIAboutNewTabService, aDefaultURL: *const nsACString) -> nsresult,

    /* readonly attribute bool overridden; */
    pub get_overridden: unsafe extern "C" fn (this: *const nsIAboutNewTabService, aOverridden: *mut bool) -> nsresult,

    /* readonly attribute bool activityStreamEnabled; */
    pub get_activityStreamEnabled: unsafe extern "C" fn (this: *const nsIAboutNewTabService, aActivityStreamEnabled: *mut bool) -> nsresult,

    /* readonly attribute ACString activityStreamURL; */
    pub get_activityStreamURL: unsafe extern "C" fn (this: *const nsIAboutNewTabService, aActivityStreamURL: *mut nsACString) -> nsresult,

    /* void resetNewTabURL (); */
    pub resetNewTabURL: unsafe extern "C" fn (this: *const nsIAboutNewTabService) -> nsresult,

}


impl nsIAboutNewTabService {
    /* attribute ACString newTabURL; */
    #[inline]
    pub unsafe fn get_newTabURL(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_newTabURL)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_newTabURL(&self, aNewTabURL: &[u8]) -> Result<(), nsresult> {
        let aNewTabURL = nsCString::from(aNewTabURL);
        match ((*self.vtable).set_newTabURL)(self as *const _, &*aNewTabURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute ACString defaultURL; */
    #[inline]
    pub unsafe fn get_defaultURL(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_defaultURL)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_defaultURL(&self, aDefaultURL: &[u8]) -> Result<(), nsresult> {
        let aDefaultURL = nsCString::from(aDefaultURL);
        match ((*self.vtable).set_defaultURL)(self as *const _, &*aDefaultURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute bool overridden; */
    #[inline]
    pub unsafe fn get_overridden(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_overridden)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool activityStreamEnabled; */
    #[inline]
    pub unsafe fn get_activityStreamEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_activityStreamEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString activityStreamURL; */
    #[inline]
    pub unsafe fn get_activityStreamURL(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_activityStreamURL)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void resetNewTabURL (); */
    #[inline]
    pub unsafe fn resetNewTabURL(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resetNewTabURL)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



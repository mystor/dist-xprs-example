//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFindService.idl
//


#[repr(C)]
pub struct nsIFindService {
    vtable: *const nsIFindServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFindService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5060b801, 0x340e, 0x11d5,
            [0xbe, 0x5b, 0xb3, 0xe0, 0x63, 0xec, 0x6a, 0x3c])
    }
}

unsafe impl RefCounted for nsIFindService {
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
pub trait nsIFindServiceCoerce {
    fn coerce_from(v: &nsIFindService) -> &Self;
}

impl nsIFindServiceCoerce for nsIFindService {
    #[inline]
    fn coerce_from(v: &nsIFindService) -> &Self {
        v
    }
}

impl nsIFindService {
    #[inline]
    pub fn coerce<T: nsIFindServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFindService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFindServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFindService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFindServiceVTable {
    pub __base: nsISupportsVTable,

    /* attribute AString searchString; */
    pub get_searchString: unsafe extern "C" fn (this: *const nsIFindService, aSearchString: *mut nsAString) -> nsresult,
    pub set_searchString: unsafe extern "C" fn (this: *const nsIFindService, aSearchString: *const nsAString) -> nsresult,

    /* attribute AString replaceString; */
    pub get_replaceString: unsafe extern "C" fn (this: *const nsIFindService, aReplaceString: *mut nsAString) -> nsresult,
    pub set_replaceString: unsafe extern "C" fn (this: *const nsIFindService, aReplaceString: *const nsAString) -> nsresult,

    /* attribute boolean findBackwards; */
    pub get_findBackwards: unsafe extern "C" fn (this: *const nsIFindService, aFindBackwards: *mut bool) -> nsresult,
    pub set_findBackwards: unsafe extern "C" fn (this: *const nsIFindService, aFindBackwards: bool) -> nsresult,

    /* attribute boolean wrapFind; */
    pub get_wrapFind: unsafe extern "C" fn (this: *const nsIFindService, aWrapFind: *mut bool) -> nsresult,
    pub set_wrapFind: unsafe extern "C" fn (this: *const nsIFindService, aWrapFind: bool) -> nsresult,

    /* attribute boolean entireWord; */
    pub get_entireWord: unsafe extern "C" fn (this: *const nsIFindService, aEntireWord: *mut bool) -> nsresult,
    pub set_entireWord: unsafe extern "C" fn (this: *const nsIFindService, aEntireWord: bool) -> nsresult,

    /* attribute boolean matchCase; */
    pub get_matchCase: unsafe extern "C" fn (this: *const nsIFindService, aMatchCase: *mut bool) -> nsresult,
    pub set_matchCase: unsafe extern "C" fn (this: *const nsIFindService, aMatchCase: bool) -> nsresult,

}


impl nsIFindService {
    /* attribute AString searchString; */
    #[inline]
    pub unsafe fn get_searchString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_searchString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_searchString(&self, aSearchString: &[u16]) -> Result<(), nsresult> {
        let aSearchString = nsString::from(aSearchString);
        match ((*self.vtable).set_searchString)(self as *const _, &*aSearchString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString replaceString; */
    #[inline]
    pub unsafe fn get_replaceString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_replaceString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_replaceString(&self, aReplaceString: &[u16]) -> Result<(), nsresult> {
        let aReplaceString = nsString::from(aReplaceString);
        match ((*self.vtable).set_replaceString)(self as *const _, &*aReplaceString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean findBackwards; */
    #[inline]
    pub unsafe fn get_findBackwards(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_findBackwards)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_findBackwards(&self, aFindBackwards: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_findBackwards)(self as *const _, aFindBackwards) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean wrapFind; */
    #[inline]
    pub unsafe fn get_wrapFind(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_wrapFind)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_wrapFind(&self, aWrapFind: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_wrapFind)(self as *const _, aWrapFind) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean entireWord; */
    #[inline]
    pub unsafe fn get_entireWord(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_entireWord)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_entireWord(&self, aEntireWord: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_entireWord)(self as *const _, aEntireWord) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean matchCase; */
    #[inline]
    pub unsafe fn get_matchCase(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_matchCase)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_matchCase(&self, aMatchCase: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_matchCase)(self as *const _, aMatchCase) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



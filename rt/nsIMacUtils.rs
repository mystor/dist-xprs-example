//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMacUtils.idl
//


#[repr(C)]
pub struct nsIMacUtils {
    vtable: *const nsIMacUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMacUtils {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5e9072d7, 0xff95, 0x455e,
            [0x94, 0x66, 0x8a, 0xf9, 0x84, 0x1a, 0x72, 0xec])
    }
}

unsafe impl RefCounted for nsIMacUtils {
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
pub trait nsIMacUtilsCoerce {
    fn coerce_from(v: &nsIMacUtils) -> &Self;
}

impl nsIMacUtilsCoerce for nsIMacUtils {
    #[inline]
    fn coerce_from(v: &nsIMacUtils) -> &Self {
        v
    }
}

impl nsIMacUtils {
    #[inline]
    pub fn coerce<T: nsIMacUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMacUtils {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMacUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMacUtils) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMacUtilsVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isUniversalBinary; */
    pub get_isUniversalBinary: unsafe extern "C" fn (this: *const nsIMacUtils, aIsUniversalBinary: *mut bool) -> nsresult,

    /* readonly attribute AString architecturesInBinary; */
    pub get_architecturesInBinary: unsafe extern "C" fn (this: *const nsIMacUtils, aArchitecturesInBinary: *mut nsAString) -> nsresult,

    /* readonly attribute boolean isTranslated; */
    pub get_isTranslated: unsafe extern "C" fn (this: *const nsIMacUtils, aIsTranslated: *mut bool) -> nsresult,

}


impl nsIMacUtils {
    /* readonly attribute boolean isUniversalBinary; */
    #[inline]
    pub unsafe fn get_isUniversalBinary(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isUniversalBinary)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString architecturesInBinary; */
    #[inline]
    pub unsafe fn get_architecturesInBinary(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_architecturesInBinary)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isTranslated; */
    #[inline]
    pub unsafe fn get_isTranslated(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isTranslated)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



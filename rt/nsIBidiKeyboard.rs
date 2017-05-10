//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBidiKeyboard.idl
//


#[repr(C)]
pub struct nsIBidiKeyboard {
    vtable: *const nsIBidiKeyboardVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBidiKeyboard {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x288dae24, 0x76e2, 0x43a3,
            [0xbe, 0xfe, 0x9d, 0x9f, 0xab, 0xe8, 0x01, 0x4e])
    }
}

unsafe impl RefCounted for nsIBidiKeyboard {
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
pub trait nsIBidiKeyboardCoerce {
    fn coerce_from(v: &nsIBidiKeyboard) -> &Self;
}

impl nsIBidiKeyboardCoerce for nsIBidiKeyboard {
    #[inline]
    fn coerce_from(v: &nsIBidiKeyboard) -> &Self {
        v
    }
}

impl nsIBidiKeyboard {
    #[inline]
    pub fn coerce<T: nsIBidiKeyboardCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBidiKeyboard {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBidiKeyboardCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBidiKeyboard) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBidiKeyboardVTable {
    pub __base: nsISupportsVTable,

    /* void reset (); */
    pub reset: unsafe extern "C" fn (this: *const nsIBidiKeyboard) -> nsresult,

    /* boolean isLangRTL (); */
    pub isLangRTL: unsafe extern "C" fn (this: *const nsIBidiKeyboard, _retval: *mut bool) -> nsresult,

    /* readonly attribute boolean haveBidiKeyboards; */
    pub get_haveBidiKeyboards: unsafe extern "C" fn (this: *const nsIBidiKeyboard, aHaveBidiKeyboards: *mut bool) -> nsresult,

}


impl nsIBidiKeyboard {
    /* void reset (); */
    #[inline]
    pub unsafe fn reset(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reset)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isLangRTL (); */
    #[inline]
    pub unsafe fn isLangRTL(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isLangRTL)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean haveBidiKeyboards; */
    #[inline]
    pub unsafe fn get_haveBidiKeyboards(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_haveBidiKeyboards)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



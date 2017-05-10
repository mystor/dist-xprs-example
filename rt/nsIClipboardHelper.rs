//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClipboardHelper.idl
//


#[repr(C)]
pub struct nsIClipboardHelper {
    vtable: *const nsIClipboardHelperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIClipboardHelper {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x438307fd, 0x0c68, 0x4d79,
            [0x92, 0x2a, 0xf6, 0xcc, 0x95, 0x50, 0xcd, 0x02])
    }
}

unsafe impl RefCounted for nsIClipboardHelper {
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
pub trait nsIClipboardHelperCoerce {
    fn coerce_from(v: &nsIClipboardHelper) -> &Self;
}

impl nsIClipboardHelperCoerce for nsIClipboardHelper {
    #[inline]
    fn coerce_from(v: &nsIClipboardHelper) -> &Self {
        v
    }
}

impl nsIClipboardHelper {
    #[inline]
    pub fn coerce<T: nsIClipboardHelperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIClipboardHelper {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIClipboardHelperCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClipboardHelper) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIClipboardHelperVTable {
    pub __base: nsISupportsVTable,

    /* void copyStringToClipboard (in AString aString, in long aClipboardID); */
    pub copyStringToClipboard: unsafe extern "C" fn (this: *const nsIClipboardHelper, aString: *const nsAString, aClipboardID: libc::int32_t) -> nsresult,

    /* void copyString (in AString aString); */
    pub copyString: unsafe extern "C" fn (this: *const nsIClipboardHelper, aString: *const nsAString) -> nsresult,

}


impl nsIClipboardHelper {
    /* void copyStringToClipboard (in AString aString, in long aClipboardID); */
    #[inline]
    pub unsafe fn copyStringToClipboard(&self, aString: &[u16], aClipboardID: libc::int32_t) -> Result<(), nsresult> {
        let aString = nsString::from(aString);
        match ((*self.vtable).copyStringToClipboard)(self as *const _, &*aString, aClipboardID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void copyString (in AString aString); */
    #[inline]
    pub unsafe fn copyString(&self, aString: &[u16]) -> Result<(), nsresult> {
        let aString = nsString::from(aString);
        match ((*self.vtable).copyString)(self as *const _, &*aString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



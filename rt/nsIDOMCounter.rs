//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCounter.idl
//


#[repr(C)]
pub struct nsIDOMCounter {
    vtable: *const nsIDOMCounterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCounter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x31adb439, 0x0055, 0x402d,
            [0x9b, 0x1d, 0xd5, 0xca, 0x94, 0xf3, 0xf5, 0x5b])
    }
}

unsafe impl RefCounted for nsIDOMCounter {
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
pub trait nsIDOMCounterCoerce {
    fn coerce_from(v: &nsIDOMCounter) -> &Self;
}

impl nsIDOMCounterCoerce for nsIDOMCounter {
    #[inline]
    fn coerce_from(v: &nsIDOMCounter) -> &Self {
        v
    }
}

impl nsIDOMCounter {
    #[inline]
    pub fn coerce<T: nsIDOMCounterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCounter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCounterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCounter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCounterVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString identifier; */
    pub get_identifier: unsafe extern "C" fn (this: *const nsIDOMCounter, aIdentifier: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString listStyle; */
    pub get_listStyle: unsafe extern "C" fn (this: *const nsIDOMCounter, aListStyle: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString separator; */
    pub get_separator: unsafe extern "C" fn (this: *const nsIDOMCounter, aSeparator: *mut nsAString) -> nsresult,

}


impl nsIDOMCounter {
    /* readonly attribute DOMString identifier; */
    #[inline]
    pub unsafe fn get_identifier(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_identifier)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString listStyle; */
    #[inline]
    pub unsafe fn get_listStyle(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_listStyle)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString separator; */
    #[inline]
    pub unsafe fn get_separator(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_separator)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



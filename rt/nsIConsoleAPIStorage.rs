//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIConsoleAPIStorage.idl
//


#[repr(C)]
pub struct nsIConsoleAPIStorage {
    vtable: *const nsIConsoleAPIStorageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIConsoleAPIStorage {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9e32a7b6, 0xc4d1, 0x4d9a,
            [0x87, 0xb9, 0x1e, 0xf6, 0xb7, 0x5c, 0x27, 0xa9])
    }
}

unsafe impl RefCounted for nsIConsoleAPIStorage {
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
pub trait nsIConsoleAPIStorageCoerce {
    fn coerce_from(v: &nsIConsoleAPIStorage) -> &Self;
}

impl nsIConsoleAPIStorageCoerce for nsIConsoleAPIStorage {
    #[inline]
    fn coerce_from(v: &nsIConsoleAPIStorage) -> &Self {
        v
    }
}

impl nsIConsoleAPIStorage {
    #[inline]
    pub fn coerce<T: nsIConsoleAPIStorageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIConsoleAPIStorage {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIConsoleAPIStorageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIConsoleAPIStorage) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIConsoleAPIStorageVTable {
    pub __base: nsISupportsVTable,

    /* jsval getEvents ([optional] in DOMString aId); */
    /// Unable to call function as its signature contains a non-rust type
    pub getEvents: *const ::libc::c_void,

    /* void recordEvent (in DOMString aId, in DOMString aOuterId, in jsval aEvent); */
    /// Unable to call function as its signature contains a non-rust type
    pub recordEvent: *const ::libc::c_void,

    /* void clearEvents ([optional] in DOMString aId); */
    pub clearEvents: unsafe extern "C" fn (this: *const nsIConsoleAPIStorage, aId: *const nsAString) -> nsresult,

}


impl nsIConsoleAPIStorage {
    /* jsval getEvents ([optional] in DOMString aId); */


    /* void recordEvent (in DOMString aId, in DOMString aOuterId, in jsval aEvent); */


    /* void clearEvents ([optional] in DOMString aId); */
    #[inline]
    pub unsafe fn clearEvents(&self, aId: &[u16]) -> Result<(), nsresult> {
        let aId = nsString::from(aId);
        match ((*self.vtable).clearEvents)(self as *const _, &*aId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIJARProtocolHandler.idl
//


#[repr(C)]
pub struct nsIJARProtocolHandler {
    vtable: *const nsIJARProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIJARProtocolHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x92c3b42c, 0x98c4, 0x11d3,
            [0x8c, 0xd9, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3])
    }
}

unsafe impl RefCounted for nsIJARProtocolHandler {
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
pub trait nsIJARProtocolHandlerCoerce {
    fn coerce_from(v: &nsIJARProtocolHandler) -> &Self;
}

impl nsIJARProtocolHandlerCoerce for nsIJARProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsIJARProtocolHandler) -> &Self {
        v
    }
}

impl nsIJARProtocolHandler {
    #[inline]
    pub fn coerce<T: nsIJARProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIJARProtocolHandler {
    type Target = nsIProtocolHandler;
    #[inline]
    fn deref(&self) -> &nsIProtocolHandler {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIProtocolHandlerCoerce> nsIJARProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJARProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIJARProtocolHandlerVTable {
    pub __base: nsIProtocolHandlerVTable,

    /* readonly attribute nsIZipReaderCache JARCache; */
    pub get_JARCache: unsafe extern "C" fn (this: *const nsIJARProtocolHandler, aJARCache: *mut *const nsIZipReaderCache) -> nsresult,

}


impl nsIJARProtocolHandler {
    /* readonly attribute nsIZipReaderCache JARCache; */
    #[inline]
    pub unsafe fn get_JARCache(&self, ) -> Result<Option<RefPtr<nsIZipReaderCache>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_JARCache)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocumentLoader.idl
//


#[repr(C)]
pub struct nsIDocumentLoader {
    vtable: *const nsIDocumentLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDocumentLoader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbbe961ee, 0x59e9, 0x42bb,
            [0xbe, 0x50, 0x03, 0x31, 0x97, 0x9b, 0xb7, 0x9f])
    }
}

unsafe impl RefCounted for nsIDocumentLoader {
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
pub trait nsIDocumentLoaderCoerce {
    fn coerce_from(v: &nsIDocumentLoader) -> &Self;
}

impl nsIDocumentLoaderCoerce for nsIDocumentLoader {
    #[inline]
    fn coerce_from(v: &nsIDocumentLoader) -> &Self {
        v
    }
}

impl nsIDocumentLoader {
    #[inline]
    pub fn coerce<T: nsIDocumentLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDocumentLoader {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDocumentLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocumentLoader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDocumentLoaderVTable {
    pub __base: nsISupportsVTable,

    /* void stop (); */
    pub stop: unsafe extern "C" fn (this: *const nsIDocumentLoader) -> nsresult,

    /* readonly attribute nsISupports container; */
    pub get_container: unsafe extern "C" fn (this: *const nsIDocumentLoader, aContainer: *mut *const nsISupports) -> nsresult,

    /* readonly attribute nsILoadGroup loadGroup; */
    pub get_loadGroup: unsafe extern "C" fn (this: *const nsIDocumentLoader, aLoadGroup: *mut *const nsILoadGroup) -> nsresult,

    /* readonly attribute nsIChannel documentChannel; */
    pub get_documentChannel: unsafe extern "C" fn (this: *const nsIDocumentLoader, aDocumentChannel: *mut *const nsIChannel) -> nsresult,

}


impl nsIDocumentLoader {
    /* void stop (); */
    #[inline]
    pub unsafe fn stop(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stop)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsISupports container; */
    #[inline]
    pub unsafe fn get_container(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_container)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsILoadGroup loadGroup; */
    #[inline]
    pub unsafe fn get_loadGroup(&self, ) -> Result<Option<RefPtr<nsILoadGroup>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_loadGroup)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIChannel documentChannel; */
    #[inline]
    pub unsafe fn get_documentChannel(&self, ) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_documentChannel)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



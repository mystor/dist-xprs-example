//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedGenerator.idl
//


#[repr(C)]
pub struct nsIFeedGenerator {
    vtable: *const nsIFeedGeneratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFeedGenerator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0fecd56b, 0xbd92, 0x481b,
            [0xa4, 0x86, 0xb8, 0xd4, 0x89, 0xcd, 0xd3, 0x85])
    }
}

unsafe impl RefCounted for nsIFeedGenerator {
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
pub trait nsIFeedGeneratorCoerce {
    fn coerce_from(v: &nsIFeedGenerator) -> &Self;
}

impl nsIFeedGeneratorCoerce for nsIFeedGenerator {
    #[inline]
    fn coerce_from(v: &nsIFeedGenerator) -> &Self {
        v
    }
}

impl nsIFeedGenerator {
    #[inline]
    pub fn coerce<T: nsIFeedGeneratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFeedGenerator {
    type Target = nsIFeedElementBase;
    #[inline]
    fn deref(&self) -> &nsIFeedElementBase {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIFeedElementBaseCoerce> nsIFeedGeneratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFeedGenerator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFeedGeneratorVTable {
    pub __base: nsIFeedElementBaseVTable,

    /* attribute AString agent; */
    pub get_agent: unsafe extern "C" fn (this: *const nsIFeedGenerator, aAgent: *mut nsAString) -> nsresult,
    pub set_agent: unsafe extern "C" fn (this: *const nsIFeedGenerator, aAgent: *const nsAString) -> nsresult,

    /* attribute AString version; */
    pub get_version: unsafe extern "C" fn (this: *const nsIFeedGenerator, aVersion: *mut nsAString) -> nsresult,
    pub set_version: unsafe extern "C" fn (this: *const nsIFeedGenerator, aVersion: *const nsAString) -> nsresult,

    /* attribute nsIURI uri; */
    pub get_uri: unsafe extern "C" fn (this: *const nsIFeedGenerator, aUri: *mut *const nsIURI) -> nsresult,
    pub set_uri: unsafe extern "C" fn (this: *const nsIFeedGenerator, aUri: *const nsIURI) -> nsresult,

}


impl nsIFeedGenerator {
    /* attribute AString agent; */
    #[inline]
    pub unsafe fn get_agent(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_agent)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_agent(&self, aAgent: &[u16]) -> Result<(), nsresult> {
        let aAgent = nsString::from(aAgent);
        match ((*self.vtable).set_agent)(self as *const _, &*aAgent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString version; */
    #[inline]
    pub unsafe fn get_version(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_version)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_version(&self, aVersion: &[u16]) -> Result<(), nsresult> {
        let aVersion = nsString::from(aVersion);
        match ((*self.vtable).set_version)(self as *const _, &*aVersion) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIURI uri; */
    #[inline]
    pub unsafe fn get_uri(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_uri)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_uri(&self, aUri: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_uri)(self as *const _, aUri.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



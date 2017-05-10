//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedElementBase.idl
//


#[repr(C)]
pub struct nsIFeedElementBase {
    vtable: *const nsIFeedElementBaseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFeedElementBase {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5215291e, 0xfa0a, 0x40c2,
            [0x8c, 0xe7, 0xe8, 0x6c, 0xd1, 0xa1, 0xd3, 0xfa])
    }
}

unsafe impl RefCounted for nsIFeedElementBase {
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
pub trait nsIFeedElementBaseCoerce {
    fn coerce_from(v: &nsIFeedElementBase) -> &Self;
}

impl nsIFeedElementBaseCoerce for nsIFeedElementBase {
    #[inline]
    fn coerce_from(v: &nsIFeedElementBase) -> &Self {
        v
    }
}

impl nsIFeedElementBase {
    #[inline]
    pub fn coerce<T: nsIFeedElementBaseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFeedElementBase {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFeedElementBaseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFeedElementBase) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFeedElementBaseVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsISAXAttributes attributes; */
    pub get_attributes: unsafe extern "C" fn (this: *const nsIFeedElementBase, aAttributes: *mut *const nsISAXAttributes) -> nsresult,
    pub set_attributes: unsafe extern "C" fn (this: *const nsIFeedElementBase, aAttributes: *const nsISAXAttributes) -> nsresult,

    /* attribute nsIURI baseURI; */
    pub get_baseURI: unsafe extern "C" fn (this: *const nsIFeedElementBase, aBaseURI: *mut *const nsIURI) -> nsresult,
    pub set_baseURI: unsafe extern "C" fn (this: *const nsIFeedElementBase, aBaseURI: *const nsIURI) -> nsresult,

}


impl nsIFeedElementBase {
    /* attribute nsISAXAttributes attributes; */
    #[inline]
    pub unsafe fn get_attributes(&self, ) -> Result<Option<RefPtr<nsISAXAttributes>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_attributes)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_attributes(&self, aAttributes: Option<&nsISAXAttributes>) -> Result<(), nsresult> {

        match ((*self.vtable).set_attributes)(self as *const _, aAttributes.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIURI baseURI; */
    #[inline]
    pub unsafe fn get_baseURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_baseURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_baseURI(&self, aBaseURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_baseURI)(self as *const _, aBaseURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



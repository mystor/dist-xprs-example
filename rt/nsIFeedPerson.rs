//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedPerson.idl
//


#[repr(C)]
pub struct nsIFeedPerson {
    vtable: *const nsIFeedPersonVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFeedPerson {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x29cbd45f, 0xf2d3, 0x4b28,
            [0xb5, 0x57, 0x3a, 0xb7, 0xa6, 0x1e, 0xcd, 0xe4])
    }
}

unsafe impl RefCounted for nsIFeedPerson {
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
pub trait nsIFeedPersonCoerce {
    fn coerce_from(v: &nsIFeedPerson) -> &Self;
}

impl nsIFeedPersonCoerce for nsIFeedPerson {
    #[inline]
    fn coerce_from(v: &nsIFeedPerson) -> &Self {
        v
    }
}

impl nsIFeedPerson {
    #[inline]
    pub fn coerce<T: nsIFeedPersonCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFeedPerson {
    type Target = nsIFeedElementBase;
    #[inline]
    fn deref(&self) -> &nsIFeedElementBase {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIFeedElementBaseCoerce> nsIFeedPersonCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFeedPerson) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFeedPersonVTable {
    pub __base: nsIFeedElementBaseVTable,

    /* attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIFeedPerson, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIFeedPerson, aName: *const nsAString) -> nsresult,

    /* attribute AString email; */
    pub get_email: unsafe extern "C" fn (this: *const nsIFeedPerson, aEmail: *mut nsAString) -> nsresult,
    pub set_email: unsafe extern "C" fn (this: *const nsIFeedPerson, aEmail: *const nsAString) -> nsresult,

    /* attribute nsIURI uri; */
    pub get_uri: unsafe extern "C" fn (this: *const nsIFeedPerson, aUri: *mut *const nsIURI) -> nsresult,
    pub set_uri: unsafe extern "C" fn (this: *const nsIFeedPerson, aUri: *const nsIURI) -> nsresult,

}


impl nsIFeedPerson {
    /* attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_name(&self, aName: &[u16]) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).set_name)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString email; */
    #[inline]
    pub unsafe fn get_email(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_email)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_email(&self, aEmail: &[u16]) -> Result<(), nsresult> {
        let aEmail = nsString::from(aEmail);
        match ((*self.vtable).set_email)(self as *const _, &*aEmail) {
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



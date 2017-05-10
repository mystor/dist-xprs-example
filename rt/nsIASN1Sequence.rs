//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIASN1Sequence.idl
//


#[repr(C)]
pub struct nsIASN1Sequence {
    vtable: *const nsIASN1SequenceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIASN1Sequence {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb6b957e6, 0x1dd1, 0x11b2,
            [0x89, 0xd7, 0xe3, 0x06, 0x24, 0xf5, 0x0b, 0x00])
    }
}

unsafe impl RefCounted for nsIASN1Sequence {
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
pub trait nsIASN1SequenceCoerce {
    fn coerce_from(v: &nsIASN1Sequence) -> &Self;
}

impl nsIASN1SequenceCoerce for nsIASN1Sequence {
    #[inline]
    fn coerce_from(v: &nsIASN1Sequence) -> &Self {
        v
    }
}

impl nsIASN1Sequence {
    #[inline]
    pub fn coerce<T: nsIASN1SequenceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIASN1Sequence {
    type Target = nsIASN1Object;
    #[inline]
    fn deref(&self) -> &nsIASN1Object {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIASN1ObjectCoerce> nsIASN1SequenceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIASN1Sequence) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIASN1SequenceVTable {
    pub __base: nsIASN1ObjectVTable,

    /* attribute nsIMutableArray ASN1Objects; */
    pub get_ASN1Objects: unsafe extern "C" fn (this: *const nsIASN1Sequence, aASN1Objects: *mut *const nsIMutableArray) -> nsresult,
    pub set_ASN1Objects: unsafe extern "C" fn (this: *const nsIASN1Sequence, aASN1Objects: *const nsIMutableArray) -> nsresult,

    /* attribute boolean isValidContainer; */
    pub get_isValidContainer: unsafe extern "C" fn (this: *const nsIASN1Sequence, aIsValidContainer: *mut bool) -> nsresult,
    pub set_isValidContainer: unsafe extern "C" fn (this: *const nsIASN1Sequence, aIsValidContainer: bool) -> nsresult,

    /* attribute boolean isExpanded; */
    pub get_isExpanded: unsafe extern "C" fn (this: *const nsIASN1Sequence, aIsExpanded: *mut bool) -> nsresult,
    pub set_isExpanded: unsafe extern "C" fn (this: *const nsIASN1Sequence, aIsExpanded: bool) -> nsresult,

}


impl nsIASN1Sequence {
    /* attribute nsIMutableArray ASN1Objects; */
    #[inline]
    pub unsafe fn get_ASN1Objects(&self, ) -> Result<Option<RefPtr<nsIMutableArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ASN1Objects)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_ASN1Objects(&self, aASN1Objects: Option<&nsIMutableArray>) -> Result<(), nsresult> {

        match ((*self.vtable).set_ASN1Objects)(self as *const _, aASN1Objects.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean isValidContainer; */
    #[inline]
    pub unsafe fn get_isValidContainer(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isValidContainer)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isValidContainer(&self, aIsValidContainer: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isValidContainer)(self as *const _, aIsValidContainer) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean isExpanded; */
    #[inline]
    pub unsafe fn get_isExpanded(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isExpanded)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isExpanded(&self, aIsExpanded: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isExpanded)(self as *const _, aIsExpanded) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



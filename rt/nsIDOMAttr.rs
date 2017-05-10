//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMAttr.idl
//


#[repr(C)]
pub struct nsIDOMAttr {
    vtable: *const nsIDOMAttrVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMAttr {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7db491e8, 0xa3a3, 0x4432,
            [0xad, 0x67, 0xe6, 0xc3, 0x3e, 0x24, 0xac, 0x6d])
    }
}

unsafe impl RefCounted for nsIDOMAttr {
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
pub trait nsIDOMAttrCoerce {
    fn coerce_from(v: &nsIDOMAttr) -> &Self;
}

impl nsIDOMAttrCoerce for nsIDOMAttr {
    #[inline]
    fn coerce_from(v: &nsIDOMAttr) -> &Self {
        v
    }
}

impl nsIDOMAttr {
    #[inline]
    pub fn coerce<T: nsIDOMAttrCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMAttr {
    type Target = nsIDOMNode;
    #[inline]
    fn deref(&self) -> &nsIDOMNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMNodeCoerce> nsIDOMAttrCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMAttr) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMAttrVTable {
    pub __base: nsIDOMNodeVTable,

    /* readonly attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMAttr, aName: *mut nsAString) -> nsresult,

    /* readonly attribute boolean specified; */
    pub get_specified: unsafe extern "C" fn (this: *const nsIDOMAttr, aSpecified: *mut bool) -> nsresult,

    /* attribute DOMString value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIDOMAttr, aValue: *mut nsAString) -> nsresult,
    pub set_value: unsafe extern "C" fn (this: *const nsIDOMAttr, aValue: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMElement ownerElement; */
    pub get_ownerElement: unsafe extern "C" fn (this: *const nsIDOMAttr, aOwnerElement: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute boolean isId; */
    pub get_isId: unsafe extern "C" fn (this: *const nsIDOMAttr, aIsId: *mut bool) -> nsresult,

}


impl nsIDOMAttr {
    /* readonly attribute DOMString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean specified; */
    #[inline]
    pub unsafe fn get_specified(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_specified)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute DOMString value; */
    #[inline]
    pub unsafe fn get_value(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_value)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_value(&self, aValue: &[u16]) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        match ((*self.vtable).set_value)(self as *const _, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMElement ownerElement; */
    #[inline]
    pub unsafe fn get_ownerElement(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ownerElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean isId; */
    #[inline]
    pub unsafe fn get_isId(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



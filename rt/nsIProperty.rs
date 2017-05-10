//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProperty.idl
//


#[repr(C)]
pub struct nsIProperty {
    vtable: *const nsIPropertyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProperty {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6dcf9030, 0xa49f, 0x11d5,
            [0x91, 0x0d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a])
    }
}

unsafe impl RefCounted for nsIProperty {
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
pub trait nsIPropertyCoerce {
    fn coerce_from(v: &nsIProperty) -> &Self;
}

impl nsIPropertyCoerce for nsIProperty {
    #[inline]
    fn coerce_from(v: &nsIProperty) -> &Self {
        v
    }
}

impl nsIProperty {
    #[inline]
    pub fn coerce<T: nsIPropertyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProperty {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPropertyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProperty) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPropertyVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIProperty, aName: *mut nsAString) -> nsresult,

    /* readonly attribute nsIVariant value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIProperty, aValue: *mut *const nsIVariant) -> nsresult,

}


impl nsIProperty {
    /* readonly attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIVariant value; */
    #[inline]
    pub unsafe fn get_value(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_value)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



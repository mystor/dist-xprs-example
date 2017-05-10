//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEnvironment.idl
//


#[repr(C)]
pub struct nsIEnvironment {
    vtable: *const nsIEnvironmentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEnvironment {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x101d5941, 0xd820, 0x4e85,
            [0xa2, 0x66, 0x9a, 0x34, 0x69, 0x94, 0x08, 0x07])
    }
}

unsafe impl RefCounted for nsIEnvironment {
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
pub trait nsIEnvironmentCoerce {
    fn coerce_from(v: &nsIEnvironment) -> &Self;
}

impl nsIEnvironmentCoerce for nsIEnvironment {
    #[inline]
    fn coerce_from(v: &nsIEnvironment) -> &Self {
        v
    }
}

impl nsIEnvironment {
    #[inline]
    pub fn coerce<T: nsIEnvironmentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEnvironment {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEnvironmentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEnvironment) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEnvironmentVTable {
    pub __base: nsISupportsVTable,

    /* void set (in AString aName, in AString aValue); */
    pub set: unsafe extern "C" fn (this: *const nsIEnvironment, aName: *const nsAString, aValue: *const nsAString) -> nsresult,

    /* AString get (in AString aName); */
    pub get: unsafe extern "C" fn (this: *const nsIEnvironment, aName: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* boolean exists (in AString aName); */
    pub exists: unsafe extern "C" fn (this: *const nsIEnvironment, aName: *const nsAString, _retval: *mut bool) -> nsresult,

}


impl nsIEnvironment {
    /* void set (in AString aName, in AString aValue); */
    #[inline]
    pub unsafe fn set(&self, aName: &[u16], aValue: &[u16]) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        let aValue = nsString::from(aValue);
        match ((*self.vtable).set)(self as *const _, &*aName, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString get (in AString aName); */
    #[inline]
    pub unsafe fn get(&self, aName: &[u16]) -> Result<nsString, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval = nsString::new();
        match ((*self.vtable).get)(self as *const _, &*aName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean exists (in AString aName); */
    #[inline]
    pub unsafe fn exists(&self, aName: &[u16]) -> Result<bool, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).exists)(self as *const _, &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



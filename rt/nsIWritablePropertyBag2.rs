//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWritablePropertyBag2.idl
//


#[repr(C)]
pub struct nsIWritablePropertyBag2 {
    vtable: *const nsIWritablePropertyBag2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWritablePropertyBag2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9cfd1587, 0x360e, 0x4957,
            [0xa5, 0x8f, 0x4c, 0x2b, 0x1c, 0x5e, 0x7e, 0xd9])
    }
}

unsafe impl RefCounted for nsIWritablePropertyBag2 {
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
pub trait nsIWritablePropertyBag2Coerce {
    fn coerce_from(v: &nsIWritablePropertyBag2) -> &Self;
}

impl nsIWritablePropertyBag2Coerce for nsIWritablePropertyBag2 {
    #[inline]
    fn coerce_from(v: &nsIWritablePropertyBag2) -> &Self {
        v
    }
}

impl nsIWritablePropertyBag2 {
    #[inline]
    pub fn coerce<T: nsIWritablePropertyBag2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWritablePropertyBag2 {
    type Target = nsIPropertyBag2;
    #[inline]
    fn deref(&self) -> &nsIPropertyBag2 {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIPropertyBag2Coerce> nsIWritablePropertyBag2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIWritablePropertyBag2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWritablePropertyBag2VTable {
    pub __base: nsIPropertyBag2VTable,

    /* void setPropertyAsInt32 (in AString prop, in int32_t value); */
    pub setPropertyAsInt32: unsafe extern "C" fn (this: *const nsIWritablePropertyBag2, prop: *const nsAString, value: int32_t) -> nsresult,

    /* void setPropertyAsUint32 (in AString prop, in uint32_t value); */
    pub setPropertyAsUint32: unsafe extern "C" fn (this: *const nsIWritablePropertyBag2, prop: *const nsAString, value: uint32_t) -> nsresult,

    /* void setPropertyAsInt64 (in AString prop, in int64_t value); */
    pub setPropertyAsInt64: unsafe extern "C" fn (this: *const nsIWritablePropertyBag2, prop: *const nsAString, value: int64_t) -> nsresult,

    /* void setPropertyAsUint64 (in AString prop, in uint64_t value); */
    pub setPropertyAsUint64: unsafe extern "C" fn (this: *const nsIWritablePropertyBag2, prop: *const nsAString, value: uint64_t) -> nsresult,

    /* void setPropertyAsDouble (in AString prop, in double value); */
    pub setPropertyAsDouble: unsafe extern "C" fn (this: *const nsIWritablePropertyBag2, prop: *const nsAString, value: libc::c_double) -> nsresult,

    /* void setPropertyAsAString (in AString prop, in AString value); */
    pub setPropertyAsAString: unsafe extern "C" fn (this: *const nsIWritablePropertyBag2, prop: *const nsAString, value: *const nsAString) -> nsresult,

    /* void setPropertyAsACString (in AString prop, in ACString value); */
    pub setPropertyAsACString: unsafe extern "C" fn (this: *const nsIWritablePropertyBag2, prop: *const nsAString, value: *const nsACString) -> nsresult,

    /* void setPropertyAsAUTF8String (in AString prop, in AUTF8String value); */
    pub setPropertyAsAUTF8String: unsafe extern "C" fn (this: *const nsIWritablePropertyBag2, prop: *const nsAString, value: *const nsACString) -> nsresult,

    /* void setPropertyAsBool (in AString prop, in boolean value); */
    pub setPropertyAsBool: unsafe extern "C" fn (this: *const nsIWritablePropertyBag2, prop: *const nsAString, value: bool) -> nsresult,

    /* void setPropertyAsInterface (in AString prop, in nsISupports value); */
    pub setPropertyAsInterface: unsafe extern "C" fn (this: *const nsIWritablePropertyBag2, prop: *const nsAString, value: *const nsISupports) -> nsresult,

}


impl nsIWritablePropertyBag2 {
    /* void setPropertyAsInt32 (in AString prop, in int32_t value); */
    #[inline]
    pub unsafe fn setPropertyAsInt32(&self, prop: &[u16], value: int32_t) -> Result<(), nsresult> {
        let prop = nsString::from(prop);
        match ((*self.vtable).setPropertyAsInt32)(self as *const _, &*prop, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPropertyAsUint32 (in AString prop, in uint32_t value); */
    #[inline]
    pub unsafe fn setPropertyAsUint32(&self, prop: &[u16], value: uint32_t) -> Result<(), nsresult> {
        let prop = nsString::from(prop);
        match ((*self.vtable).setPropertyAsUint32)(self as *const _, &*prop, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPropertyAsInt64 (in AString prop, in int64_t value); */
    #[inline]
    pub unsafe fn setPropertyAsInt64(&self, prop: &[u16], value: int64_t) -> Result<(), nsresult> {
        let prop = nsString::from(prop);
        match ((*self.vtable).setPropertyAsInt64)(self as *const _, &*prop, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPropertyAsUint64 (in AString prop, in uint64_t value); */
    #[inline]
    pub unsafe fn setPropertyAsUint64(&self, prop: &[u16], value: uint64_t) -> Result<(), nsresult> {
        let prop = nsString::from(prop);
        match ((*self.vtable).setPropertyAsUint64)(self as *const _, &*prop, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPropertyAsDouble (in AString prop, in double value); */
    #[inline]
    pub unsafe fn setPropertyAsDouble(&self, prop: &[u16], value: libc::c_double) -> Result<(), nsresult> {
        let prop = nsString::from(prop);
        match ((*self.vtable).setPropertyAsDouble)(self as *const _, &*prop, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPropertyAsAString (in AString prop, in AString value); */
    #[inline]
    pub unsafe fn setPropertyAsAString(&self, prop: &[u16], value: &[u16]) -> Result<(), nsresult> {
        let prop = nsString::from(prop);
        let value = nsString::from(value);
        match ((*self.vtable).setPropertyAsAString)(self as *const _, &*prop, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPropertyAsACString (in AString prop, in ACString value); */
    #[inline]
    pub unsafe fn setPropertyAsACString(&self, prop: &[u16], value: &[u8]) -> Result<(), nsresult> {
        let prop = nsString::from(prop);
        let value = nsCString::from(value);
        match ((*self.vtable).setPropertyAsACString)(self as *const _, &*prop, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPropertyAsAUTF8String (in AString prop, in AUTF8String value); */
    #[inline]
    pub unsafe fn setPropertyAsAUTF8String(&self, prop: &[u16], value: &[u8]) -> Result<(), nsresult> {
        let prop = nsString::from(prop);
        let value = nsCString::from(value);
        match ((*self.vtable).setPropertyAsAUTF8String)(self as *const _, &*prop, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPropertyAsBool (in AString prop, in boolean value); */
    #[inline]
    pub unsafe fn setPropertyAsBool(&self, prop: &[u16], value: bool) -> Result<(), nsresult> {
        let prop = nsString::from(prop);
        match ((*self.vtable).setPropertyAsBool)(self as *const _, &*prop, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPropertyAsInterface (in AString prop, in nsISupports value); */
    #[inline]
    pub unsafe fn setPropertyAsInterface(&self, prop: &[u16], value: Option<&nsISupports>) -> Result<(), nsresult> {
        let prop = nsString::from(prop);
        match ((*self.vtable).setPropertyAsInterface)(self as *const _, &*prop, value.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



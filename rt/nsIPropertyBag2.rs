//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPropertyBag2.idl
//


#[repr(C)]
pub struct nsIPropertyBag2 {
    vtable: *const nsIPropertyBag2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPropertyBag2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x625cfd1e, 0xda1e, 0x4417,
            [0x9e, 0xe9, 0xdb, 0xc8, 0xe0, 0xb3, 0xfd, 0x79])
    }
}

unsafe impl RefCounted for nsIPropertyBag2 {
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
pub trait nsIPropertyBag2Coerce {
    fn coerce_from(v: &nsIPropertyBag2) -> &Self;
}

impl nsIPropertyBag2Coerce for nsIPropertyBag2 {
    #[inline]
    fn coerce_from(v: &nsIPropertyBag2) -> &Self {
        v
    }
}

impl nsIPropertyBag2 {
    #[inline]
    pub fn coerce<T: nsIPropertyBag2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPropertyBag2 {
    type Target = nsIPropertyBag;
    #[inline]
    fn deref(&self) -> &nsIPropertyBag {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIPropertyBagCoerce> nsIPropertyBag2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIPropertyBag2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPropertyBag2VTable {
    pub __base: nsIPropertyBagVTable,

    /* int32_t getPropertyAsInt32 (in AString prop); */
    pub getPropertyAsInt32: unsafe extern "C" fn (this: *const nsIPropertyBag2, prop: *const nsAString, _retval: *mut int32_t) -> nsresult,

    /* uint32_t getPropertyAsUint32 (in AString prop); */
    pub getPropertyAsUint32: unsafe extern "C" fn (this: *const nsIPropertyBag2, prop: *const nsAString, _retval: *mut uint32_t) -> nsresult,

    /* int64_t getPropertyAsInt64 (in AString prop); */
    pub getPropertyAsInt64: unsafe extern "C" fn (this: *const nsIPropertyBag2, prop: *const nsAString, _retval: *mut int64_t) -> nsresult,

    /* uint64_t getPropertyAsUint64 (in AString prop); */
    pub getPropertyAsUint64: unsafe extern "C" fn (this: *const nsIPropertyBag2, prop: *const nsAString, _retval: *mut uint64_t) -> nsresult,

    /* double getPropertyAsDouble (in AString prop); */
    pub getPropertyAsDouble: unsafe extern "C" fn (this: *const nsIPropertyBag2, prop: *const nsAString, _retval: *mut libc::c_double) -> nsresult,

    /* AString getPropertyAsAString (in AString prop); */
    pub getPropertyAsAString: unsafe extern "C" fn (this: *const nsIPropertyBag2, prop: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* ACString getPropertyAsACString (in AString prop); */
    pub getPropertyAsACString: unsafe extern "C" fn (this: *const nsIPropertyBag2, prop: *const nsAString, _retval: *mut nsACString) -> nsresult,

    /* AUTF8String getPropertyAsAUTF8String (in AString prop); */
    pub getPropertyAsAUTF8String: unsafe extern "C" fn (this: *const nsIPropertyBag2, prop: *const nsAString, _retval: *mut nsACString) -> nsresult,

    /* boolean getPropertyAsBool (in AString prop); */
    pub getPropertyAsBool: unsafe extern "C" fn (this: *const nsIPropertyBag2, prop: *const nsAString, _retval: *mut bool) -> nsresult,

    /* void getPropertyAsInterface (in AString prop, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    pub getPropertyAsInterface: unsafe extern "C" fn (this: *const nsIPropertyBag2, prop: *const nsAString, iid: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

    /* nsIVariant get (in AString prop); */
    pub get: unsafe extern "C" fn (this: *const nsIPropertyBag2, prop: *const nsAString, _retval: *mut *const nsIVariant) -> nsresult,

    /* boolean hasKey (in AString prop); */
    pub hasKey: unsafe extern "C" fn (this: *const nsIPropertyBag2, prop: *const nsAString, _retval: *mut bool) -> nsresult,

}


impl nsIPropertyBag2 {
    /* int32_t getPropertyAsInt32 (in AString prop); */
    #[inline]
    pub unsafe fn getPropertyAsInt32(&self, prop: &[u16]) -> Result<int32_t, nsresult> {
        let prop = nsString::from(prop);
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getPropertyAsInt32)(self as *const _, &*prop, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint32_t getPropertyAsUint32 (in AString prop); */
    #[inline]
    pub unsafe fn getPropertyAsUint32(&self, prop: &[u16]) -> Result<uint32_t, nsresult> {
        let prop = nsString::from(prop);
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getPropertyAsUint32)(self as *const _, &*prop, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* int64_t getPropertyAsInt64 (in AString prop); */
    #[inline]
    pub unsafe fn getPropertyAsInt64(&self, prop: &[u16]) -> Result<int64_t, nsresult> {
        let prop = nsString::from(prop);
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).getPropertyAsInt64)(self as *const _, &*prop, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint64_t getPropertyAsUint64 (in AString prop); */
    #[inline]
    pub unsafe fn getPropertyAsUint64(&self, prop: &[u16]) -> Result<uint64_t, nsresult> {
        let prop = nsString::from(prop);
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).getPropertyAsUint64)(self as *const _, &*prop, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* double getPropertyAsDouble (in AString prop); */
    #[inline]
    pub unsafe fn getPropertyAsDouble(&self, prop: &[u16]) -> Result<libc::c_double, nsresult> {
        let prop = nsString::from(prop);
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).getPropertyAsDouble)(self as *const _, &*prop, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getPropertyAsAString (in AString prop); */
    #[inline]
    pub unsafe fn getPropertyAsAString(&self, prop: &[u16]) -> Result<nsString, nsresult> {
        let prop = nsString::from(prop);
        let mut _retval = nsString::new();
        match ((*self.vtable).getPropertyAsAString)(self as *const _, &*prop, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getPropertyAsACString (in AString prop); */
    #[inline]
    pub unsafe fn getPropertyAsACString(&self, prop: &[u16]) -> Result<nsCString, nsresult> {
        let prop = nsString::from(prop);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getPropertyAsACString)(self as *const _, &*prop, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String getPropertyAsAUTF8String (in AString prop); */
    #[inline]
    pub unsafe fn getPropertyAsAUTF8String(&self, prop: &[u16]) -> Result<nsCString, nsresult> {
        let prop = nsString::from(prop);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getPropertyAsAUTF8String)(self as *const _, &*prop, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean getPropertyAsBool (in AString prop); */
    #[inline]
    pub unsafe fn getPropertyAsBool(&self, prop: &[u16]) -> Result<bool, nsresult> {
        let prop = nsString::from(prop);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getPropertyAsBool)(self as *const _, &*prop, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getPropertyAsInterface (in AString prop, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn getPropertyAsInterface<T: XpCom>(&self, prop: &[u16]) -> Result<Option<RefPtr<T>>, nsresult> {
        let prop = nsString::from(prop);
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).getPropertyAsInterface)(self as *const _, &*prop, &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

    /* nsIVariant get (in AString prop); */
    #[inline]
    pub unsafe fn get(&self, prop: &[u16]) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let prop = nsString::from(prop);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get)(self as *const _, &*prop, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean hasKey (in AString prop); */
    #[inline]
    pub unsafe fn hasKey(&self, prop: &[u16]) -> Result<bool, nsresult> {
        let prop = nsString::from(prop);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasKey)(self as *const _, &*prop, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



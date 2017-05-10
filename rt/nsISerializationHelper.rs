//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISerializationHelper.idl
//


#[repr(C)]
pub struct nsISerializationHelper {
    vtable: *const nsISerializationHelperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISerializationHelper {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x31654c0f, 0x35f3, 0x44c6,
            [0xb3, 0x1e, 0x37, 0xa1, 0x15, 0x16, 0xe6, 0xbc])
    }
}

unsafe impl RefCounted for nsISerializationHelper {
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
pub trait nsISerializationHelperCoerce {
    fn coerce_from(v: &nsISerializationHelper) -> &Self;
}

impl nsISerializationHelperCoerce for nsISerializationHelper {
    #[inline]
    fn coerce_from(v: &nsISerializationHelper) -> &Self {
        v
    }
}

impl nsISerializationHelper {
    #[inline]
    pub fn coerce<T: nsISerializationHelperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISerializationHelper {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISerializationHelperCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISerializationHelper) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISerializationHelperVTable {
    pub __base: nsISupportsVTable,

    /* ACString serializeToString (in nsISerializable serializable); */
    pub serializeToString: unsafe extern "C" fn (this: *const nsISerializationHelper, serializable: *const nsISerializable, _retval: *mut nsACString) -> nsresult,

    /* nsISupports deserializeObject (in ACString input); */
    pub deserializeObject: unsafe extern "C" fn (this: *const nsISerializationHelper, input: *const nsACString, _retval: *mut *const nsISupports) -> nsresult,

}


impl nsISerializationHelper {
    /* ACString serializeToString (in nsISerializable serializable); */
    #[inline]
    pub unsafe fn serializeToString(&self, serializable: Option<&nsISerializable>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).serializeToString)(self as *const _, serializable.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISupports deserializeObject (in ACString input); */
    #[inline]
    pub unsafe fn deserializeObject(&self, input: &[u8]) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let input = nsCString::from(input);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).deserializeObject)(self as *const _, &*input, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



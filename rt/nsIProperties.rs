//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProperties.idl
//


#[repr(C)]
pub struct nsIProperties {
    vtable: *const nsIPropertiesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProperties {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x78650582, 0x4e93, 0x4b60,
            [0x8e, 0x85, 0x26, 0xeb, 0xd3, 0xeb, 0x14, 0xca])
    }
}

unsafe impl RefCounted for nsIProperties {
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
pub trait nsIPropertiesCoerce {
    fn coerce_from(v: &nsIProperties) -> &Self;
}

impl nsIPropertiesCoerce for nsIProperties {
    #[inline]
    fn coerce_from(v: &nsIProperties) -> &Self {
        v
    }
}

impl nsIProperties {
    #[inline]
    pub fn coerce<T: nsIPropertiesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProperties {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPropertiesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProperties) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPropertiesVTable {
    pub __base: nsISupportsVTable,

    /* void get (in string prop, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    pub get: unsafe extern "C" fn (this: *const nsIProperties, prop: *const libc::c_char, iid: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

    /* void set (in string prop, in nsISupports value); */
    pub set: unsafe extern "C" fn (this: *const nsIProperties, prop: *const libc::c_char, value: *const nsISupports) -> nsresult,

    /* boolean has (in string prop); */
    pub has: unsafe extern "C" fn (this: *const nsIProperties, prop: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* void undefine (in string prop); */
    pub undefine: unsafe extern "C" fn (this: *const nsIProperties, prop: *const libc::c_char) -> nsresult,

    /* void getKeys (out uint32_t count, [array, size_is (count), retval] out string keys); */
    /// Unable to call function as its signature contains a non-rust type
    pub getKeys: *const ::libc::c_void,

}


impl nsIProperties {
    /* void get (in string prop, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn get<T: XpCom>(&self, prop: *const libc::c_char) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).get)(self as *const _, prop, &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

    /* void set (in string prop, in nsISupports value); */
    #[inline]
    pub unsafe fn set(&self, prop: *const libc::c_char, value: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set)(self as *const _, prop, value.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean has (in string prop); */
    #[inline]
    pub unsafe fn has(&self, prop: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).has)(self as *const _, prop, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void undefine (in string prop); */
    #[inline]
    pub unsafe fn undefine(&self, prop: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).undefine)(self as *const _, prop) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getKeys (out uint32_t count, [array, size_is (count), retval] out string keys); */


}



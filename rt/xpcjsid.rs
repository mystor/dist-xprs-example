//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpcjsid.idl
//


#[repr(C)]
pub struct nsIJSID {
    vtable: *const nsIJSIDVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIJSID {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x62883d14, 0x4146, 0x4039,
            [0x94, 0xf5, 0xa5, 0xe1, 0xe1, 0xa5, 0x1a, 0x15])
    }
}

unsafe impl RefCounted for nsIJSID {
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
pub trait nsIJSIDCoerce {
    fn coerce_from(v: &nsIJSID) -> &Self;
}

impl nsIJSIDCoerce for nsIJSID {
    #[inline]
    fn coerce_from(v: &nsIJSID) -> &Self {
        v
    }
}

impl nsIJSID {
    #[inline]
    pub fn coerce<T: nsIJSIDCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIJSID {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIJSIDCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJSID) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIJSIDVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute string name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIJSID, aName: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute string number; */
    pub get_number: unsafe extern "C" fn (this: *const nsIJSID, aNumber: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute boolean valid; */
    pub get_valid: unsafe extern "C" fn (this: *const nsIJSID, aValid: *mut bool) -> nsresult,

    /* boolean equals (in nsIJSID other); */
    pub equals: unsafe extern "C" fn (this: *const nsIJSID, other: *const nsIJSID, _retval: *mut bool) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsIJSID, _retval: *mut *const libc::c_char) -> nsresult,

    /* [noscript] void initialize (in string idString); */
    pub initialize: unsafe extern "C" fn (this: *const nsIJSID, idString: *const libc::c_char) -> nsresult,

    /* [notxpcom] const_nsID_ptr getID (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getID: *const ::libc::c_void,

}


impl nsIJSID {
    /* readonly attribute string name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_name)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string number; */
    #[inline]
    pub unsafe fn get_number(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_number)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean valid; */
    #[inline]
    pub unsafe fn get_valid(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_valid)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean equals (in nsIJSID other); */
    #[inline]
    pub unsafe fn equals(&self, other: Option<&nsIJSID>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).equals)(self as *const _, other.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void initialize (in string idString); */
    #[inline]
    pub unsafe fn initialize(&self, idString: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).initialize)(self as *const _, idString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [notxpcom] const_nsID_ptr getID (); */


}


#[repr(C)]
pub struct nsIJSIID {
    vtable: *const nsIJSIIDVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIJSIID {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe76ec564, 0xa080, 0x4705,
            [0x86, 0x09, 0x38, 0x4c, 0x75, 0x5e, 0xc9, 0x1e])
    }
}

unsafe impl RefCounted for nsIJSIID {
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
pub trait nsIJSIIDCoerce {
    fn coerce_from(v: &nsIJSIID) -> &Self;
}

impl nsIJSIIDCoerce for nsIJSIID {
    #[inline]
    fn coerce_from(v: &nsIJSIID) -> &Self {
        v
    }
}

impl nsIJSIID {
    #[inline]
    pub fn coerce<T: nsIJSIIDCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIJSIID {
    type Target = nsIJSID;
    #[inline]
    fn deref(&self) -> &nsIJSID {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIJSIDCoerce> nsIJSIIDCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJSIID) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIJSIIDVTable {
    pub __base: nsIJSIDVTable,

}


impl nsIJSIID {
}


#[repr(C)]
pub struct nsIJSCID {
    vtable: *const nsIJSCIDVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIJSCID {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbf5eb086, 0x9eaa, 0x4694,
            [0xae, 0xc3, 0xfe, 0x4a, 0xac, 0x61, 0x19, 0xbd])
    }
}

unsafe impl RefCounted for nsIJSCID {
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
pub trait nsIJSCIDCoerce {
    fn coerce_from(v: &nsIJSCID) -> &Self;
}

impl nsIJSCIDCoerce for nsIJSCID {
    #[inline]
    fn coerce_from(v: &nsIJSCID) -> &Self {
        v
    }
}

impl nsIJSCID {
    #[inline]
    pub fn coerce<T: nsIJSCIDCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIJSCID {
    type Target = nsIJSID;
    #[inline]
    fn deref(&self) -> &nsIJSID {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIJSIDCoerce> nsIJSCIDCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJSCID) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIJSCIDVTable {
    pub __base: nsIJSIDVTable,

    /* [implicit_jscontext,optional_argc] jsval createInstance ([optional] in jsval iid); */
    /// Unable to call function as its signature contains a non-rust type
    pub createInstance: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] jsval getService ([optional] in jsval iid); */
    /// Unable to call function as its signature contains a non-rust type
    pub getService: *const ::libc::c_void,

}


impl nsIJSCID {
    /* [implicit_jscontext,optional_argc] jsval createInstance ([optional] in jsval iid); */


    /* [implicit_jscontext,optional_argc] jsval getService ([optional] in jsval iid); */


}



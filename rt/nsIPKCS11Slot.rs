//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPKCS11Slot.idl
//


pub mod nsIPKCS11Slot_consts {
    pub const SLOT_DISABLED: i64 = 0;
    pub const SLOT_NOT_PRESENT: i64 = 1;
    pub const SLOT_UNINITIALIZED: i64 = 2;
    pub const SLOT_NOT_LOGGED_IN: i64 = 3;
    pub const SLOT_LOGGED_IN: i64 = 4;
    pub const SLOT_READY: i64 = 5;
}


#[repr(C)]
pub struct nsIPKCS11Slot {
    vtable: *const nsIPKCS11SlotVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPKCS11Slot {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc2d4f296, 0xee60, 0x11d4,
            [0x99, 0x8b, 0x00, 0xb0, 0xd0, 0x23, 0x54, 0xa0])
    }
}

unsafe impl RefCounted for nsIPKCS11Slot {
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
pub trait nsIPKCS11SlotCoerce {
    fn coerce_from(v: &nsIPKCS11Slot) -> &Self;
}

impl nsIPKCS11SlotCoerce for nsIPKCS11Slot {
    #[inline]
    fn coerce_from(v: &nsIPKCS11Slot) -> &Self {
        v
    }
}

impl nsIPKCS11Slot {
    #[inline]
    pub fn coerce<T: nsIPKCS11SlotCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPKCS11Slot {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPKCS11SlotCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPKCS11Slot) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPKCS11SlotVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIPKCS11Slot, aName: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String desc; */
    pub get_desc: unsafe extern "C" fn (this: *const nsIPKCS11Slot, aDesc: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String manID; */
    pub get_manID: unsafe extern "C" fn (this: *const nsIPKCS11Slot, aManID: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String HWVersion; */
    pub get_HWVersion: unsafe extern "C" fn (this: *const nsIPKCS11Slot, aHWVersion: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String FWVersion; */
    pub get_FWVersion: unsafe extern "C" fn (this: *const nsIPKCS11Slot, aFWVersion: *mut nsACString) -> nsresult,

    /* readonly attribute unsigned long status; */
    pub get_status: unsafe extern "C" fn (this: *const nsIPKCS11Slot, aStatus: *mut libc::uint32_t) -> nsresult,

    /* nsIPK11Token getToken (); */
    pub getToken: unsafe extern "C" fn (this: *const nsIPKCS11Slot, _retval: *mut *const nsIPK11Token) -> nsresult,

    /* readonly attribute AUTF8String tokenName; */
    pub get_tokenName: unsafe extern "C" fn (this: *const nsIPKCS11Slot, aTokenName: *mut nsACString) -> nsresult,

}


impl nsIPKCS11Slot {
    /* readonly attribute AUTF8String name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String desc; */
    #[inline]
    pub unsafe fn get_desc(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_desc)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String manID; */
    #[inline]
    pub unsafe fn get_manID(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_manID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String HWVersion; */
    #[inline]
    pub unsafe fn get_HWVersion(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_HWVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String FWVersion; */
    #[inline]
    pub unsafe fn get_FWVersion(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_FWVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long status; */
    #[inline]
    pub unsafe fn get_status(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_status)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIPK11Token getToken (); */
    #[inline]
    pub unsafe fn getToken(&self, ) -> Result<Option<RefPtr<nsIPK11Token>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getToken)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AUTF8String tokenName; */
    #[inline]
    pub unsafe fn get_tokenName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_tokenName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMScreen.idl
//


#[repr(C)]
pub struct nsIDOMScreen {
    vtable: *const nsIDOMScreenVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMScreen {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x82c7924b, 0x4b46, 0x4e5a,
            [0xa8, 0xd2, 0x6e, 0xdb, 0x5f, 0xc0, 0xa6, 0x0d])
    }
}

unsafe impl RefCounted for nsIDOMScreen {
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
pub trait nsIDOMScreenCoerce {
    fn coerce_from(v: &nsIDOMScreen) -> &Self;
}

impl nsIDOMScreenCoerce for nsIDOMScreen {
    #[inline]
    fn coerce_from(v: &nsIDOMScreen) -> &Self {
        v
    }
}

impl nsIDOMScreen {
    #[inline]
    pub fn coerce<T: nsIDOMScreenCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMScreen {
    type Target = nsIDOMEventTarget;
    #[inline]
    fn deref(&self) -> &nsIDOMEventTarget {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMEventTargetCoerce> nsIDOMScreenCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMScreen) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMScreenVTable {
    pub __base: nsIDOMEventTargetVTable,

    /* readonly attribute long top; */
    pub get_top: unsafe extern "C" fn (this: *const nsIDOMScreen, aTop: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long availWidth; */
    pub get_availWidth: unsafe extern "C" fn (this: *const nsIDOMScreen, aAvailWidth: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long availHeight; */
    pub get_availHeight: unsafe extern "C" fn (this: *const nsIDOMScreen, aAvailHeight: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long availLeft; */
    pub get_availLeft: unsafe extern "C" fn (this: *const nsIDOMScreen, aAvailLeft: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long availTop; */
    pub get_availTop: unsafe extern "C" fn (this: *const nsIDOMScreen, aAvailTop: *mut libc::int32_t) -> nsresult,

}


impl nsIDOMScreen {
    /* readonly attribute long top; */
    #[inline]
    pub unsafe fn get_top(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_top)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long availWidth; */
    #[inline]
    pub unsafe fn get_availWidth(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_availWidth)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long availHeight; */
    #[inline]
    pub unsafe fn get_availHeight(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_availHeight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long availLeft; */
    #[inline]
    pub unsafe fn get_availLeft(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_availLeft)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long availTop; */
    #[inline]
    pub unsafe fn get_availTop(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_availTop)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScreenManager.idl
//


#[repr(C)]
pub struct nsIScreenManager {
    vtable: *const nsIScreenManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScreenManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe8a96e60, 0x6b61, 0x4a14,
            [0xba, 0xcc, 0x53, 0x89, 0x16, 0x04, 0xb5, 0x02])
    }
}

unsafe impl RefCounted for nsIScreenManager {
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
pub trait nsIScreenManagerCoerce {
    fn coerce_from(v: &nsIScreenManager) -> &Self;
}

impl nsIScreenManagerCoerce for nsIScreenManager {
    #[inline]
    fn coerce_from(v: &nsIScreenManager) -> &Self {
        v
    }
}

impl nsIScreenManager {
    #[inline]
    pub fn coerce<T: nsIScreenManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScreenManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScreenManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScreenManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScreenManagerVTable {
    pub __base: nsISupportsVTable,

    /* nsIScreen screenForRect (in long left, in long top, in long width, in long height); */
    pub screenForRect: unsafe extern "C" fn (this: *const nsIScreenManager, left: libc::int32_t, top: libc::int32_t, width: libc::int32_t, height: libc::int32_t, _retval: *mut *const nsIScreen) -> nsresult,

    /* readonly attribute nsIScreen primaryScreen; */
    pub get_primaryScreen: unsafe extern "C" fn (this: *const nsIScreenManager, aPrimaryScreen: *mut *const nsIScreen) -> nsresult,

}


impl nsIScreenManager {
    /* nsIScreen screenForRect (in long left, in long top, in long width, in long height); */
    #[inline]
    pub unsafe fn screenForRect(&self, left: libc::int32_t, top: libc::int32_t, width: libc::int32_t, height: libc::int32_t) -> Result<Option<RefPtr<nsIScreen>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).screenForRect)(self as *const _, left, top, width, height, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIScreen primaryScreen; */
    #[inline]
    pub unsafe fn get_primaryScreen(&self, ) -> Result<Option<RefPtr<nsIScreen>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_primaryScreen)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



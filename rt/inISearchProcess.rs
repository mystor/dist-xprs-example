//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/inISearchProcess.idl
//


#[repr(C)]
pub struct inISearchProcess {
    vtable: *const inISearchProcessVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for inISearchProcess {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd5fa765b, 0x2448, 0x4686,
            [0xb7, 0xc1, 0x5f, 0xf1, 0x3a, 0xcb, 0x0f, 0xc9])
    }
}

unsafe impl RefCounted for inISearchProcess {
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
pub trait inISearchProcessCoerce {
    fn coerce_from(v: &inISearchProcess) -> &Self;
}

impl inISearchProcessCoerce for inISearchProcess {
    #[inline]
    fn coerce_from(v: &inISearchProcess) -> &Self {
        v
    }
}

impl inISearchProcess {
    #[inline]
    pub fn coerce<T: inISearchProcessCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for inISearchProcess {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> inISearchProcessCoerce for T {
    #[inline]
    fn coerce_from(v: &inISearchProcess) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct inISearchProcessVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isActive; */
    pub get_isActive: unsafe extern "C" fn (this: *const inISearchProcess, aIsActive: *mut bool) -> nsresult,

    /* readonly attribute long resultCount; */
    pub get_resultCount: unsafe extern "C" fn (this: *const inISearchProcess, aResultCount: *mut libc::int32_t) -> nsresult,

    /* attribute boolean holdResults; */
    pub get_holdResults: unsafe extern "C" fn (this: *const inISearchProcess, aHoldResults: *mut bool) -> nsresult,
    pub set_holdResults: unsafe extern "C" fn (this: *const inISearchProcess, aHoldResults: bool) -> nsresult,

    /* void searchSync (); */
    pub searchSync: unsafe extern "C" fn (this: *const inISearchProcess) -> nsresult,

    /* void searchAsync (in inISearchObserver aObserver); */
    pub searchAsync: unsafe extern "C" fn (this: *const inISearchProcess, aObserver: *const inISearchObserver) -> nsresult,

    /* void searchStop (); */
    pub searchStop: unsafe extern "C" fn (this: *const inISearchProcess) -> nsresult,

    /* boolean searchStep (); */
    pub searchStep: unsafe extern "C" fn (this: *const inISearchProcess, _retval: *mut bool) -> nsresult,

    /* AString getStringResultAt (in long aIndex); */
    pub getStringResultAt: unsafe extern "C" fn (this: *const inISearchProcess, aIndex: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* long getIntResultAt (in long aIndex); */
    pub getIntResultAt: unsafe extern "C" fn (this: *const inISearchProcess, aIndex: libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* unsigned long getUIntResultAt (in long aIndex); */
    pub getUIntResultAt: unsafe extern "C" fn (this: *const inISearchProcess, aIndex: libc::int32_t, _retval: *mut libc::uint32_t) -> nsresult,

}


impl inISearchProcess {
    /* readonly attribute boolean isActive; */
    #[inline]
    pub unsafe fn get_isActive(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isActive)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long resultCount; */
    #[inline]
    pub unsafe fn get_resultCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_resultCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean holdResults; */
    #[inline]
    pub unsafe fn get_holdResults(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_holdResults)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_holdResults(&self, aHoldResults: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_holdResults)(self as *const _, aHoldResults) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void searchSync (); */
    #[inline]
    pub unsafe fn searchSync(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).searchSync)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void searchAsync (in inISearchObserver aObserver); */
    #[inline]
    pub unsafe fn searchAsync(&self, aObserver: Option<&inISearchObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).searchAsync)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void searchStop (); */
    #[inline]
    pub unsafe fn searchStop(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).searchStop)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean searchStep (); */
    #[inline]
    pub unsafe fn searchStep(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).searchStep)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getStringResultAt (in long aIndex); */
    #[inline]
    pub unsafe fn getStringResultAt(&self, aIndex: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getStringResultAt)(self as *const _, aIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getIntResultAt (in long aIndex); */
    #[inline]
    pub unsafe fn getIntResultAt(&self, aIndex: libc::int32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getIntResultAt)(self as *const _, aIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* unsigned long getUIntResultAt (in long aIndex); */
    #[inline]
    pub unsafe fn getUIntResultAt(&self, aIndex: libc::int32_t) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getUIntResultAt)(self as *const _, aIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



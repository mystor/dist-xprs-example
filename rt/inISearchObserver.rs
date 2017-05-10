//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/inISearchObserver.idl
//


pub mod inISearchObserver_consts {
    pub const IN_SUCCESS: i64 = 1;
    pub const IN_INTERRUPTED: i64 = 2;
    pub const IN_ERROR: i64 = 3;
}


#[repr(C)]
pub struct inISearchObserver {
    vtable: *const inISearchObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for inISearchObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x46226d9b, 0xe398, 0x4106,
            [0x8d, 0x9b, 0x22, 0x5d, 0x4d, 0x05, 0x89, 0xf5])
    }
}

unsafe impl RefCounted for inISearchObserver {
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
pub trait inISearchObserverCoerce {
    fn coerce_from(v: &inISearchObserver) -> &Self;
}

impl inISearchObserverCoerce for inISearchObserver {
    #[inline]
    fn coerce_from(v: &inISearchObserver) -> &Self {
        v
    }
}

impl inISearchObserver {
    #[inline]
    pub fn coerce<T: inISearchObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for inISearchObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> inISearchObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &inISearchObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct inISearchObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onSearchStart (in inISearchProcess aModule); */
    pub onSearchStart: unsafe extern "C" fn (this: *const inISearchObserver, aModule: *const inISearchProcess) -> nsresult,

    /* void onSearchResult (in inISearchProcess aModule); */
    pub onSearchResult: unsafe extern "C" fn (this: *const inISearchObserver, aModule: *const inISearchProcess) -> nsresult,

    /* void onSearchEnd (in inISearchProcess aModule, in short aResult); */
    pub onSearchEnd: unsafe extern "C" fn (this: *const inISearchObserver, aModule: *const inISearchProcess, aResult: libc::int16_t) -> nsresult,

    /* void onSearchError (in inISearchProcess aModule, in AString aMessage); */
    pub onSearchError: unsafe extern "C" fn (this: *const inISearchObserver, aModule: *const inISearchProcess, aMessage: *const nsAString) -> nsresult,

}


impl inISearchObserver {
    /* void onSearchStart (in inISearchProcess aModule); */
    #[inline]
    pub unsafe fn onSearchStart(&self, aModule: Option<&inISearchProcess>) -> Result<(), nsresult> {

        match ((*self.vtable).onSearchStart)(self as *const _, aModule.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onSearchResult (in inISearchProcess aModule); */
    #[inline]
    pub unsafe fn onSearchResult(&self, aModule: Option<&inISearchProcess>) -> Result<(), nsresult> {

        match ((*self.vtable).onSearchResult)(self as *const _, aModule.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onSearchEnd (in inISearchProcess aModule, in short aResult); */
    #[inline]
    pub unsafe fn onSearchEnd(&self, aModule: Option<&inISearchProcess>, aResult: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).onSearchEnd)(self as *const _, aModule.map_or(::std::ptr::null(), |x| x as *const _), aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onSearchError (in inISearchProcess aModule, in AString aMessage); */
    #[inline]
    pub unsafe fn onSearchError(&self, aModule: Option<&inISearchProcess>, aMessage: &[u16]) -> Result<(), nsresult> {
        let aMessage = nsString::from(aMessage);
        match ((*self.vtable).onSearchError)(self as *const _, aModule.map_or(::std::ptr::null(), |x| x as *const _), &*aMessage) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



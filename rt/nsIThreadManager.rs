//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIThreadManager.idl
//


pub mod nsIThreadManager_consts {
    pub const DEFAULT_STACK_SIZE: i64 = 0;
}


#[repr(C)]
pub struct nsIThreadManager {
    vtable: *const nsIThreadManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIThreadManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1be89eca, 0xe2f7, 0x453b,
            [0x8d, 0x38, 0xc1, 0x1b, 0xa2, 0x47, 0xf6, 0xf3])
    }
}

unsafe impl RefCounted for nsIThreadManager {
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
pub trait nsIThreadManagerCoerce {
    fn coerce_from(v: &nsIThreadManager) -> &Self;
}

impl nsIThreadManagerCoerce for nsIThreadManager {
    #[inline]
    fn coerce_from(v: &nsIThreadManager) -> &Self {
        v
    }
}

impl nsIThreadManager {
    #[inline]
    pub fn coerce<T: nsIThreadManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIThreadManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIThreadManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThreadManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIThreadManagerVTable {
    pub __base: nsISupportsVTable,

    /* nsIThread newThread (in unsigned long creationFlags, [optional] in unsigned long stackSize); */
    pub newThread: unsafe extern "C" fn (this: *const nsIThreadManager, creationFlags: libc::uint32_t, stackSize: libc::uint32_t, _retval: *mut *const nsIThread) -> nsresult,

    /* [noscript] nsIThread newNamedThread (in ACString name, [optional] in unsigned long stackSize); */
    pub newNamedThread: unsafe extern "C" fn (this: *const nsIThreadManager, name: *const nsACString, stackSize: libc::uint32_t, _retval: *mut *const nsIThread) -> nsresult,

    /* [noscript] nsIThread getThreadFromPRThread (in PRThread prthread); */
    /// Unable to call function as its signature contains a non-rust type
    pub getThreadFromPRThread: *const ::libc::c_void,

    /* readonly attribute nsIThread mainThread; */
    pub get_mainThread: unsafe extern "C" fn (this: *const nsIThreadManager, aMainThread: *mut *const nsIThread) -> nsresult,

    /* readonly attribute nsIThread currentThread; */
    pub get_currentThread: unsafe extern "C" fn (this: *const nsIThreadManager, aCurrentThread: *mut *const nsIThread) -> nsresult,

    /* readonly attribute boolean isMainThread; */
    pub get_isMainThread: unsafe extern "C" fn (this: *const nsIThreadManager, aIsMainThread: *mut bool) -> nsresult,

    /* void dispatchToMainThread (in nsIRunnable event); */
    pub dispatchToMainThread: unsafe extern "C" fn (this: *const nsIThreadManager, event: *const nsIRunnable) -> nsresult,

}


impl nsIThreadManager {
    /* nsIThread newThread (in unsigned long creationFlags, [optional] in unsigned long stackSize); */
    #[inline]
    pub unsafe fn newThread(&self, creationFlags: libc::uint32_t, stackSize: libc::uint32_t) -> Result<Option<RefPtr<nsIThread>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newThread)(self as *const _, creationFlags, stackSize, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] nsIThread newNamedThread (in ACString name, [optional] in unsigned long stackSize); */
    #[inline]
    pub unsafe fn newNamedThread(&self, name: &[u8], stackSize: libc::uint32_t) -> Result<Option<RefPtr<nsIThread>>, nsresult> {
        let name = nsCString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newNamedThread)(self as *const _, &*name, stackSize, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] nsIThread getThreadFromPRThread (in PRThread prthread); */


    /* readonly attribute nsIThread mainThread; */
    #[inline]
    pub unsafe fn get_mainThread(&self, ) -> Result<Option<RefPtr<nsIThread>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_mainThread)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIThread currentThread; */
    #[inline]
    pub unsafe fn get_currentThread(&self, ) -> Result<Option<RefPtr<nsIThread>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentThread)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean isMainThread; */
    #[inline]
    pub unsafe fn get_isMainThread(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isMainThread)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void dispatchToMainThread (in nsIRunnable event); */
    #[inline]
    pub unsafe fn dispatchToMainThread(&self, event: Option<&nsIRunnable>) -> Result<(), nsresult> {

        match ((*self.vtable).dispatchToMainThread)(self as *const _, event.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPowerManagerService.idl
//


#[repr(C)]
pub struct nsIPowerManagerService {
    vtable: *const nsIPowerManagerServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPowerManagerService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xba7ca4c1, 0x9d92, 0x4425,
            [0xa8, 0x3b, 0x85, 0xdd, 0x7f, 0xa9, 0x53, 0xf7])
    }
}

unsafe impl RefCounted for nsIPowerManagerService {
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
pub trait nsIPowerManagerServiceCoerce {
    fn coerce_from(v: &nsIPowerManagerService) -> &Self;
}

impl nsIPowerManagerServiceCoerce for nsIPowerManagerService {
    #[inline]
    fn coerce_from(v: &nsIPowerManagerService) -> &Self {
        v
    }
}

impl nsIPowerManagerService {
    #[inline]
    pub fn coerce<T: nsIPowerManagerServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPowerManagerService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPowerManagerServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPowerManagerService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPowerManagerServiceVTable {
    pub __base: nsISupportsVTable,

    /* void powerOff (); */
    pub powerOff: unsafe extern "C" fn (this: *const nsIPowerManagerService) -> nsresult,

    /* void reboot (); */
    pub reboot: unsafe extern "C" fn (this: *const nsIPowerManagerService) -> nsresult,

    /* void restart (); */
    pub restart: unsafe extern "C" fn (this: *const nsIPowerManagerService) -> nsresult,

    /* void addWakeLockListener (in nsIDOMMozWakeLockListener aListener); */
    pub addWakeLockListener: unsafe extern "C" fn (this: *const nsIPowerManagerService, aListener: *const nsIDOMMozWakeLockListener) -> nsresult,

    /* void removeWakeLockListener (in nsIDOMMozWakeLockListener aListener); */
    pub removeWakeLockListener: unsafe extern "C" fn (this: *const nsIPowerManagerService, aListener: *const nsIDOMMozWakeLockListener) -> nsresult,

    /* DOMString getWakeLockState (in DOMString aTopic); */
    pub getWakeLockState: unsafe extern "C" fn (this: *const nsIPowerManagerService, aTopic: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* nsISupports newWakeLock (in DOMString aTopic, [optional] in mozIDOMWindow aWindow); */
    pub newWakeLock: unsafe extern "C" fn (this: *const nsIPowerManagerService, aTopic: *const nsAString, aWindow: *const mozIDOMWindow, _retval: *mut *const nsISupports) -> nsresult,

}


impl nsIPowerManagerService {
    /* void powerOff (); */
    #[inline]
    pub unsafe fn powerOff(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).powerOff)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reboot (); */
    #[inline]
    pub unsafe fn reboot(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reboot)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void restart (); */
    #[inline]
    pub unsafe fn restart(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).restart)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addWakeLockListener (in nsIDOMMozWakeLockListener aListener); */
    #[inline]
    pub unsafe fn addWakeLockListener(&self, aListener: Option<&nsIDOMMozWakeLockListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addWakeLockListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeWakeLockListener (in nsIDOMMozWakeLockListener aListener); */
    #[inline]
    pub unsafe fn removeWakeLockListener(&self, aListener: Option<&nsIDOMMozWakeLockListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeWakeLockListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* DOMString getWakeLockState (in DOMString aTopic); */
    #[inline]
    pub unsafe fn getWakeLockState(&self, aTopic: &[u16]) -> Result<nsString, nsresult> {
        let aTopic = nsString::from(aTopic);
        let mut _retval = nsString::new();
        match ((*self.vtable).getWakeLockState)(self as *const _, &*aTopic, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISupports newWakeLock (in DOMString aTopic, [optional] in mozIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn newWakeLock(&self, aTopic: &[u16], aWindow: Option<&mozIDOMWindow>) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let aTopic = nsString::from(aTopic);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newWakeLock)(self as *const _, &*aTopic, aWindow.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIConsoleService.idl
//


#[repr(C)]
pub struct nsIConsoleService {
    vtable: *const nsIConsoleServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIConsoleService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0eb81d20, 0xc37e, 0x42d4,
            [0x82, 0xa8, 0xca, 0x9a, 0xe9, 0x6b, 0xdf, 0x52])
    }
}

unsafe impl RefCounted for nsIConsoleService {
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
pub trait nsIConsoleServiceCoerce {
    fn coerce_from(v: &nsIConsoleService) -> &Self;
}

impl nsIConsoleServiceCoerce for nsIConsoleService {
    #[inline]
    fn coerce_from(v: &nsIConsoleService) -> &Self {
        v
    }
}

impl nsIConsoleService {
    #[inline]
    pub fn coerce<T: nsIConsoleServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIConsoleService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIConsoleServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIConsoleService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIConsoleServiceVTable {
    pub __base: nsISupportsVTable,

    /* void logMessage (in nsIConsoleMessage message); */
    pub logMessage: unsafe extern "C" fn (this: *const nsIConsoleService, message: *const nsIConsoleMessage) -> nsresult,

    /* void logStringMessage (in wstring message); */
    pub logStringMessage: unsafe extern "C" fn (this: *const nsIConsoleService, message: *const libc::int16_t) -> nsresult,

    /* void getMessageArray ([optional] out uint32_t count, [array, size_is (count), retval] out nsIConsoleMessage messages); */
    /// Unable to call function as its signature contains a non-rust type
    pub getMessageArray: *const ::libc::c_void,

    /* void registerListener (in nsIConsoleListener listener); */
    pub registerListener: unsafe extern "C" fn (this: *const nsIConsoleService, listener: *const nsIConsoleListener) -> nsresult,

    /* void unregisterListener (in nsIConsoleListener listener); */
    pub unregisterListener: unsafe extern "C" fn (this: *const nsIConsoleService, listener: *const nsIConsoleListener) -> nsresult,

    /* void reset (); */
    pub reset: unsafe extern "C" fn (this: *const nsIConsoleService) -> nsresult,

}


impl nsIConsoleService {
    /* void logMessage (in nsIConsoleMessage message); */
    #[inline]
    pub unsafe fn logMessage(&self, message: Option<&nsIConsoleMessage>) -> Result<(), nsresult> {

        match ((*self.vtable).logMessage)(self as *const _, message.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void logStringMessage (in wstring message); */
    #[inline]
    pub unsafe fn logStringMessage(&self, message: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).logStringMessage)(self as *const _, message) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getMessageArray ([optional] out uint32_t count, [array, size_is (count), retval] out nsIConsoleMessage messages); */


    /* void registerListener (in nsIConsoleListener listener); */
    #[inline]
    pub unsafe fn registerListener(&self, listener: Option<&nsIConsoleListener>) -> Result<(), nsresult> {

        match ((*self.vtable).registerListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterListener (in nsIConsoleListener listener); */
    #[inline]
    pub unsafe fn unregisterListener(&self, listener: Option<&nsIConsoleListener>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reset (); */
    #[inline]
    pub unsafe fn reset(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reset)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



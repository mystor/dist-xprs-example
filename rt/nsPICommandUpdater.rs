//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPICommandUpdater.idl
//


#[repr(C)]
pub struct nsPICommandUpdater {
    vtable: *const nsPICommandUpdaterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsPICommandUpdater {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x35e474ae, 0x8016, 0x4c34,
            [0x96, 0x44, 0xed, 0xc1, 0x1f, 0x8b, 0x0c, 0xe1])
    }
}

unsafe impl RefCounted for nsPICommandUpdater {
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
pub trait nsPICommandUpdaterCoerce {
    fn coerce_from(v: &nsPICommandUpdater) -> &Self;
}

impl nsPICommandUpdaterCoerce for nsPICommandUpdater {
    #[inline]
    fn coerce_from(v: &nsPICommandUpdater) -> &Self {
        v
    }
}

impl nsPICommandUpdater {
    #[inline]
    pub fn coerce<T: nsPICommandUpdaterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsPICommandUpdater {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsPICommandUpdaterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPICommandUpdater) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsPICommandUpdaterVTable {
    pub __base: nsISupportsVTable,

    /* void init (in mozIDOMWindowProxy aWindow); */
    pub init: unsafe extern "C" fn (this: *const nsPICommandUpdater, aWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* void commandStatusChanged (in string aCommandName); */
    pub commandStatusChanged: unsafe extern "C" fn (this: *const nsPICommandUpdater, aCommandName: *const libc::c_char) -> nsresult,

}


impl nsPICommandUpdater {
    /* void init (in mozIDOMWindowProxy aWindow); */
    #[inline]
    pub unsafe fn init(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void commandStatusChanged (in string aCommandName); */
    #[inline]
    pub unsafe fn commandStatusChanged(&self, aCommandName: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).commandStatusChanged)(self as *const _, aCommandName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



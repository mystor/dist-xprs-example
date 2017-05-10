//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMModalContentWindow.idl
//


#[repr(C)]
pub struct nsIDOMModalContentWindow {
    vtable: *const nsIDOMModalContentWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMModalContentWindow {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3f4cb2d0, 0x5f7e, 0x44a9,
            [0x9f, 0x4f, 0x37, 0x09, 0x45, 0xf8, 0xdb, 0x08])
    }
}

unsafe impl RefCounted for nsIDOMModalContentWindow {
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
pub trait nsIDOMModalContentWindowCoerce {
    fn coerce_from(v: &nsIDOMModalContentWindow) -> &Self;
}

impl nsIDOMModalContentWindowCoerce for nsIDOMModalContentWindow {
    #[inline]
    fn coerce_from(v: &nsIDOMModalContentWindow) -> &Self {
        v
    }
}

impl nsIDOMModalContentWindow {
    #[inline]
    pub fn coerce<T: nsIDOMModalContentWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMModalContentWindow {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMModalContentWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMModalContentWindow) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMModalContentWindowVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIVariant dialogArguments; */
    pub get_dialogArguments: unsafe extern "C" fn (this: *const nsIDOMModalContentWindow, aDialogArguments: *mut *const nsIVariant) -> nsresult,

    /* attribute nsIVariant returnValue; */
    pub get_returnValue: unsafe extern "C" fn (this: *const nsIDOMModalContentWindow, aReturnValue: *mut *const nsIVariant) -> nsresult,
    pub set_returnValue: unsafe extern "C" fn (this: *const nsIDOMModalContentWindow, aReturnValue: *const nsIVariant) -> nsresult,

}


impl nsIDOMModalContentWindow {
    /* readonly attribute nsIVariant dialogArguments; */
    #[inline]
    pub unsafe fn get_dialogArguments(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_dialogArguments)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsIVariant returnValue; */
    #[inline]
    pub unsafe fn get_returnValue(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_returnValue)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_returnValue(&self, aReturnValue: Option<&nsIVariant>) -> Result<(), nsresult> {

        match ((*self.vtable).set_returnValue)(self as *const _, aReturnValue.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



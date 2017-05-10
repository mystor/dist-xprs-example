//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMMozBrowserFrame.idl
//


#[repr(C)]
pub struct nsIDOMMozBrowserFrame {
    vtable: *const nsIDOMMozBrowserFrameVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMMozBrowserFrame {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4cafe116, 0x581b, 0x4194,
            [0xb0, 0xde, 0x7f, 0x02, 0x37, 0x8f, 0xc5, 0x1d])
    }
}

unsafe impl RefCounted for nsIDOMMozBrowserFrame {
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
pub trait nsIDOMMozBrowserFrameCoerce {
    fn coerce_from(v: &nsIDOMMozBrowserFrame) -> &Self;
}

impl nsIDOMMozBrowserFrameCoerce for nsIDOMMozBrowserFrame {
    #[inline]
    fn coerce_from(v: &nsIDOMMozBrowserFrame) -> &Self {
        v
    }
}

impl nsIDOMMozBrowserFrame {
    #[inline]
    pub fn coerce<T: nsIDOMMozBrowserFrameCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMMozBrowserFrame {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMMozBrowserFrameCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMMozBrowserFrame) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMMozBrowserFrameVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean mozbrowser; */
    pub get_mozbrowser: unsafe extern "C" fn (this: *const nsIDOMMozBrowserFrame, aMozbrowser: *mut bool) -> nsresult,
    pub set_mozbrowser: unsafe extern "C" fn (this: *const nsIDOMMozBrowserFrame, aMozbrowser: bool) -> nsresult,

}


impl nsIDOMMozBrowserFrame {
    /* attribute boolean mozbrowser; */
    #[inline]
    pub unsafe fn get_mozbrowser(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mozbrowser)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_mozbrowser(&self, aMozbrowser: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_mozbrowser)(self as *const _, aMozbrowser) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



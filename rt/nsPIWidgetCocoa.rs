//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPIWidgetCocoa.idl
//


#[repr(C)]
pub struct nsPIWidgetCocoa {
    vtable: *const nsPIWidgetCocoaVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsPIWidgetCocoa {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf75ff69e, 0x3a51, 0x419e,
            [0xbd, 0x29, 0x04, 0x2f, 0x80, 0x4b, 0xc2, 0xed])
    }
}

unsafe impl RefCounted for nsPIWidgetCocoa {
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
pub trait nsPIWidgetCocoaCoerce {
    fn coerce_from(v: &nsPIWidgetCocoa) -> &Self;
}

impl nsPIWidgetCocoaCoerce for nsPIWidgetCocoa {
    #[inline]
    fn coerce_from(v: &nsPIWidgetCocoa) -> &Self {
        v
    }
}

impl nsPIWidgetCocoa {
    #[inline]
    pub fn coerce<T: nsPIWidgetCocoaCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsPIWidgetCocoa {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsPIWidgetCocoaCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPIWidgetCocoa) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsPIWidgetCocoaVTable {
    pub __base: nsISupportsVTable,

    /* void SendSetZLevelEvent (); */
    pub SendSetZLevelEvent: unsafe extern "C" fn (this: *const nsPIWidgetCocoa) -> nsresult,

    /* nsIWidget GetChildSheet (in boolean aShown); */
    pub GetChildSheet: unsafe extern "C" fn (this: *const nsPIWidgetCocoa, aShown: bool, _retval: *mut *const nsIWidget) -> nsresult,

    /* nsIWidget GetRealParent (); */
    pub GetRealParent: unsafe extern "C" fn (this: *const nsPIWidgetCocoa, _retval: *mut *const nsIWidget) -> nsresult,

    /* readonly attribute NSWindowPtr sheetWindowParent; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_sheetWindowParent: *const ::libc::c_void,

    /* readonly attribute boolean isSheet; */
    pub get_isSheet: unsafe extern "C" fn (this: *const nsPIWidgetCocoa, aIsSheet: *mut bool) -> nsresult,

}


impl nsPIWidgetCocoa {
    /* void SendSetZLevelEvent (); */
    #[inline]
    pub unsafe fn SendSetZLevelEvent(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).SendSetZLevelEvent)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIWidget GetChildSheet (in boolean aShown); */
    #[inline]
    pub unsafe fn GetChildSheet(&self, aShown: bool) -> Result<Option<RefPtr<nsIWidget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetChildSheet)(self as *const _, aShown, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIWidget GetRealParent (); */
    #[inline]
    pub unsafe fn GetRealParent(&self, ) -> Result<Option<RefPtr<nsIWidget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetRealParent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute NSWindowPtr sheetWindowParent; */


    /* readonly attribute boolean isSheet; */
    #[inline]
    pub unsafe fn get_isSheet(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSheet)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



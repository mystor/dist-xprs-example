//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMPaintRequest.idl
//


#[repr(C)]
pub struct nsIDOMPaintRequest {
    vtable: *const nsIDOMPaintRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMPaintRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9eb5268f, 0x73a4, 0x41da,
            [0x97, 0x90, 0xd2, 0x1f, 0xce, 0xfd, 0x5f, 0xfa])
    }
}

unsafe impl RefCounted for nsIDOMPaintRequest {
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
pub trait nsIDOMPaintRequestCoerce {
    fn coerce_from(v: &nsIDOMPaintRequest) -> &Self;
}

impl nsIDOMPaintRequestCoerce for nsIDOMPaintRequest {
    #[inline]
    fn coerce_from(v: &nsIDOMPaintRequest) -> &Self {
        v
    }
}

impl nsIDOMPaintRequest {
    #[inline]
    pub fn coerce<T: nsIDOMPaintRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMPaintRequest {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMPaintRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMPaintRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMPaintRequestVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMClientRect clientRect; */
    pub get_clientRect: unsafe extern "C" fn (this: *const nsIDOMPaintRequest, aClientRect: *mut *const nsIDOMClientRect) -> nsresult,

    /* [binaryname(XPCOMReason)] readonly attribute DOMString reason; */
    pub get_XPCOMReason: unsafe extern "C" fn (this: *const nsIDOMPaintRequest, aReason: *mut nsAString) -> nsresult,

}


impl nsIDOMPaintRequest {
    /* readonly attribute nsIDOMClientRect clientRect; */
    #[inline]
    pub unsafe fn get_clientRect(&self, ) -> Result<Option<RefPtr<nsIDOMClientRect>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_clientRect)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [binaryname(XPCOMReason)] readonly attribute DOMString reason; */
    #[inline]
    pub unsafe fn get_XPCOMReason(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_XPCOMReason)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



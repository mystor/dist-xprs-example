//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULControlElement.idl
//


#[repr(C)]
pub struct nsIDOMXULControlElement {
    vtable: *const nsIDOMXULControlElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULControlElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xea7f92d0, 0xb379, 0x4107,
            [0x91, 0xb4, 0x1e, 0x69, 0xbd, 0xd7, 0x71, 0xe3])
    }
}

unsafe impl RefCounted for nsIDOMXULControlElement {
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
pub trait nsIDOMXULControlElementCoerce {
    fn coerce_from(v: &nsIDOMXULControlElement) -> &Self;
}

impl nsIDOMXULControlElementCoerce for nsIDOMXULControlElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULControlElement) -> &Self {
        v
    }
}

impl nsIDOMXULControlElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULControlElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULControlElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMXULControlElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULControlElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULControlElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean disabled; */
    pub get_disabled: unsafe extern "C" fn (this: *const nsIDOMXULControlElement, aDisabled: *mut bool) -> nsresult,
    pub set_disabled: unsafe extern "C" fn (this: *const nsIDOMXULControlElement, aDisabled: bool) -> nsresult,

    /* attribute long tabIndex; */
    pub get_tabIndex: unsafe extern "C" fn (this: *const nsIDOMXULControlElement, aTabIndex: *mut libc::int32_t) -> nsresult,
    pub set_tabIndex: unsafe extern "C" fn (this: *const nsIDOMXULControlElement, aTabIndex: libc::int32_t) -> nsresult,

}


impl nsIDOMXULControlElement {
    /* attribute boolean disabled; */
    #[inline]
    pub unsafe fn get_disabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_disabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_disabled(&self, aDisabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_disabled)(self as *const _, aDisabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long tabIndex; */
    #[inline]
    pub unsafe fn get_tabIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_tabIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_tabIndex(&self, aTabIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_tabIndex)(self as *const _, aTabIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



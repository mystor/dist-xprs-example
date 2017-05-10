//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULCheckboxElement.idl
//


pub mod nsIDOMXULCheckboxElement_consts {
    pub const CHECKSTATE_UNCHECKED: i64 = 0;
    pub const CHECKSTATE_CHECKED: i64 = 1;
    pub const CHECKSTATE_MIXED: i64 = 2;
}


#[repr(C)]
pub struct nsIDOMXULCheckboxElement {
    vtable: *const nsIDOMXULCheckboxElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULCheckboxElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x745518ad, 0x3163, 0x41f0,
            [0xb3, 0x58, 0xc8, 0x1f, 0xb5, 0xa5, 0x87, 0xbc])
    }
}

unsafe impl RefCounted for nsIDOMXULCheckboxElement {
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
pub trait nsIDOMXULCheckboxElementCoerce {
    fn coerce_from(v: &nsIDOMXULCheckboxElement) -> &Self;
}

impl nsIDOMXULCheckboxElementCoerce for nsIDOMXULCheckboxElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULCheckboxElement) -> &Self {
        v
    }
}

impl nsIDOMXULCheckboxElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULCheckboxElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULCheckboxElement {
    type Target = nsIDOMXULLabeledControlElement;
    #[inline]
    fn deref(&self) -> &nsIDOMXULLabeledControlElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMXULLabeledControlElementCoerce> nsIDOMXULCheckboxElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULCheckboxElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULCheckboxElementVTable {
    pub __base: nsIDOMXULLabeledControlElementVTable,

    /* attribute boolean checked; */
    pub get_checked: unsafe extern "C" fn (this: *const nsIDOMXULCheckboxElement, aChecked: *mut bool) -> nsresult,
    pub set_checked: unsafe extern "C" fn (this: *const nsIDOMXULCheckboxElement, aChecked: bool) -> nsresult,

    /* attribute long checkState; */
    pub get_checkState: unsafe extern "C" fn (this: *const nsIDOMXULCheckboxElement, aCheckState: *mut libc::int32_t) -> nsresult,
    pub set_checkState: unsafe extern "C" fn (this: *const nsIDOMXULCheckboxElement, aCheckState: libc::int32_t) -> nsresult,

    /* attribute boolean autoCheck; */
    pub get_autoCheck: unsafe extern "C" fn (this: *const nsIDOMXULCheckboxElement, aAutoCheck: *mut bool) -> nsresult,
    pub set_autoCheck: unsafe extern "C" fn (this: *const nsIDOMXULCheckboxElement, aAutoCheck: bool) -> nsresult,

}


impl nsIDOMXULCheckboxElement {
    /* attribute boolean checked; */
    #[inline]
    pub unsafe fn get_checked(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_checked)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_checked(&self, aChecked: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_checked)(self as *const _, aChecked) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long checkState; */
    #[inline]
    pub unsafe fn get_checkState(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_checkState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_checkState(&self, aCheckState: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_checkState)(self as *const _, aCheckState) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean autoCheck; */
    #[inline]
    pub unsafe fn get_autoCheck(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_autoCheck)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_autoCheck(&self, aAutoCheck: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_autoCheck)(self as *const _, aAutoCheck) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



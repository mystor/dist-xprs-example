//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDroppedLinkHandler.idl
//


#[repr(C)]
pub struct nsIDroppedLinkItem {
    vtable: *const nsIDroppedLinkItemVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDroppedLinkItem {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x69e14f91, 0x2e09, 0x4ca6,
            [0xa5, 0x11, 0xa7, 0x15, 0xc9, 0x9a, 0x28, 0x04])
    }
}

unsafe impl RefCounted for nsIDroppedLinkItem {
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
pub trait nsIDroppedLinkItemCoerce {
    fn coerce_from(v: &nsIDroppedLinkItem) -> &Self;
}

impl nsIDroppedLinkItemCoerce for nsIDroppedLinkItem {
    #[inline]
    fn coerce_from(v: &nsIDroppedLinkItem) -> &Self {
        v
    }
}

impl nsIDroppedLinkItem {
    #[inline]
    pub fn coerce<T: nsIDroppedLinkItemCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDroppedLinkItem {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDroppedLinkItemCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDroppedLinkItem) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDroppedLinkItemVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString url; */
    pub get_url: unsafe extern "C" fn (this: *const nsIDroppedLinkItem, aUrl: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDroppedLinkItem, aName: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDroppedLinkItem, aType: *mut nsAString) -> nsresult,

}


impl nsIDroppedLinkItem {
    /* readonly attribute DOMString url; */
    #[inline]
    pub unsafe fn get_url(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_url)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIDroppedLinkHandler {
    vtable: *const nsIDroppedLinkHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDroppedLinkHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x21b5c25a, 0x28a9, 0x47bd,
            [0x84, 0x31, 0xfa, 0x91, 0x16, 0x30, 0x5d, 0xed])
    }
}

unsafe impl RefCounted for nsIDroppedLinkHandler {
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
pub trait nsIDroppedLinkHandlerCoerce {
    fn coerce_from(v: &nsIDroppedLinkHandler) -> &Self;
}

impl nsIDroppedLinkHandlerCoerce for nsIDroppedLinkHandler {
    #[inline]
    fn coerce_from(v: &nsIDroppedLinkHandler) -> &Self {
        v
    }
}

impl nsIDroppedLinkHandler {
    #[inline]
    pub fn coerce<T: nsIDroppedLinkHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDroppedLinkHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDroppedLinkHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDroppedLinkHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDroppedLinkHandlerVTable {
    pub __base: nsISupportsVTable,

    /* boolean canDropLink (in nsIDOMDragEvent aEvent, in boolean aAllowSameDocument); */
    pub canDropLink: unsafe extern "C" fn (this: *const nsIDroppedLinkHandler, aEvent: *const nsIDOMDragEvent, aAllowSameDocument: bool, _retval: *mut bool) -> nsresult,

    /* AString dropLink (in nsIDOMDragEvent aEvent, out AString aName, [optional] in boolean aDisallowInherit); */
    pub dropLink: unsafe extern "C" fn (this: *const nsIDroppedLinkHandler, aEvent: *const nsIDOMDragEvent, aName: *mut nsAString, aDisallowInherit: bool, _retval: *mut nsAString) -> nsresult,

    /* void dropLinks (in nsIDOMDragEvent aEvent, [optional] in boolean aDisallowInherit, [optional] out unsigned long aCount, [array, size_is (aCount), retval] out nsIDroppedLinkItem aLinks); */
    /// Unable to call function as its signature contains a non-rust type
    pub dropLinks: *const ::libc::c_void,

}


impl nsIDroppedLinkHandler {
    /* boolean canDropLink (in nsIDOMDragEvent aEvent, in boolean aAllowSameDocument); */
    #[inline]
    pub unsafe fn canDropLink(&self, aEvent: Option<&nsIDOMDragEvent>, aAllowSameDocument: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canDropLink)(self as *const _, aEvent.map_or(::std::ptr::null(), |x| x as *const _), aAllowSameDocument, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString dropLink (in nsIDOMDragEvent aEvent, out AString aName, [optional] in boolean aDisallowInherit); */
    #[inline]
    pub unsafe fn dropLink(&self, aEvent: Option<&nsIDOMDragEvent>, aDisallowInherit: bool) -> Result<(nsString, nsString), nsresult> {
        let mut aName = nsString::new();
        let mut _retval = nsString::new();
        match ((*self.vtable).dropLink)(self as *const _, aEvent.map_or(::std::ptr::null(), |x| x as *const _), &mut *aName, aDisallowInherit, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aName, _retval))
    }

    /* void dropLinks (in nsIDOMDragEvent aEvent, [optional] in boolean aDisallowInherit, [optional] out unsigned long aCount, [array, size_is (aCount), retval] out nsIDroppedLinkItem aLinks); */


}



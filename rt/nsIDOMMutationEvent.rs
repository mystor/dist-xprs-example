//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMMutationEvent.idl
//


pub mod nsIDOMMutationEvent_consts {
    pub const MODIFICATION: i64 = 1;
    pub const ADDITION: i64 = 2;
    pub const REMOVAL: i64 = 3;
}


#[repr(C)]
pub struct nsIDOMMutationEvent {
    vtable: *const nsIDOMMutationEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMMutationEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x30c9997f, 0xbc4c, 0x4890,
            [0xb8, 0x90, 0xfe, 0xbb, 0x6a, 0xe3, 0x05, 0x1b])
    }
}

unsafe impl RefCounted for nsIDOMMutationEvent {
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
pub trait nsIDOMMutationEventCoerce {
    fn coerce_from(v: &nsIDOMMutationEvent) -> &Self;
}

impl nsIDOMMutationEventCoerce for nsIDOMMutationEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMMutationEvent) -> &Self {
        v
    }
}

impl nsIDOMMutationEvent {
    #[inline]
    pub fn coerce<T: nsIDOMMutationEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMMutationEvent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMMutationEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMMutationEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMMutationEventVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMNode relatedNode; */
    pub get_relatedNode: unsafe extern "C" fn (this: *const nsIDOMMutationEvent, aRelatedNode: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute DOMString prevValue; */
    pub get_prevValue: unsafe extern "C" fn (this: *const nsIDOMMutationEvent, aPrevValue: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString newValue; */
    pub get_newValue: unsafe extern "C" fn (this: *const nsIDOMMutationEvent, aNewValue: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString attrName; */
    pub get_attrName: unsafe extern "C" fn (this: *const nsIDOMMutationEvent, aAttrName: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned short attrChange; */
    pub get_attrChange: unsafe extern "C" fn (this: *const nsIDOMMutationEvent, aAttrChange: *mut libc::uint16_t) -> nsresult,

    /* void initMutationEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in nsIDOMNode relatedNodeArg, in DOMString prevValueArg, in DOMString newValueArg, in DOMString attrNameArg, in unsigned short attrChangeArg); */
    pub initMutationEvent: unsafe extern "C" fn (this: *const nsIDOMMutationEvent, typeArg: *const nsAString, canBubbleArg: bool, cancelableArg: bool, relatedNodeArg: *const nsIDOMNode, prevValueArg: *const nsAString, newValueArg: *const nsAString, attrNameArg: *const nsAString, attrChangeArg: libc::uint16_t) -> nsresult,

}


impl nsIDOMMutationEvent {
    /* readonly attribute nsIDOMNode relatedNode; */
    #[inline]
    pub unsafe fn get_relatedNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_relatedNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute DOMString prevValue; */
    #[inline]
    pub unsafe fn get_prevValue(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_prevValue)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString newValue; */
    #[inline]
    pub unsafe fn get_newValue(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_newValue)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString attrName; */
    #[inline]
    pub unsafe fn get_attrName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_attrName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned short attrChange; */
    #[inline]
    pub unsafe fn get_attrChange(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_attrChange)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void initMutationEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in nsIDOMNode relatedNodeArg, in DOMString prevValueArg, in DOMString newValueArg, in DOMString attrNameArg, in unsigned short attrChangeArg); */
    #[inline]
    pub unsafe fn initMutationEvent(&self, typeArg: &[u16], canBubbleArg: bool, cancelableArg: bool, relatedNodeArg: Option<&nsIDOMNode>, prevValueArg: &[u16], newValueArg: &[u16], attrNameArg: &[u16], attrChangeArg: libc::uint16_t) -> Result<(), nsresult> {
        let typeArg = nsString::from(typeArg);
        let prevValueArg = nsString::from(prevValueArg);
        let newValueArg = nsString::from(newValueArg);
        let attrNameArg = nsString::from(attrNameArg);
        match ((*self.vtable).initMutationEvent)(self as *const _, &*typeArg, canBubbleArg, cancelableArg, relatedNodeArg.map_or(::std::ptr::null(), |x| x as *const _), &*prevValueArg, &*newValueArg, &*attrNameArg, attrChangeArg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



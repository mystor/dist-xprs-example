//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULCommandEvent.idl
//


#[repr(C)]
pub struct nsIDOMXULCommandEvent {
    vtable: *const nsIDOMXULCommandEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULCommandEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x564496b4, 0x1174, 0x48ec,
            [0x92, 0x7d, 0xed, 0xeb, 0x66, 0xb8, 0x67, 0x57])
    }
}

unsafe impl RefCounted for nsIDOMXULCommandEvent {
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
pub trait nsIDOMXULCommandEventCoerce {
    fn coerce_from(v: &nsIDOMXULCommandEvent) -> &Self;
}

impl nsIDOMXULCommandEventCoerce for nsIDOMXULCommandEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMXULCommandEvent) -> &Self {
        v
    }
}

impl nsIDOMXULCommandEvent {
    #[inline]
    pub fn coerce<T: nsIDOMXULCommandEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULCommandEvent {
    type Target = nsIDOMUIEvent;
    #[inline]
    fn deref(&self) -> &nsIDOMUIEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMUIEventCoerce> nsIDOMXULCommandEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULCommandEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULCommandEventVTable {
    pub __base: nsIDOMUIEventVTable,

    /* readonly attribute boolean ctrlKey; */
    pub get_ctrlKey: unsafe extern "C" fn (this: *const nsIDOMXULCommandEvent, aCtrlKey: *mut bool) -> nsresult,

    /* readonly attribute boolean shiftKey; */
    pub get_shiftKey: unsafe extern "C" fn (this: *const nsIDOMXULCommandEvent, aShiftKey: *mut bool) -> nsresult,

    /* readonly attribute boolean altKey; */
    pub get_altKey: unsafe extern "C" fn (this: *const nsIDOMXULCommandEvent, aAltKey: *mut bool) -> nsresult,

    /* readonly attribute boolean metaKey; */
    pub get_metaKey: unsafe extern "C" fn (this: *const nsIDOMXULCommandEvent, aMetaKey: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMEvent sourceEvent; */
    pub get_sourceEvent: unsafe extern "C" fn (this: *const nsIDOMXULCommandEvent, aSourceEvent: *mut *const nsIDOMEvent) -> nsresult,

    /* void initCommandEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in mozIDOMWindow viewArg, in long detailArg, in boolean ctrlKeyArg, in boolean altKeyArg, in boolean shiftKeyArg, in boolean metaKeyArg, in nsIDOMEvent sourceEvent); */
    pub initCommandEvent: unsafe extern "C" fn (this: *const nsIDOMXULCommandEvent, typeArg: *const nsAString, canBubbleArg: bool, cancelableArg: bool, viewArg: *const mozIDOMWindow, detailArg: libc::int32_t, ctrlKeyArg: bool, altKeyArg: bool, shiftKeyArg: bool, metaKeyArg: bool, sourceEvent: *const nsIDOMEvent) -> nsresult,

}


impl nsIDOMXULCommandEvent {
    /* readonly attribute boolean ctrlKey; */
    #[inline]
    pub unsafe fn get_ctrlKey(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_ctrlKey)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean shiftKey; */
    #[inline]
    pub unsafe fn get_shiftKey(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_shiftKey)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean altKey; */
    #[inline]
    pub unsafe fn get_altKey(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_altKey)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean metaKey; */
    #[inline]
    pub unsafe fn get_metaKey(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_metaKey)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMEvent sourceEvent; */
    #[inline]
    pub unsafe fn get_sourceEvent(&self, ) -> Result<Option<RefPtr<nsIDOMEvent>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sourceEvent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void initCommandEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in mozIDOMWindow viewArg, in long detailArg, in boolean ctrlKeyArg, in boolean altKeyArg, in boolean shiftKeyArg, in boolean metaKeyArg, in nsIDOMEvent sourceEvent); */
    #[inline]
    pub unsafe fn initCommandEvent(&self, typeArg: &[u16], canBubbleArg: bool, cancelableArg: bool, viewArg: Option<&mozIDOMWindow>, detailArg: libc::int32_t, ctrlKeyArg: bool, altKeyArg: bool, shiftKeyArg: bool, metaKeyArg: bool, sourceEvent: Option<&nsIDOMEvent>) -> Result<(), nsresult> {
        let typeArg = nsString::from(typeArg);
        match ((*self.vtable).initCommandEvent)(self as *const _, &*typeArg, canBubbleArg, cancelableArg, viewArg.map_or(::std::ptr::null(), |x| x as *const _), detailArg, ctrlKeyArg, altKeyArg, shiftKeyArg, metaKeyArg, sourceEvent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



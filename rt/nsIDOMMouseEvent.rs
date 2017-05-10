//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMMouseEvent.idl
//


pub mod nsIDOMMouseEvent_consts {
    pub const MOZ_SOURCE_UNKNOWN: i64 = 0;
    pub const MOZ_SOURCE_MOUSE: i64 = 1;
    pub const MOZ_SOURCE_PEN: i64 = 2;
    pub const MOZ_SOURCE_ERASER: i64 = 3;
    pub const MOZ_SOURCE_CURSOR: i64 = 4;
    pub const MOZ_SOURCE_TOUCH: i64 = 5;
    pub const MOZ_SOURCE_KEYBOARD: i64 = 6;
}


#[repr(C)]
pub struct nsIDOMMouseEvent {
    vtable: *const nsIDOMMouseEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMMouseEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5bdab8d8, 0x7933, 0x4c5c,
            [0xb6, 0xd1, 0xab, 0x34, 0x48, 0x12, 0x37, 0xf7])
    }
}

unsafe impl RefCounted for nsIDOMMouseEvent {
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
pub trait nsIDOMMouseEventCoerce {
    fn coerce_from(v: &nsIDOMMouseEvent) -> &Self;
}

impl nsIDOMMouseEventCoerce for nsIDOMMouseEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMMouseEvent) -> &Self {
        v
    }
}

impl nsIDOMMouseEvent {
    #[inline]
    pub fn coerce<T: nsIDOMMouseEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMMouseEvent {
    type Target = nsIDOMUIEvent;
    #[inline]
    fn deref(&self) -> &nsIDOMUIEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMUIEventCoerce> nsIDOMMouseEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMMouseEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMMouseEventVTable {
    pub __base: nsIDOMUIEventVTable,

    /* readonly attribute long screenX; */
    pub get_screenX: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aScreenX: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long screenY; */
    pub get_screenY: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aScreenY: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long mozMovementX; */
    pub get_mozMovementX: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aMozMovementX: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long mozMovementY; */
    pub get_mozMovementY: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aMozMovementY: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long clientX; */
    pub get_clientX: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aClientX: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long clientY; */
    pub get_clientY: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aClientY: *mut libc::int32_t) -> nsresult,

    /* readonly attribute boolean ctrlKey; */
    pub get_ctrlKey: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aCtrlKey: *mut bool) -> nsresult,

    /* readonly attribute boolean shiftKey; */
    pub get_shiftKey: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aShiftKey: *mut bool) -> nsresult,

    /* readonly attribute boolean altKey; */
    pub get_altKey: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aAltKey: *mut bool) -> nsresult,

    /* readonly attribute boolean metaKey; */
    pub get_metaKey: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aMetaKey: *mut bool) -> nsresult,

    /* readonly attribute short button; */
    pub get_button: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aButton: *mut libc::int16_t) -> nsresult,

    /* readonly attribute unsigned short buttons; */
    pub get_buttons: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aButtons: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute nsIDOMEventTarget relatedTarget; */
    pub get_relatedTarget: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aRelatedTarget: *mut *const nsIDOMEventTarget) -> nsresult,

    /* void initMouseEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in mozIDOMWindow viewArg, in long detailArg, in long screenXArg, in long screenYArg, in long clientXArg, in long clientYArg, in boolean ctrlKeyArg, in boolean altKeyArg, in boolean shiftKeyArg, in boolean metaKeyArg, in unsigned short buttonArg, in nsIDOMEventTarget relatedTargetArg); */
    pub initMouseEvent: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, typeArg: *const nsAString, canBubbleArg: bool, cancelableArg: bool, viewArg: *const mozIDOMWindow, detailArg: libc::int32_t, screenXArg: libc::int32_t, screenYArg: libc::int32_t, clientXArg: libc::int32_t, clientYArg: libc::int32_t, ctrlKeyArg: bool, altKeyArg: bool, shiftKeyArg: bool, metaKeyArg: bool, buttonArg: libc::uint16_t, relatedTargetArg: *const nsIDOMEventTarget) -> nsresult,

    /* readonly attribute float mozPressure; */
    pub get_mozPressure: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aMozPressure: *mut libc::c_float) -> nsresult,

    /* readonly attribute unsigned short mozInputSource; */
    pub get_mozInputSource: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, aMozInputSource: *mut libc::uint16_t) -> nsresult,

    /* bool getModifierState (in DOMString keyArg); */
    pub getModifierState: unsafe extern "C" fn (this: *const nsIDOMMouseEvent, keyArg: *const nsAString, _retval: *mut bool) -> nsresult,

}


impl nsIDOMMouseEvent {
    /* readonly attribute long screenX; */
    #[inline]
    pub unsafe fn get_screenX(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_screenX)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long screenY; */
    #[inline]
    pub unsafe fn get_screenY(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_screenY)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long mozMovementX; */
    #[inline]
    pub unsafe fn get_mozMovementX(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_mozMovementX)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long mozMovementY; */
    #[inline]
    pub unsafe fn get_mozMovementY(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_mozMovementY)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long clientX; */
    #[inline]
    pub unsafe fn get_clientX(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_clientX)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long clientY; */
    #[inline]
    pub unsafe fn get_clientY(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_clientY)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

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

    /* readonly attribute short button; */
    #[inline]
    pub unsafe fn get_button(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_button)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned short buttons; */
    #[inline]
    pub unsafe fn get_buttons(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_buttons)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMEventTarget relatedTarget; */
    #[inline]
    pub unsafe fn get_relatedTarget(&self, ) -> Result<Option<RefPtr<nsIDOMEventTarget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_relatedTarget)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void initMouseEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in mozIDOMWindow viewArg, in long detailArg, in long screenXArg, in long screenYArg, in long clientXArg, in long clientYArg, in boolean ctrlKeyArg, in boolean altKeyArg, in boolean shiftKeyArg, in boolean metaKeyArg, in unsigned short buttonArg, in nsIDOMEventTarget relatedTargetArg); */
    #[inline]
    pub unsafe fn initMouseEvent(&self, typeArg: &[u16], canBubbleArg: bool, cancelableArg: bool, viewArg: Option<&mozIDOMWindow>, detailArg: libc::int32_t, screenXArg: libc::int32_t, screenYArg: libc::int32_t, clientXArg: libc::int32_t, clientYArg: libc::int32_t, ctrlKeyArg: bool, altKeyArg: bool, shiftKeyArg: bool, metaKeyArg: bool, buttonArg: libc::uint16_t, relatedTargetArg: Option<&nsIDOMEventTarget>) -> Result<(), nsresult> {
        let typeArg = nsString::from(typeArg);
        match ((*self.vtable).initMouseEvent)(self as *const _, &*typeArg, canBubbleArg, cancelableArg, viewArg.map_or(::std::ptr::null(), |x| x as *const _), detailArg, screenXArg, screenYArg, clientXArg, clientYArg, ctrlKeyArg, altKeyArg, shiftKeyArg, metaKeyArg, buttonArg, relatedTargetArg.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute float mozPressure; */
    #[inline]
    pub unsafe fn get_mozPressure(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_mozPressure)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned short mozInputSource; */
    #[inline]
    pub unsafe fn get_mozInputSource(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_mozInputSource)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool getModifierState (in DOMString keyArg); */
    #[inline]
    pub unsafe fn getModifierState(&self, keyArg: &[u16]) -> Result<bool, nsresult> {
        let keyArg = nsString::from(keyArg);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getModifierState)(self as *const _, &*keyArg, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



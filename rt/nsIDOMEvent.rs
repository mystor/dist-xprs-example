//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMEvent.idl
//


pub mod nsIDOMEvent_consts {
    pub const NONE: i64 = 0;
    pub const CAPTURING_PHASE: i64 = 1;
    pub const AT_TARGET: i64 = 2;
    pub const BUBBLING_PHASE: i64 = 3;
    pub const ALT_MASK: i64 = 1;
    pub const CONTROL_MASK: i64 = 2;
    pub const SHIFT_MASK: i64 = 4;
    pub const META_MASK: i64 = 8;
}


#[repr(C)]
pub struct nsIDOMEvent {
    vtable: *const nsIDOMEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf58daacf, 0x4d1a, 0x4002,
            [0x8f, 0xd7, 0x06, 0xb6, 0x14, 0xdf, 0xbc, 0xf6])
    }
}

unsafe impl RefCounted for nsIDOMEvent {
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
pub trait nsIDOMEventCoerce {
    fn coerce_from(v: &nsIDOMEvent) -> &Self;
}

impl nsIDOMEventCoerce for nsIDOMEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMEvent) -> &Self {
        v
    }
}

impl nsIDOMEvent {
    #[inline]
    pub fn coerce<T: nsIDOMEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMEvent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMEventVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMEvent, aType: *mut nsAString) -> nsresult,

    /* readonly attribute nsIDOMEventTarget target; */
    pub get_target: unsafe extern "C" fn (this: *const nsIDOMEvent, aTarget: *mut *const nsIDOMEventTarget) -> nsresult,

    /* readonly attribute nsIDOMEventTarget currentTarget; */
    pub get_currentTarget: unsafe extern "C" fn (this: *const nsIDOMEvent, aCurrentTarget: *mut *const nsIDOMEventTarget) -> nsresult,

    /* readonly attribute unsigned short eventPhase; */
    pub get_eventPhase: unsafe extern "C" fn (this: *const nsIDOMEvent, aEventPhase: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute boolean bubbles; */
    pub get_bubbles: unsafe extern "C" fn (this: *const nsIDOMEvent, aBubbles: *mut bool) -> nsresult,

    /* readonly attribute boolean cancelable; */
    pub get_cancelable: unsafe extern "C" fn (this: *const nsIDOMEvent, aCancelable: *mut bool) -> nsresult,

    /* readonly attribute DOMTimeStamp timeStamp; */
    pub get_timeStamp: unsafe extern "C" fn (this: *const nsIDOMEvent, aTimeStamp: *mut DOMTimeStamp) -> nsresult,

    /* void stopPropagation (); */
    pub stopPropagation: unsafe extern "C" fn (this: *const nsIDOMEvent) -> nsresult,

    /* void preventDefault (); */
    pub preventDefault: unsafe extern "C" fn (this: *const nsIDOMEvent) -> nsresult,

    /* [nostdcall,notxpcom] void initEvent (in DOMString eventTypeArg, in boolean canBubbleArg, in boolean cancelableArg); */
    pub initEvent: unsafe extern "C" fn (this: *const nsIDOMEvent, eventTypeArg: *const nsAString, canBubbleArg: bool, cancelableArg: bool) -> libc::c_void,

    /* readonly attribute boolean defaultPrevented; */
    pub get_defaultPrevented: unsafe extern "C" fn (this: *const nsIDOMEvent, aDefaultPrevented: *mut bool) -> nsresult,

    /* void stopImmediatePropagation (); */
    pub stopImmediatePropagation: unsafe extern "C" fn (this: *const nsIDOMEvent) -> nsresult,

    /* readonly attribute nsIDOMEventTarget originalTarget; */
    pub get_originalTarget: unsafe extern "C" fn (this: *const nsIDOMEvent, aOriginalTarget: *mut *const nsIDOMEventTarget) -> nsresult,

    /* readonly attribute nsIDOMEventTarget explicitOriginalTarget; */
    pub get_explicitOriginalTarget: unsafe extern "C" fn (this: *const nsIDOMEvent, aExplicitOriginalTarget: *mut *const nsIDOMEventTarget) -> nsresult,

    /* boolean getPreventDefault (); */
    pub getPreventDefault: unsafe extern "C" fn (this: *const nsIDOMEvent, _retval: *mut bool) -> nsresult,

    /* readonly attribute boolean isTrusted; */
    pub get_isTrusted: unsafe extern "C" fn (this: *const nsIDOMEvent, aIsTrusted: *mut bool) -> nsresult,

    /* attribute boolean cancelBubble; */
    pub get_cancelBubble: unsafe extern "C" fn (this: *const nsIDOMEvent, aCancelBubble: *mut bool) -> nsresult,
    pub set_cancelBubble: unsafe extern "C" fn (this: *const nsIDOMEvent, aCancelBubble: bool) -> nsresult,

    /* [noscript] void duplicatePrivateData (); */
    pub duplicatePrivateData: unsafe extern "C" fn (this: *const nsIDOMEvent) -> nsresult,

    /* [noscript] void setTarget (in nsIDOMEventTarget aTarget); */
    pub setTarget: unsafe extern "C" fn (this: *const nsIDOMEvent, aTarget: *const nsIDOMEventTarget) -> nsresult,

    /* [notxpcom] boolean IsDispatchStopped (); */
    pub IsDispatchStopped: unsafe extern "C" fn (this: *const nsIDOMEvent) -> bool,

    /* [notxpcom] WidgetEvent WidgetEventPtr (); */
    /// Unable to call function as its signature contains a non-rust type
    pub WidgetEventPtr: *const ::libc::c_void,

    /* [noscript,notxpcom] void SetTrusted (in boolean aTrusted); */
    pub SetTrusted: unsafe extern "C" fn (this: *const nsIDOMEvent, aTrusted: bool) -> libc::c_void,

    /* [notxpcom] void Serialize (in IPCMessagePtr aMsg, in boolean aSerializeInterfaceType); */
    /// Unable to call function as its signature contains a non-rust type
    pub Serialize: *const ::libc::c_void,

    /* [notxpcom] boolean Deserialize (in ConstIPCMessagePtr aMsg, in PickleIterator aIter); */
    /// Unable to call function as its signature contains a non-rust type
    pub Deserialize: *const ::libc::c_void,

    /* [noscript,notxpcom] void SetOwner (in EventTargetPtr aOwner); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetOwner: *const ::libc::c_void,

    /* [notxpcom] DOMEventPtr InternalDOMEvent (); */
    /// Unable to call function as its signature contains a non-rust type
    pub InternalDOMEvent: *const ::libc::c_void,

    /* [noscript] void stopCrossProcessForwarding (); */
    pub stopCrossProcessForwarding: unsafe extern "C" fn (this: *const nsIDOMEvent) -> nsresult,

}


impl nsIDOMEvent {
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

    /* readonly attribute nsIDOMEventTarget target; */
    #[inline]
    pub unsafe fn get_target(&self, ) -> Result<Option<RefPtr<nsIDOMEventTarget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_target)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMEventTarget currentTarget; */
    #[inline]
    pub unsafe fn get_currentTarget(&self, ) -> Result<Option<RefPtr<nsIDOMEventTarget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentTarget)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned short eventPhase; */
    #[inline]
    pub unsafe fn get_eventPhase(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_eventPhase)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean bubbles; */
    #[inline]
    pub unsafe fn get_bubbles(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_bubbles)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean cancelable; */
    #[inline]
    pub unsafe fn get_cancelable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_cancelable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMTimeStamp timeStamp; */
    #[inline]
    pub unsafe fn get_timeStamp(&self, ) -> Result<DOMTimeStamp, nsresult> {
        let mut _retval: DOMTimeStamp = ::std::mem::zeroed();
        match ((*self.vtable).get_timeStamp)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void stopPropagation (); */
    #[inline]
    pub unsafe fn stopPropagation(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stopPropagation)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void preventDefault (); */
    #[inline]
    pub unsafe fn preventDefault(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).preventDefault)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [nostdcall,notxpcom] void initEvent (in DOMString eventTypeArg, in boolean canBubbleArg, in boolean cancelableArg); */
    #[inline]
    pub unsafe fn initEvent(&self, eventTypeArg: &[u16], canBubbleArg: bool, cancelableArg: bool) -> () {
        let eventTypeArg = nsString::from(eventTypeArg);
        let _retval = ((*self.vtable).initEvent)(self as *const _, &*eventTypeArg, canBubbleArg, cancelableArg);
        ()
    }

    /* readonly attribute boolean defaultPrevented; */
    #[inline]
    pub unsafe fn get_defaultPrevented(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultPrevented)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void stopImmediatePropagation (); */
    #[inline]
    pub unsafe fn stopImmediatePropagation(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stopImmediatePropagation)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMEventTarget originalTarget; */
    #[inline]
    pub unsafe fn get_originalTarget(&self, ) -> Result<Option<RefPtr<nsIDOMEventTarget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_originalTarget)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMEventTarget explicitOriginalTarget; */
    #[inline]
    pub unsafe fn get_explicitOriginalTarget(&self, ) -> Result<Option<RefPtr<nsIDOMEventTarget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_explicitOriginalTarget)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean getPreventDefault (); */
    #[inline]
    pub unsafe fn getPreventDefault(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getPreventDefault)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isTrusted; */
    #[inline]
    pub unsafe fn get_isTrusted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isTrusted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean cancelBubble; */
    #[inline]
    pub unsafe fn get_cancelBubble(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_cancelBubble)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_cancelBubble(&self, aCancelBubble: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_cancelBubble)(self as *const _, aCancelBubble) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void duplicatePrivateData (); */
    #[inline]
    pub unsafe fn duplicatePrivateData(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).duplicatePrivateData)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setTarget (in nsIDOMEventTarget aTarget); */
    #[inline]
    pub unsafe fn setTarget(&self, aTarget: Option<&nsIDOMEventTarget>) -> Result<(), nsresult> {

        match ((*self.vtable).setTarget)(self as *const _, aTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [notxpcom] boolean IsDispatchStopped (); */
    #[inline]
    pub unsafe fn IsDispatchStopped(&self, ) -> bool {

        let _retval = ((*self.vtable).IsDispatchStopped)(self as *const _, );
        _retval
    }

    /* [notxpcom] WidgetEvent WidgetEventPtr (); */


    /* [noscript,notxpcom] void SetTrusted (in boolean aTrusted); */
    #[inline]
    pub unsafe fn SetTrusted(&self, aTrusted: bool) -> () {

        let _retval = ((*self.vtable).SetTrusted)(self as *const _, aTrusted);
        ()
    }

    /* [notxpcom] void Serialize (in IPCMessagePtr aMsg, in boolean aSerializeInterfaceType); */


    /* [notxpcom] boolean Deserialize (in ConstIPCMessagePtr aMsg, in PickleIterator aIter); */


    /* [noscript,notxpcom] void SetOwner (in EventTargetPtr aOwner); */


    /* [notxpcom] DOMEventPtr InternalDOMEvent (); */


    /* [noscript] void stopCrossProcessForwarding (); */
    #[inline]
    pub unsafe fn stopCrossProcessForwarding(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stopCrossProcessForwarding)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



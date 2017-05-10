//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMEventTarget.idl
//


#[repr(C)]
pub struct nsIDOMEventTarget {
    vtable: *const nsIDOMEventTargetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMEventTarget {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9a78ac3c, 0x9507, 0x4d00,
            [0xb2, 0xd6, 0x10, 0xb5, 0x08, 0xd2, 0xec, 0x31])
    }
}

unsafe impl RefCounted for nsIDOMEventTarget {
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
pub trait nsIDOMEventTargetCoerce {
    fn coerce_from(v: &nsIDOMEventTarget) -> &Self;
}

impl nsIDOMEventTargetCoerce for nsIDOMEventTarget {
    #[inline]
    fn coerce_from(v: &nsIDOMEventTarget) -> &Self {
        v
    }
}

impl nsIDOMEventTarget {
    #[inline]
    pub fn coerce<T: nsIDOMEventTargetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMEventTarget {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMEventTargetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMEventTarget) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMEventTargetVTable {
    pub __base: nsISupportsVTable,

    /* [optional_argc] void addEventListener (in DOMString type, in nsIDOMEventListener listener, [optional] in boolean useCapture, [optional] in boolean wantsUntrusted); */
    /// Unable to call function as its signature contains a non-rust type
    pub addEventListener: *const ::libc::c_void,

    /* [noscript,optional_argc] void addSystemEventListener (in DOMString type, in nsIDOMEventListener listener, [optional] in boolean aUseCapture, [optional] in boolean aWantsUntrusted); */
    /// Unable to call function as its signature contains a non-rust type
    pub addSystemEventListener: *const ::libc::c_void,

    /* void removeEventListener (in DOMString type, in nsIDOMEventListener listener, [optional] in boolean useCapture); */
    pub removeEventListener: unsafe extern "C" fn (this: *const nsIDOMEventTarget, type_: *const nsAString, listener: *const nsIDOMEventListener, useCapture: bool) -> nsresult,

    /* [noscript] void removeSystemEventListener (in DOMString type, in nsIDOMEventListener listener, [optional] in boolean aUseCapture); */
    pub removeSystemEventListener: unsafe extern "C" fn (this: *const nsIDOMEventTarget, type_: *const nsAString, listener: *const nsIDOMEventListener, aUseCapture: bool) -> nsresult,

    /* boolean dispatchEvent (in nsIDOMEvent evt) raises (DOMException); */
    pub dispatchEvent: unsafe extern "C" fn (this: *const nsIDOMEventTarget, evt: *const nsIDOMEvent, _retval: *mut bool) -> nsresult,

    /* [nostdcall,notxpcom] EventTargetPtr GetTargetForDOMEvent (); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetTargetForDOMEvent: *const ::libc::c_void,

    /* [nostdcall,notxpcom] EventTargetPtr GetTargetForEventTargetChain (); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetTargetForEventTargetChain: *const ::libc::c_void,

    /* [noscript,nostdcall] void GetEventTargetParent (in EventChainPreVisitorRef aVisitor); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetEventTargetParent: *const ::libc::c_void,

    /* [noscript,nostdcall] void WillHandleEvent (in EventChainPostVisitorRef aVisitor); */
    /// Unable to call function as its signature contains a non-rust type
    pub WillHandleEvent: *const ::libc::c_void,

    /* [noscript,nostdcall] void PostHandleEvent (in EventChainPostVisitorRef aVisitor); */
    /// Unable to call function as its signature contains a non-rust type
    pub PostHandleEvent: *const ::libc::c_void,

    /* [noscript,nostdcall] void DispatchDOMEvent (in WidgetEventPtr aEvent, in nsIDOMEvent aDOMEvent, in nsPresContextPtr aPresContext, in nsEventStatusPtr aEventStatus); */
    /// Unable to call function as its signature contains a non-rust type
    pub DispatchDOMEvent: *const ::libc::c_void,

    /* [nostdcall,notxpcom] nsIScriptContext GetContextForEventHandlers (out nsresult aRv); */
    pub GetContextForEventHandlers: unsafe extern "C" fn (this: *const nsIDOMEventTarget, aRv: *mut nsresult) -> *const nsIScriptContext,

}


impl nsIDOMEventTarget {
    /* [optional_argc] void addEventListener (in DOMString type, in nsIDOMEventListener listener, [optional] in boolean useCapture, [optional] in boolean wantsUntrusted); */


    /* [noscript,optional_argc] void addSystemEventListener (in DOMString type, in nsIDOMEventListener listener, [optional] in boolean aUseCapture, [optional] in boolean aWantsUntrusted); */


    /* void removeEventListener (in DOMString type, in nsIDOMEventListener listener, [optional] in boolean useCapture); */
    #[inline]
    pub unsafe fn removeEventListener(&self, type_: &[u16], listener: Option<&nsIDOMEventListener>, useCapture: bool) -> Result<(), nsresult> {
        let type_ = nsString::from(type_);
        match ((*self.vtable).removeEventListener)(self as *const _, &*type_, listener.map_or(::std::ptr::null(), |x| x as *const _), useCapture) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void removeSystemEventListener (in DOMString type, in nsIDOMEventListener listener, [optional] in boolean aUseCapture); */
    #[inline]
    pub unsafe fn removeSystemEventListener(&self, type_: &[u16], listener: Option<&nsIDOMEventListener>, aUseCapture: bool) -> Result<(), nsresult> {
        let type_ = nsString::from(type_);
        match ((*self.vtable).removeSystemEventListener)(self as *const _, &*type_, listener.map_or(::std::ptr::null(), |x| x as *const _), aUseCapture) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean dispatchEvent (in nsIDOMEvent evt) raises (DOMException); */
    #[inline]
    pub unsafe fn dispatchEvent(&self, evt: Option<&nsIDOMEvent>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).dispatchEvent)(self as *const _, evt.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [nostdcall,notxpcom] EventTargetPtr GetTargetForDOMEvent (); */


    /* [nostdcall,notxpcom] EventTargetPtr GetTargetForEventTargetChain (); */


    /* [noscript,nostdcall] void GetEventTargetParent (in EventChainPreVisitorRef aVisitor); */


    /* [noscript,nostdcall] void WillHandleEvent (in EventChainPostVisitorRef aVisitor); */


    /* [noscript,nostdcall] void PostHandleEvent (in EventChainPostVisitorRef aVisitor); */


    /* [noscript,nostdcall] void DispatchDOMEvent (in WidgetEventPtr aEvent, in nsIDOMEvent aDOMEvent, in nsPresContextPtr aPresContext, in nsEventStatusPtr aEventStatus); */


    /* [nostdcall,notxpcom] nsIScriptContext GetContextForEventHandlers (out nsresult aRv); */
    #[inline]
    pub unsafe fn GetContextForEventHandlers(&self, ) -> (nsresult, Option<RefPtr<nsIScriptContext>>) {
        let mut aRv: nsresult = ::std::mem::zeroed();
        let _retval = ((*self.vtable).GetContextForEventHandlers)(self as *const _, &mut aRv as *mut _);
        (aRv, RefPtr::from_raw(_retval))
    }

}



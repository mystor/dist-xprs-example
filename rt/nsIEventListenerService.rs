//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEventListenerService.idl
//


#[repr(C)]
pub struct nsIEventListenerChange {
    vtable: *const nsIEventListenerChangeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEventListenerChange {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x07222b02, 0xda12, 0x4cf4,
            [0xb2, 0xf7, 0x76, 0x1d, 0xa0, 0x07, 0xa8, 0xd8])
    }
}

unsafe impl RefCounted for nsIEventListenerChange {
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
pub trait nsIEventListenerChangeCoerce {
    fn coerce_from(v: &nsIEventListenerChange) -> &Self;
}

impl nsIEventListenerChangeCoerce for nsIEventListenerChange {
    #[inline]
    fn coerce_from(v: &nsIEventListenerChange) -> &Self {
        v
    }
}

impl nsIEventListenerChange {
    #[inline]
    pub fn coerce<T: nsIEventListenerChangeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEventListenerChange {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEventListenerChangeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEventListenerChange) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEventListenerChangeVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMEventTarget target; */
    pub get_target: unsafe extern "C" fn (this: *const nsIEventListenerChange, aTarget: *mut *const nsIDOMEventTarget) -> nsresult,

    /* readonly attribute nsIArray changedListenerNames; */
    pub get_changedListenerNames: unsafe extern "C" fn (this: *const nsIEventListenerChange, aChangedListenerNames: *mut *const nsIArray) -> nsresult,

}


impl nsIEventListenerChange {
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

    /* readonly attribute nsIArray changedListenerNames; */
    #[inline]
    pub unsafe fn get_changedListenerNames(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_changedListenerNames)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIListenerChangeListener {
    vtable: *const nsIListenerChangeListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIListenerChangeListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xaa7c95f6, 0xd3b5, 0x44b3,
            [0x95, 0x97, 0x1d, 0x9f, 0x19, 0xb9, 0xc5, 0xf2])
    }
}

unsafe impl RefCounted for nsIListenerChangeListener {
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
pub trait nsIListenerChangeListenerCoerce {
    fn coerce_from(v: &nsIListenerChangeListener) -> &Self;
}

impl nsIListenerChangeListenerCoerce for nsIListenerChangeListener {
    #[inline]
    fn coerce_from(v: &nsIListenerChangeListener) -> &Self {
        v
    }
}

impl nsIListenerChangeListener {
    #[inline]
    pub fn coerce<T: nsIListenerChangeListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIListenerChangeListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIListenerChangeListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIListenerChangeListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIListenerChangeListenerVTable {
    pub __base: nsISupportsVTable,

    /* void listenersChanged (in nsIArray aEventListenerChanges); */
    pub listenersChanged: unsafe extern "C" fn (this: *const nsIListenerChangeListener, aEventListenerChanges: *const nsIArray) -> nsresult,

}


impl nsIListenerChangeListener {
    /* void listenersChanged (in nsIArray aEventListenerChanges); */
    #[inline]
    pub unsafe fn listenersChanged(&self, aEventListenerChanges: Option<&nsIArray>) -> Result<(), nsresult> {

        match ((*self.vtable).listenersChanged)(self as *const _, aEventListenerChanges.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIEventListenerInfo {
    vtable: *const nsIEventListenerInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEventListenerInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x11ba5fd7, 0x8db2, 0x4b1a,
            [0x9f, 0x67, 0x34, 0x2c, 0xfa, 0x11, 0xaf, 0xad])
    }
}

unsafe impl RefCounted for nsIEventListenerInfo {
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
pub trait nsIEventListenerInfoCoerce {
    fn coerce_from(v: &nsIEventListenerInfo) -> &Self;
}

impl nsIEventListenerInfoCoerce for nsIEventListenerInfo {
    #[inline]
    fn coerce_from(v: &nsIEventListenerInfo) -> &Self {
        v
    }
}

impl nsIEventListenerInfo {
    #[inline]
    pub fn coerce<T: nsIEventListenerInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEventListenerInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEventListenerInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEventListenerInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEventListenerInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIEventListenerInfo, aType: *mut nsAString) -> nsresult,

    /* readonly attribute boolean capturing; */
    pub get_capturing: unsafe extern "C" fn (this: *const nsIEventListenerInfo, aCapturing: *mut bool) -> nsresult,

    /* readonly attribute boolean allowsUntrusted; */
    pub get_allowsUntrusted: unsafe extern "C" fn (this: *const nsIEventListenerInfo, aAllowsUntrusted: *mut bool) -> nsresult,

    /* readonly attribute boolean inSystemEventGroup; */
    pub get_inSystemEventGroup: unsafe extern "C" fn (this: *const nsIEventListenerInfo, aInSystemEventGroup: *mut bool) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval listenerObject; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_listenerObject: *const ::libc::c_void,

    /* AString toSource (); */
    pub toSource: unsafe extern "C" fn (this: *const nsIEventListenerInfo, _retval: *mut nsAString) -> nsresult,

}


impl nsIEventListenerInfo {
    /* readonly attribute AString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean capturing; */
    #[inline]
    pub unsafe fn get_capturing(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_capturing)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean allowsUntrusted; */
    #[inline]
    pub unsafe fn get_allowsUntrusted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowsUntrusted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean inSystemEventGroup; */
    #[inline]
    pub unsafe fn get_inSystemEventGroup(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_inSystemEventGroup)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] readonly attribute jsval listenerObject; */


    /* AString toSource (); */
    #[inline]
    pub unsafe fn toSource(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).toSource)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIEventListenerService {
    vtable: *const nsIEventListenerServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEventListenerService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x77aab5f7, 0x213d, 0x4db4,
            [0x9f, 0x22, 0xe4, 0x6d, 0xfb, 0x77, 0x4f, 0x15])
    }
}

unsafe impl RefCounted for nsIEventListenerService {
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
pub trait nsIEventListenerServiceCoerce {
    fn coerce_from(v: &nsIEventListenerService) -> &Self;
}

impl nsIEventListenerServiceCoerce for nsIEventListenerService {
    #[inline]
    fn coerce_from(v: &nsIEventListenerService) -> &Self {
        v
    }
}

impl nsIEventListenerService {
    #[inline]
    pub fn coerce<T: nsIEventListenerServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEventListenerService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEventListenerServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEventListenerService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEventListenerServiceVTable {
    pub __base: nsISupportsVTable,

    /* void getListenerInfoFor (in nsIDOMEventTarget aEventTarget, [optional] out unsigned long aCount, [array, size_is (aCount), retval] out nsIEventListenerInfo aOutArray); */
    /// Unable to call function as its signature contains a non-rust type
    pub getListenerInfoFor: *const ::libc::c_void,

    /* void getEventTargetChainFor (in nsIDOMEventTarget aEventTarget, in boolean composed, [optional] out unsigned long aCount, [array, size_is (aCount), retval] out nsIDOMEventTarget aOutArray); */
    /// Unable to call function as its signature contains a non-rust type
    pub getEventTargetChainFor: *const ::libc::c_void,

    /* boolean hasListenersFor (in nsIDOMEventTarget aEventTarget, in DOMString aType); */
    pub hasListenersFor: unsafe extern "C" fn (this: *const nsIEventListenerService, aEventTarget: *const nsIDOMEventTarget, aType: *const nsAString, _retval: *mut bool) -> nsresult,

    /* void addSystemEventListener (in nsIDOMEventTarget target, in DOMString type, in nsIDOMEventListener listener, in boolean useCapture); */
    pub addSystemEventListener: unsafe extern "C" fn (this: *const nsIEventListenerService, target: *const nsIDOMEventTarget, type_: *const nsAString, listener: *const nsIDOMEventListener, useCapture: bool) -> nsresult,

    /* void removeSystemEventListener (in nsIDOMEventTarget target, in DOMString type, in nsIDOMEventListener listener, in boolean useCapture); */
    pub removeSystemEventListener: unsafe extern "C" fn (this: *const nsIEventListenerService, target: *const nsIDOMEventTarget, type_: *const nsAString, listener: *const nsIDOMEventListener, useCapture: bool) -> nsresult,

    /* void addListenerForAllEvents (in nsIDOMEventTarget target, in nsIDOMEventListener listener, [optional] in boolean aUseCapture, [optional] in boolean aWantsUntrusted, [optional] in boolean aSystemEventGroup); */
    pub addListenerForAllEvents: unsafe extern "C" fn (this: *const nsIEventListenerService, target: *const nsIDOMEventTarget, listener: *const nsIDOMEventListener, aUseCapture: bool, aWantsUntrusted: bool, aSystemEventGroup: bool) -> nsresult,

    /* void removeListenerForAllEvents (in nsIDOMEventTarget target, in nsIDOMEventListener listener, [optional] in boolean aUseCapture, [optional] in boolean aSystemEventGroup); */
    pub removeListenerForAllEvents: unsafe extern "C" fn (this: *const nsIEventListenerService, target: *const nsIDOMEventTarget, listener: *const nsIDOMEventListener, aUseCapture: bool, aSystemEventGroup: bool) -> nsresult,

    /* void addListenerChangeListener (in nsIListenerChangeListener aListener); */
    pub addListenerChangeListener: unsafe extern "C" fn (this: *const nsIEventListenerService, aListener: *const nsIListenerChangeListener) -> nsresult,

    /* void removeListenerChangeListener (in nsIListenerChangeListener aListener); */
    pub removeListenerChangeListener: unsafe extern "C" fn (this: *const nsIEventListenerService, aListener: *const nsIListenerChangeListener) -> nsresult,

}


impl nsIEventListenerService {
    /* void getListenerInfoFor (in nsIDOMEventTarget aEventTarget, [optional] out unsigned long aCount, [array, size_is (aCount), retval] out nsIEventListenerInfo aOutArray); */


    /* void getEventTargetChainFor (in nsIDOMEventTarget aEventTarget, in boolean composed, [optional] out unsigned long aCount, [array, size_is (aCount), retval] out nsIDOMEventTarget aOutArray); */


    /* boolean hasListenersFor (in nsIDOMEventTarget aEventTarget, in DOMString aType); */
    #[inline]
    pub unsafe fn hasListenersFor(&self, aEventTarget: Option<&nsIDOMEventTarget>, aType: &[u16]) -> Result<bool, nsresult> {
        let aType = nsString::from(aType);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasListenersFor)(self as *const _, aEventTarget.map_or(::std::ptr::null(), |x| x as *const _), &*aType, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addSystemEventListener (in nsIDOMEventTarget target, in DOMString type, in nsIDOMEventListener listener, in boolean useCapture); */
    #[inline]
    pub unsafe fn addSystemEventListener(&self, target: Option<&nsIDOMEventTarget>, type_: &[u16], listener: Option<&nsIDOMEventListener>, useCapture: bool) -> Result<(), nsresult> {
        let type_ = nsString::from(type_);
        match ((*self.vtable).addSystemEventListener)(self as *const _, target.map_or(::std::ptr::null(), |x| x as *const _), &*type_, listener.map_or(::std::ptr::null(), |x| x as *const _), useCapture) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeSystemEventListener (in nsIDOMEventTarget target, in DOMString type, in nsIDOMEventListener listener, in boolean useCapture); */
    #[inline]
    pub unsafe fn removeSystemEventListener(&self, target: Option<&nsIDOMEventTarget>, type_: &[u16], listener: Option<&nsIDOMEventListener>, useCapture: bool) -> Result<(), nsresult> {
        let type_ = nsString::from(type_);
        match ((*self.vtable).removeSystemEventListener)(self as *const _, target.map_or(::std::ptr::null(), |x| x as *const _), &*type_, listener.map_or(::std::ptr::null(), |x| x as *const _), useCapture) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addListenerForAllEvents (in nsIDOMEventTarget target, in nsIDOMEventListener listener, [optional] in boolean aUseCapture, [optional] in boolean aWantsUntrusted, [optional] in boolean aSystemEventGroup); */
    #[inline]
    pub unsafe fn addListenerForAllEvents(&self, target: Option<&nsIDOMEventTarget>, listener: Option<&nsIDOMEventListener>, aUseCapture: bool, aWantsUntrusted: bool, aSystemEventGroup: bool) -> Result<(), nsresult> {

        match ((*self.vtable).addListenerForAllEvents)(self as *const _, target.map_or(::std::ptr::null(), |x| x as *const _), listener.map_or(::std::ptr::null(), |x| x as *const _), aUseCapture, aWantsUntrusted, aSystemEventGroup) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeListenerForAllEvents (in nsIDOMEventTarget target, in nsIDOMEventListener listener, [optional] in boolean aUseCapture, [optional] in boolean aSystemEventGroup); */
    #[inline]
    pub unsafe fn removeListenerForAllEvents(&self, target: Option<&nsIDOMEventTarget>, listener: Option<&nsIDOMEventListener>, aUseCapture: bool, aSystemEventGroup: bool) -> Result<(), nsresult> {

        match ((*self.vtable).removeListenerForAllEvents)(self as *const _, target.map_or(::std::ptr::null(), |x| x as *const _), listener.map_or(::std::ptr::null(), |x| x as *const _), aUseCapture, aSystemEventGroup) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addListenerChangeListener (in nsIListenerChangeListener aListener); */
    #[inline]
    pub unsafe fn addListenerChangeListener(&self, aListener: Option<&nsIListenerChangeListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addListenerChangeListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeListenerChangeListener (in nsIListenerChangeListener aListener); */
    #[inline]
    pub unsafe fn removeListenerChangeListener(&self, aListener: Option<&nsIListenerChangeListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeListenerChangeListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



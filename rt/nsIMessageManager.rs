//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMessageManager.idl
//


#[repr(C)]
pub struct nsIMessageListener {
    vtable: *const nsIMessageListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMessageListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2b44eb57, 0xa9c6, 0x4773,
            [0x9a, 0x1e, 0xfe, 0x08, 0x18, 0x73, 0x9a, 0x4c])
    }
}

unsafe impl RefCounted for nsIMessageListener {
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
pub trait nsIMessageListenerCoerce {
    fn coerce_from(v: &nsIMessageListener) -> &Self;
}

impl nsIMessageListenerCoerce for nsIMessageListener {
    #[inline]
    fn coerce_from(v: &nsIMessageListener) -> &Self {
        v
    }
}

impl nsIMessageListener {
    #[inline]
    pub fn coerce<T: nsIMessageListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMessageListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMessageListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMessageListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMessageListenerVTable {
    pub __base: nsISupportsVTable,

    /* void receiveMessage (); */
    pub receiveMessage: unsafe extern "C" fn (this: *const nsIMessageListener) -> nsresult,

}


impl nsIMessageListener {
    /* void receiveMessage (); */
    #[inline]
    pub unsafe fn receiveMessage(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).receiveMessage)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIMessageListenerManager {
    vtable: *const nsIMessageListenerManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMessageListenerManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb949bfec, 0xbb7d, 0x47bc,
            [0xb3, 0x87, 0xac, 0x6a, 0x9b, 0x65, 0x50, 0x72])
    }
}

unsafe impl RefCounted for nsIMessageListenerManager {
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
pub trait nsIMessageListenerManagerCoerce {
    fn coerce_from(v: &nsIMessageListenerManager) -> &Self;
}

impl nsIMessageListenerManagerCoerce for nsIMessageListenerManager {
    #[inline]
    fn coerce_from(v: &nsIMessageListenerManager) -> &Self {
        v
    }
}

impl nsIMessageListenerManager {
    #[inline]
    pub fn coerce<T: nsIMessageListenerManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMessageListenerManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMessageListenerManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMessageListenerManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMessageListenerManagerVTable {
    pub __base: nsISupportsVTable,

    /* void addMessageListener (in AString messageName, in nsIMessageListener listener, [optional] in boolean listenWhenClosed); */
    pub addMessageListener: unsafe extern "C" fn (this: *const nsIMessageListenerManager, messageName: *const nsAString, listener: *const nsIMessageListener, listenWhenClosed: bool) -> nsresult,

    /* void removeMessageListener (in AString messageName, in nsIMessageListener listener); */
    pub removeMessageListener: unsafe extern "C" fn (this: *const nsIMessageListenerManager, messageName: *const nsAString, listener: *const nsIMessageListener) -> nsresult,

    /* void addWeakMessageListener (in AString messageName, in nsIMessageListener listener); */
    pub addWeakMessageListener: unsafe extern "C" fn (this: *const nsIMessageListenerManager, messageName: *const nsAString, listener: *const nsIMessageListener) -> nsresult,

    /* void removeWeakMessageListener (in AString messageName, in nsIMessageListener listener); */
    pub removeWeakMessageListener: unsafe extern "C" fn (this: *const nsIMessageListenerManager, messageName: *const nsAString, listener: *const nsIMessageListener) -> nsresult,

    /* [notxpcom] boolean markForCC (); */
    pub markForCC: unsafe extern "C" fn (this: *const nsIMessageListenerManager) -> bool,

}


impl nsIMessageListenerManager {
    /* void addMessageListener (in AString messageName, in nsIMessageListener listener, [optional] in boolean listenWhenClosed); */
    #[inline]
    pub unsafe fn addMessageListener(&self, messageName: &[u16], listener: Option<&nsIMessageListener>, listenWhenClosed: bool) -> Result<(), nsresult> {
        let messageName = nsString::from(messageName);
        match ((*self.vtable).addMessageListener)(self as *const _, &*messageName, listener.map_or(::std::ptr::null(), |x| x as *const _), listenWhenClosed) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeMessageListener (in AString messageName, in nsIMessageListener listener); */
    #[inline]
    pub unsafe fn removeMessageListener(&self, messageName: &[u16], listener: Option<&nsIMessageListener>) -> Result<(), nsresult> {
        let messageName = nsString::from(messageName);
        match ((*self.vtable).removeMessageListener)(self as *const _, &*messageName, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addWeakMessageListener (in AString messageName, in nsIMessageListener listener); */
    #[inline]
    pub unsafe fn addWeakMessageListener(&self, messageName: &[u16], listener: Option<&nsIMessageListener>) -> Result<(), nsresult> {
        let messageName = nsString::from(messageName);
        match ((*self.vtable).addWeakMessageListener)(self as *const _, &*messageName, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeWeakMessageListener (in AString messageName, in nsIMessageListener listener); */
    #[inline]
    pub unsafe fn removeWeakMessageListener(&self, messageName: &[u16], listener: Option<&nsIMessageListener>) -> Result<(), nsresult> {
        let messageName = nsString::from(messageName);
        match ((*self.vtable).removeWeakMessageListener)(self as *const _, &*messageName, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [notxpcom] boolean markForCC (); */
    #[inline]
    pub unsafe fn markForCC(&self, ) -> bool {

        let _retval = ((*self.vtable).markForCC)(self as *const _, );
        _retval
    }

}


#[repr(C)]
pub struct nsIMessageSender {
    vtable: *const nsIMessageSenderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMessageSender {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbb5d79e4, 0xe73c, 0x45e7,
            [0x96, 0x51, 0x4d, 0x71, 0x8f, 0x4b, 0x99, 0x4c])
    }
}

unsafe impl RefCounted for nsIMessageSender {
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
pub trait nsIMessageSenderCoerce {
    fn coerce_from(v: &nsIMessageSender) -> &Self;
}

impl nsIMessageSenderCoerce for nsIMessageSender {
    #[inline]
    fn coerce_from(v: &nsIMessageSender) -> &Self {
        v
    }
}

impl nsIMessageSender {
    #[inline]
    pub fn coerce<T: nsIMessageSenderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMessageSender {
    type Target = nsIMessageListenerManager;
    #[inline]
    fn deref(&self) -> &nsIMessageListenerManager {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIMessageListenerManagerCoerce> nsIMessageSenderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMessageSender) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMessageSenderVTable {
    pub __base: nsIMessageListenerManagerVTable,

    /* [implicit_jscontext,optional_argc] void sendAsyncMessage ([optional] in AString messageName, [optional] in jsval obj, [optional] in jsval objects, [optional] in nsIPrincipal principal, [optional] in jsval transfers); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendAsyncMessage: *const ::libc::c_void,

    /* readonly attribute nsIMessageSender processMessageManager; */
    pub get_processMessageManager: unsafe extern "C" fn (this: *const nsIMessageSender, aProcessMessageManager: *mut *const nsIMessageSender) -> nsresult,

}


impl nsIMessageSender {
    /* [implicit_jscontext,optional_argc] void sendAsyncMessage ([optional] in AString messageName, [optional] in jsval obj, [optional] in jsval objects, [optional] in nsIPrincipal principal, [optional] in jsval transfers); */


    /* readonly attribute nsIMessageSender processMessageManager; */
    #[inline]
    pub unsafe fn get_processMessageManager(&self, ) -> Result<Option<RefPtr<nsIMessageSender>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_processMessageManager)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIMessageBroadcaster {
    vtable: *const nsIMessageBroadcasterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMessageBroadcaster {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4d7d62ad, 0x4725, 0x4f39,
            [0x86, 0xcf, 0x8f, 0xb2, 0x2b, 0xf9, 0xc1, 0xd8])
    }
}

unsafe impl RefCounted for nsIMessageBroadcaster {
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
pub trait nsIMessageBroadcasterCoerce {
    fn coerce_from(v: &nsIMessageBroadcaster) -> &Self;
}

impl nsIMessageBroadcasterCoerce for nsIMessageBroadcaster {
    #[inline]
    fn coerce_from(v: &nsIMessageBroadcaster) -> &Self {
        v
    }
}

impl nsIMessageBroadcaster {
    #[inline]
    pub fn coerce<T: nsIMessageBroadcasterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMessageBroadcaster {
    type Target = nsIMessageListenerManager;
    #[inline]
    fn deref(&self) -> &nsIMessageListenerManager {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIMessageListenerManagerCoerce> nsIMessageBroadcasterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMessageBroadcaster) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMessageBroadcasterVTable {
    pub __base: nsIMessageListenerManagerVTable,

    /* [implicit_jscontext,optional_argc] void broadcastAsyncMessage ([optional] in AString messageName, [optional] in jsval obj, [optional] in jsval objects); */
    /// Unable to call function as its signature contains a non-rust type
    pub broadcastAsyncMessage: *const ::libc::c_void,

    /* readonly attribute unsigned long childCount; */
    pub get_childCount: unsafe extern "C" fn (this: *const nsIMessageBroadcaster, aChildCount: *mut libc::uint32_t) -> nsresult,

    /* nsIMessageListenerManager getChildAt (in unsigned long aIndex); */
    pub getChildAt: unsafe extern "C" fn (this: *const nsIMessageBroadcaster, aIndex: libc::uint32_t, _retval: *mut *const nsIMessageListenerManager) -> nsresult,

    /* void releaseCachedProcesses (); */
    pub releaseCachedProcesses: unsafe extern "C" fn (this: *const nsIMessageBroadcaster) -> nsresult,

}


impl nsIMessageBroadcaster {
    /* [implicit_jscontext,optional_argc] void broadcastAsyncMessage ([optional] in AString messageName, [optional] in jsval obj, [optional] in jsval objects); */


    /* readonly attribute unsigned long childCount; */
    #[inline]
    pub unsafe fn get_childCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_childCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIMessageListenerManager getChildAt (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn getChildAt(&self, aIndex: libc::uint32_t) -> Result<Option<RefPtr<nsIMessageListenerManager>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChildAt)(self as *const _, aIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void releaseCachedProcesses (); */
    #[inline]
    pub unsafe fn releaseCachedProcesses(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).releaseCachedProcesses)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsISyncMessageSender {
    vtable: *const nsISyncMessageSenderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISyncMessageSender {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0e602c9e, 0x1977, 0x422a,
            [0xa8, 0xe4, 0xfe, 0x0d, 0x4a, 0x4f, 0x78, 0xd0])
    }
}

unsafe impl RefCounted for nsISyncMessageSender {
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
pub trait nsISyncMessageSenderCoerce {
    fn coerce_from(v: &nsISyncMessageSender) -> &Self;
}

impl nsISyncMessageSenderCoerce for nsISyncMessageSender {
    #[inline]
    fn coerce_from(v: &nsISyncMessageSender) -> &Self {
        v
    }
}

impl nsISyncMessageSender {
    #[inline]
    pub fn coerce<T: nsISyncMessageSenderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISyncMessageSender {
    type Target = nsIMessageSender;
    #[inline]
    fn deref(&self) -> &nsIMessageSender {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIMessageSenderCoerce> nsISyncMessageSenderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISyncMessageSender) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISyncMessageSenderVTable {
    pub __base: nsIMessageSenderVTable,

    /* [implicit_jscontext,optional_argc] jsval sendSyncMessage ([optional] in AString messageName, [optional] in jsval obj, [optional] in jsval objects, [optional] in nsIPrincipal principal); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendSyncMessage: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] jsval sendRpcMessage ([optional] in AString messageName, [optional] in jsval obj, [optional] in jsval objects, [optional] in nsIPrincipal principal); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendRpcMessage: *const ::libc::c_void,

}


impl nsISyncMessageSender {
    /* [implicit_jscontext,optional_argc] jsval sendSyncMessage ([optional] in AString messageName, [optional] in jsval obj, [optional] in jsval objects, [optional] in nsIPrincipal principal); */


    /* [implicit_jscontext,optional_argc] jsval sendRpcMessage ([optional] in AString messageName, [optional] in jsval obj, [optional] in jsval objects, [optional] in nsIPrincipal principal); */


}


#[repr(C)]
pub struct nsIMessageManagerGlobal {
    vtable: *const nsIMessageManagerGlobalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMessageManagerGlobal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x13f3555f, 0x769e, 0x44ea,
            [0xb6, 0x07, 0x52, 0x39, 0x23, 0x0c, 0x31, 0x62])
    }
}

unsafe impl RefCounted for nsIMessageManagerGlobal {
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
pub trait nsIMessageManagerGlobalCoerce {
    fn coerce_from(v: &nsIMessageManagerGlobal) -> &Self;
}

impl nsIMessageManagerGlobalCoerce for nsIMessageManagerGlobal {
    #[inline]
    fn coerce_from(v: &nsIMessageManagerGlobal) -> &Self {
        v
    }
}

impl nsIMessageManagerGlobal {
    #[inline]
    pub fn coerce<T: nsIMessageManagerGlobalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMessageManagerGlobal {
    type Target = nsISyncMessageSender;
    #[inline]
    fn deref(&self) -> &nsISyncMessageSender {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISyncMessageSenderCoerce> nsIMessageManagerGlobalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMessageManagerGlobal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMessageManagerGlobalVTable {
    pub __base: nsISyncMessageSenderVTable,

    /* void dump (in DOMString aStr); */
    pub dump: unsafe extern "C" fn (this: *const nsIMessageManagerGlobal, aStr: *const nsAString) -> nsresult,

    /* void privateNoteIntentionalCrash (); */
    pub privateNoteIntentionalCrash: unsafe extern "C" fn (this: *const nsIMessageManagerGlobal) -> nsresult,

    /* DOMString atob (in DOMString aAsciiString); */
    pub atob: unsafe extern "C" fn (this: *const nsIMessageManagerGlobal, aAsciiString: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* DOMString btoa (in DOMString aBase64Data); */
    pub btoa: unsafe extern "C" fn (this: *const nsIMessageManagerGlobal, aBase64Data: *const nsAString, _retval: *mut nsAString) -> nsresult,

}


impl nsIMessageManagerGlobal {
    /* void dump (in DOMString aStr); */
    #[inline]
    pub unsafe fn dump(&self, aStr: &[u16]) -> Result<(), nsresult> {
        let aStr = nsString::from(aStr);
        match ((*self.vtable).dump)(self as *const _, &*aStr) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void privateNoteIntentionalCrash (); */
    #[inline]
    pub unsafe fn privateNoteIntentionalCrash(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).privateNoteIntentionalCrash)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* DOMString atob (in DOMString aAsciiString); */
    #[inline]
    pub unsafe fn atob(&self, aAsciiString: &[u16]) -> Result<nsString, nsresult> {
        let aAsciiString = nsString::from(aAsciiString);
        let mut _retval = nsString::new();
        match ((*self.vtable).atob)(self as *const _, &*aAsciiString, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* DOMString btoa (in DOMString aBase64Data); */
    #[inline]
    pub unsafe fn btoa(&self, aBase64Data: &[u16]) -> Result<nsString, nsresult> {
        let aBase64Data = nsString::from(aBase64Data);
        let mut _retval = nsString::new();
        match ((*self.vtable).btoa)(self as *const _, &*aBase64Data, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIContentFrameMessageManager {
    vtable: *const nsIContentFrameMessageManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentFrameMessageManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x694e367c, 0xaa25, 0x4446,
            [0x84, 0x99, 0x2c, 0x52, 0x7c, 0x4b, 0xd8, 0x38])
    }
}

unsafe impl RefCounted for nsIContentFrameMessageManager {
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
pub trait nsIContentFrameMessageManagerCoerce {
    fn coerce_from(v: &nsIContentFrameMessageManager) -> &Self;
}

impl nsIContentFrameMessageManagerCoerce for nsIContentFrameMessageManager {
    #[inline]
    fn coerce_from(v: &nsIContentFrameMessageManager) -> &Self {
        v
    }
}

impl nsIContentFrameMessageManager {
    #[inline]
    pub fn coerce<T: nsIContentFrameMessageManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentFrameMessageManager {
    type Target = nsIMessageManagerGlobal;
    #[inline]
    fn deref(&self) -> &nsIMessageManagerGlobal {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIMessageManagerGlobalCoerce> nsIContentFrameMessageManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentFrameMessageManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentFrameMessageManagerVTable {
    pub __base: nsIMessageManagerGlobalVTable,

    /* readonly attribute mozIDOMWindowProxy content; */
    pub get_content: unsafe extern "C" fn (this: *const nsIContentFrameMessageManager, aContent: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* readonly attribute nsIDocShell docShell; */
    pub get_docShell: unsafe extern "C" fn (this: *const nsIContentFrameMessageManager, aDocShell: *mut *const nsIDocShell) -> nsresult,

}


impl nsIContentFrameMessageManager {
    /* readonly attribute mozIDOMWindowProxy content; */
    #[inline]
    pub unsafe fn get_content(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_content)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDocShell docShell; */
    #[inline]
    pub unsafe fn get_docShell(&self, ) -> Result<Option<RefPtr<nsIDocShell>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_docShell)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIInProcessContentFrameMessageManager {
    vtable: *const nsIInProcessContentFrameMessageManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInProcessContentFrameMessageManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb39a3324, 0xb574, 0x4f85,
            [0x8c, 0xdb, 0x27, 0x4d, 0x04, 0xf8, 0x07, 0xef])
    }
}

unsafe impl RefCounted for nsIInProcessContentFrameMessageManager {
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
pub trait nsIInProcessContentFrameMessageManagerCoerce {
    fn coerce_from(v: &nsIInProcessContentFrameMessageManager) -> &Self;
}

impl nsIInProcessContentFrameMessageManagerCoerce for nsIInProcessContentFrameMessageManager {
    #[inline]
    fn coerce_from(v: &nsIInProcessContentFrameMessageManager) -> &Self {
        v
    }
}

impl nsIInProcessContentFrameMessageManager {
    #[inline]
    pub fn coerce<T: nsIInProcessContentFrameMessageManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInProcessContentFrameMessageManager {
    type Target = nsIContentFrameMessageManager;
    #[inline]
    fn deref(&self) -> &nsIContentFrameMessageManager {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIContentFrameMessageManagerCoerce> nsIInProcessContentFrameMessageManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInProcessContentFrameMessageManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInProcessContentFrameMessageManagerVTable {
    pub __base: nsIContentFrameMessageManagerVTable,

    /* [notxpcom] nsIContent getOwnerContent (); */
    pub getOwnerContent: unsafe extern "C" fn (this: *const nsIInProcessContentFrameMessageManager) -> *const nsIContent,

    /* [notxpcom] void cacheFrameLoader (in nsIFrameLoader aFrameLoader); */
    pub cacheFrameLoader: unsafe extern "C" fn (this: *const nsIInProcessContentFrameMessageManager, aFrameLoader: *const nsIFrameLoader) -> libc::c_void,

}


impl nsIInProcessContentFrameMessageManager {
    /* [notxpcom] nsIContent getOwnerContent (); */
    #[inline]
    pub unsafe fn getOwnerContent(&self, ) -> Option<RefPtr<nsIContent>> {

        let _retval = ((*self.vtable).getOwnerContent)(self as *const _, );
        RefPtr::from_raw(_retval)
    }

    /* [notxpcom] void cacheFrameLoader (in nsIFrameLoader aFrameLoader); */
    #[inline]
    pub unsafe fn cacheFrameLoader(&self, aFrameLoader: Option<&nsIFrameLoader>) -> () {

        let _retval = ((*self.vtable).cacheFrameLoader)(self as *const _, aFrameLoader.map_or(::std::ptr::null(), |x| x as *const _));
        ()
    }

}


#[repr(C)]
pub struct nsIContentProcessMessageManager {
    vtable: *const nsIContentProcessMessageManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentProcessMessageManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6d12e467, 0x2446, 0x46db,
            [0x99, 0x65, 0xe4, 0xe9, 0x3c, 0xb8, 0x7c, 0xa5])
    }
}

unsafe impl RefCounted for nsIContentProcessMessageManager {
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
pub trait nsIContentProcessMessageManagerCoerce {
    fn coerce_from(v: &nsIContentProcessMessageManager) -> &Self;
}

impl nsIContentProcessMessageManagerCoerce for nsIContentProcessMessageManager {
    #[inline]
    fn coerce_from(v: &nsIContentProcessMessageManager) -> &Self {
        v
    }
}

impl nsIContentProcessMessageManager {
    #[inline]
    pub fn coerce<T: nsIContentProcessMessageManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentProcessMessageManager {
    type Target = nsIMessageManagerGlobal;
    #[inline]
    fn deref(&self) -> &nsIMessageManagerGlobal {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIMessageManagerGlobalCoerce> nsIContentProcessMessageManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentProcessMessageManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentProcessMessageManagerVTable {
    pub __base: nsIMessageManagerGlobalVTable,

    /* [implicit_jscontext] readonly attribute jsval initialProcessData; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_initialProcessData: *const ::libc::c_void,

}


impl nsIContentProcessMessageManager {
    /* [implicit_jscontext] readonly attribute jsval initialProcessData; */


}


#[repr(C)]
pub struct nsIFrameScriptLoader {
    vtable: *const nsIFrameScriptLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFrameScriptLoader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbf61446b, 0xba24, 0x4b1d,
            [0x88, 0xc7, 0x4f, 0x94, 0x72, 0x4b, 0x9c, 0xe1])
    }
}

unsafe impl RefCounted for nsIFrameScriptLoader {
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
pub trait nsIFrameScriptLoaderCoerce {
    fn coerce_from(v: &nsIFrameScriptLoader) -> &Self;
}

impl nsIFrameScriptLoaderCoerce for nsIFrameScriptLoader {
    #[inline]
    fn coerce_from(v: &nsIFrameScriptLoader) -> &Self {
        v
    }
}

impl nsIFrameScriptLoader {
    #[inline]
    pub fn coerce<T: nsIFrameScriptLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFrameScriptLoader {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFrameScriptLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFrameScriptLoader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFrameScriptLoaderVTable {
    pub __base: nsISupportsVTable,

    /* void loadFrameScript (in AString aURL, in boolean aAllowDelayedLoad, [optional] in boolean aRunInGlobalScope); */
    pub loadFrameScript: unsafe extern "C" fn (this: *const nsIFrameScriptLoader, aURL: *const nsAString, aAllowDelayedLoad: bool, aRunInGlobalScope: bool) -> nsresult,

    /* void removeDelayedFrameScript (in AString aURL); */
    pub removeDelayedFrameScript: unsafe extern "C" fn (this: *const nsIFrameScriptLoader, aURL: *const nsAString) -> nsresult,

    /* [implicit_jscontext] jsval getDelayedFrameScripts (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getDelayedFrameScripts: *const ::libc::c_void,

}


impl nsIFrameScriptLoader {
    /* void loadFrameScript (in AString aURL, in boolean aAllowDelayedLoad, [optional] in boolean aRunInGlobalScope); */
    #[inline]
    pub unsafe fn loadFrameScript(&self, aURL: &[u16], aAllowDelayedLoad: bool, aRunInGlobalScope: bool) -> Result<(), nsresult> {
        let aURL = nsString::from(aURL);
        match ((*self.vtable).loadFrameScript)(self as *const _, &*aURL, aAllowDelayedLoad, aRunInGlobalScope) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeDelayedFrameScript (in AString aURL); */
    #[inline]
    pub unsafe fn removeDelayedFrameScript(&self, aURL: &[u16]) -> Result<(), nsresult> {
        let aURL = nsString::from(aURL);
        match ((*self.vtable).removeDelayedFrameScript)(self as *const _, &*aURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] jsval getDelayedFrameScripts (); */


}


#[repr(C)]
pub struct nsIProcessScriptLoader {
    vtable: *const nsIProcessScriptLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProcessScriptLoader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7e1e1a20, 0xb24f, 0x11e4,
            [0xab, 0x27, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIProcessScriptLoader {
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
pub trait nsIProcessScriptLoaderCoerce {
    fn coerce_from(v: &nsIProcessScriptLoader) -> &Self;
}

impl nsIProcessScriptLoaderCoerce for nsIProcessScriptLoader {
    #[inline]
    fn coerce_from(v: &nsIProcessScriptLoader) -> &Self {
        v
    }
}

impl nsIProcessScriptLoader {
    #[inline]
    pub fn coerce<T: nsIProcessScriptLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProcessScriptLoader {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProcessScriptLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProcessScriptLoader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProcessScriptLoaderVTable {
    pub __base: nsISupportsVTable,

    /* void loadProcessScript (in AString aURL, in boolean aAllowDelayedLoad); */
    pub loadProcessScript: unsafe extern "C" fn (this: *const nsIProcessScriptLoader, aURL: *const nsAString, aAllowDelayedLoad: bool) -> nsresult,

    /* void removeDelayedProcessScript (in AString aURL); */
    pub removeDelayedProcessScript: unsafe extern "C" fn (this: *const nsIProcessScriptLoader, aURL: *const nsAString) -> nsresult,

    /* [implicit_jscontext] jsval getDelayedProcessScripts (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getDelayedProcessScripts: *const ::libc::c_void,

}


impl nsIProcessScriptLoader {
    /* void loadProcessScript (in AString aURL, in boolean aAllowDelayedLoad); */
    #[inline]
    pub unsafe fn loadProcessScript(&self, aURL: &[u16], aAllowDelayedLoad: bool) -> Result<(), nsresult> {
        let aURL = nsString::from(aURL);
        match ((*self.vtable).loadProcessScript)(self as *const _, &*aURL, aAllowDelayedLoad) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeDelayedProcessScript (in AString aURL); */
    #[inline]
    pub unsafe fn removeDelayedProcessScript(&self, aURL: &[u16]) -> Result<(), nsresult> {
        let aURL = nsString::from(aURL);
        match ((*self.vtable).removeDelayedProcessScript)(self as *const _, &*aURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] jsval getDelayedProcessScripts (); */


}


#[repr(C)]
pub struct nsIGlobalProcessScriptLoader {
    vtable: *const nsIGlobalProcessScriptLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGlobalProcessScriptLoader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5b390753, 0xabb3, 0x49b0,
            [0xae, 0x3b, 0xb8, 0x03, 0xda, 0xb5, 0x81, 0x44])
    }
}

unsafe impl RefCounted for nsIGlobalProcessScriptLoader {
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
pub trait nsIGlobalProcessScriptLoaderCoerce {
    fn coerce_from(v: &nsIGlobalProcessScriptLoader) -> &Self;
}

impl nsIGlobalProcessScriptLoaderCoerce for nsIGlobalProcessScriptLoader {
    #[inline]
    fn coerce_from(v: &nsIGlobalProcessScriptLoader) -> &Self {
        v
    }
}

impl nsIGlobalProcessScriptLoader {
    #[inline]
    pub fn coerce<T: nsIGlobalProcessScriptLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGlobalProcessScriptLoader {
    type Target = nsIProcessScriptLoader;
    #[inline]
    fn deref(&self) -> &nsIProcessScriptLoader {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIProcessScriptLoaderCoerce> nsIGlobalProcessScriptLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGlobalProcessScriptLoader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGlobalProcessScriptLoaderVTable {
    pub __base: nsIProcessScriptLoaderVTable,

    /* [implicit_jscontext] readonly attribute jsval initialProcessData; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_initialProcessData: *const ::libc::c_void,

}


impl nsIGlobalProcessScriptLoader {
    /* [implicit_jscontext] readonly attribute jsval initialProcessData; */


}



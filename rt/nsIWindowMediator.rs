//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowMediator.idl
//


pub mod nsIWindowMediator_consts {
    pub const zLevelTop: i64 = 1;
    pub const zLevelBottom: i64 = 2;
    pub const zLevelBelow: i64 = 3;
}


#[repr(C)]
pub struct nsIWindowMediator {
    vtable: *const nsIWindowMediatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWindowMediator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdf0da056, 0x357d, 0x427f,
            [0xba, 0xfd, 0xe6, 0xcb, 0xf1, 0x9c, 0x93, 0x81])
    }
}

unsafe impl RefCounted for nsIWindowMediator {
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
pub trait nsIWindowMediatorCoerce {
    fn coerce_from(v: &nsIWindowMediator) -> &Self;
}

impl nsIWindowMediatorCoerce for nsIWindowMediator {
    #[inline]
    fn coerce_from(v: &nsIWindowMediator) -> &Self {
        v
    }
}

impl nsIWindowMediator {
    #[inline]
    pub fn coerce<T: nsIWindowMediatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWindowMediator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWindowMediatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowMediator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWindowMediatorVTable {
    pub __base: nsISupportsVTable,

    /* nsISimpleEnumerator getEnumerator (in wstring aWindowType); */
    pub getEnumerator: unsafe extern "C" fn (this: *const nsIWindowMediator, aWindowType: *const libc::int16_t, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* nsISimpleEnumerator getXULWindowEnumerator (in wstring aWindowType); */
    pub getXULWindowEnumerator: unsafe extern "C" fn (this: *const nsIWindowMediator, aWindowType: *const libc::int16_t, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* nsISimpleEnumerator getZOrderDOMWindowEnumerator (in wstring aWindowType, in boolean aFrontToBack); */
    pub getZOrderDOMWindowEnumerator: unsafe extern "C" fn (this: *const nsIWindowMediator, aWindowType: *const libc::int16_t, aFrontToBack: bool, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* nsISimpleEnumerator getZOrderXULWindowEnumerator (in wstring aWindowType, in boolean aFrontToBack); */
    pub getZOrderXULWindowEnumerator: unsafe extern "C" fn (this: *const nsIWindowMediator, aWindowType: *const libc::int16_t, aFrontToBack: bool, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* mozIDOMWindowProxy getMostRecentWindow (in wstring aWindowType); */
    pub getMostRecentWindow: unsafe extern "C" fn (this: *const nsIWindowMediator, aWindowType: *const libc::int16_t, _retval: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* mozIDOMWindowProxy getOuterWindowWithId (in unsigned long long aOuterWindowID); */
    pub getOuterWindowWithId: unsafe extern "C" fn (this: *const nsIWindowMediator, aOuterWindowID: libc::uint64_t, _retval: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* mozIDOMWindow getCurrentInnerWindowWithId (in unsigned long long aInnerWindowID); */
    pub getCurrentInnerWindowWithId: unsafe extern "C" fn (this: *const nsIWindowMediator, aInnerWindowID: libc::uint64_t, _retval: *mut *const mozIDOMWindow) -> nsresult,

    /* [noscript] void registerWindow (in nsIXULWindow aWindow); */
    pub registerWindow: unsafe extern "C" fn (this: *const nsIWindowMediator, aWindow: *const nsIXULWindow) -> nsresult,

    /* [noscript] void unregisterWindow (in nsIXULWindow aWindow); */
    pub unregisterWindow: unsafe extern "C" fn (this: *const nsIWindowMediator, aWindow: *const nsIXULWindow) -> nsresult,

    /* [noscript] void updateWindowTimeStamp (in nsIXULWindow aWindow); */
    pub updateWindowTimeStamp: unsafe extern "C" fn (this: *const nsIWindowMediator, aWindow: *const nsIXULWindow) -> nsresult,

    /* [noscript] void updateWindowTitle (in nsIXULWindow aWindow, in wstring inTitle); */
    pub updateWindowTitle: unsafe extern "C" fn (this: *const nsIWindowMediator, aWindow: *const nsIXULWindow, inTitle: *const libc::int16_t) -> nsresult,

    /* [noscript] boolean calculateZPosition (in nsIXULWindow inWindow, in unsigned long inPosition, in nsIWidget inBelow, out unsigned long outPosition, out nsIWidget outBelow); */
    pub calculateZPosition: unsafe extern "C" fn (this: *const nsIWindowMediator, inWindow: *const nsIXULWindow, inPosition: libc::uint32_t, inBelow: *const nsIWidget, outPosition: *mut libc::uint32_t, outBelow: *mut *const nsIWidget, _retval: *mut bool) -> nsresult,

    /* [noscript] void setZPosition (in nsIXULWindow inWindow, in unsigned long inPosition, in nsIXULWindow inBelow); */
    pub setZPosition: unsafe extern "C" fn (this: *const nsIWindowMediator, inWindow: *const nsIXULWindow, inPosition: libc::uint32_t, inBelow: *const nsIXULWindow) -> nsresult,

    /* [noscript] uint32_t getZLevel (in nsIXULWindow aWindow); */
    pub getZLevel: unsafe extern "C" fn (this: *const nsIWindowMediator, aWindow: *const nsIXULWindow, _retval: *mut uint32_t) -> nsresult,

    /* [noscript] void setZLevel (in nsIXULWindow aWindow, in uint32_t aZLevel); */
    pub setZLevel: unsafe extern "C" fn (this: *const nsIWindowMediator, aWindow: *const nsIXULWindow, aZLevel: uint32_t) -> nsresult,

    /* void addListener (in nsIWindowMediatorListener aListener); */
    pub addListener: unsafe extern "C" fn (this: *const nsIWindowMediator, aListener: *const nsIWindowMediatorListener) -> nsresult,

    /* void removeListener (in nsIWindowMediatorListener aListener); */
    pub removeListener: unsafe extern "C" fn (this: *const nsIWindowMediator, aListener: *const nsIWindowMediatorListener) -> nsresult,

}


impl nsIWindowMediator {
    /* nsISimpleEnumerator getEnumerator (in wstring aWindowType); */
    #[inline]
    pub unsafe fn getEnumerator(&self, aWindowType: *const libc::int16_t) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getEnumerator)(self as *const _, aWindowType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator getXULWindowEnumerator (in wstring aWindowType); */
    #[inline]
    pub unsafe fn getXULWindowEnumerator(&self, aWindowType: *const libc::int16_t) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getXULWindowEnumerator)(self as *const _, aWindowType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator getZOrderDOMWindowEnumerator (in wstring aWindowType, in boolean aFrontToBack); */
    #[inline]
    pub unsafe fn getZOrderDOMWindowEnumerator(&self, aWindowType: *const libc::int16_t, aFrontToBack: bool) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getZOrderDOMWindowEnumerator)(self as *const _, aWindowType, aFrontToBack, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator getZOrderXULWindowEnumerator (in wstring aWindowType, in boolean aFrontToBack); */
    #[inline]
    pub unsafe fn getZOrderXULWindowEnumerator(&self, aWindowType: *const libc::int16_t, aFrontToBack: bool) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getZOrderXULWindowEnumerator)(self as *const _, aWindowType, aFrontToBack, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIDOMWindowProxy getMostRecentWindow (in wstring aWindowType); */
    #[inline]
    pub unsafe fn getMostRecentWindow(&self, aWindowType: *const libc::int16_t) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getMostRecentWindow)(self as *const _, aWindowType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIDOMWindowProxy getOuterWindowWithId (in unsigned long long aOuterWindowID); */
    #[inline]
    pub unsafe fn getOuterWindowWithId(&self, aOuterWindowID: libc::uint64_t) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getOuterWindowWithId)(self as *const _, aOuterWindowID, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIDOMWindow getCurrentInnerWindowWithId (in unsigned long long aInnerWindowID); */
    #[inline]
    pub unsafe fn getCurrentInnerWindowWithId(&self, aInnerWindowID: libc::uint64_t) -> Result<Option<RefPtr<mozIDOMWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCurrentInnerWindowWithId)(self as *const _, aInnerWindowID, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] void registerWindow (in nsIXULWindow aWindow); */
    #[inline]
    pub unsafe fn registerWindow(&self, aWindow: Option<&nsIXULWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).registerWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void unregisterWindow (in nsIXULWindow aWindow); */
    #[inline]
    pub unsafe fn unregisterWindow(&self, aWindow: Option<&nsIXULWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void updateWindowTimeStamp (in nsIXULWindow aWindow); */
    #[inline]
    pub unsafe fn updateWindowTimeStamp(&self, aWindow: Option<&nsIXULWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).updateWindowTimeStamp)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void updateWindowTitle (in nsIXULWindow aWindow, in wstring inTitle); */
    #[inline]
    pub unsafe fn updateWindowTitle(&self, aWindow: Option<&nsIXULWindow>, inTitle: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).updateWindowTitle)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), inTitle) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] boolean calculateZPosition (in nsIXULWindow inWindow, in unsigned long inPosition, in nsIWidget inBelow, out unsigned long outPosition, out nsIWidget outBelow); */
    #[inline]
    pub unsafe fn calculateZPosition(&self, inWindow: Option<&nsIXULWindow>, inPosition: libc::uint32_t, inBelow: Option<&nsIWidget>) -> Result<(libc::uint32_t, Option<RefPtr<nsIWidget>>, bool), nsresult> {
        let mut outPosition: libc::uint32_t = ::std::mem::zeroed();
        let mut outBelow = GetterAddrefs::new();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).calculateZPosition)(self as *const _, inWindow.map_or(::std::ptr::null(), |x| x as *const _), inPosition, inBelow.map_or(::std::ptr::null(), |x| x as *const _), &mut outPosition as *mut _, outBelow.ptr(), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((outPosition, outBelow.refptr(), _retval))
    }

    /* [noscript] void setZPosition (in nsIXULWindow inWindow, in unsigned long inPosition, in nsIXULWindow inBelow); */
    #[inline]
    pub unsafe fn setZPosition(&self, inWindow: Option<&nsIXULWindow>, inPosition: libc::uint32_t, inBelow: Option<&nsIXULWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).setZPosition)(self as *const _, inWindow.map_or(::std::ptr::null(), |x| x as *const _), inPosition, inBelow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] uint32_t getZLevel (in nsIXULWindow aWindow); */
    #[inline]
    pub unsafe fn getZLevel(&self, aWindow: Option<&nsIXULWindow>) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getZLevel)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void setZLevel (in nsIXULWindow aWindow, in uint32_t aZLevel); */
    #[inline]
    pub unsafe fn setZLevel(&self, aWindow: Option<&nsIXULWindow>, aZLevel: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setZLevel)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aZLevel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addListener (in nsIWindowMediatorListener aListener); */
    #[inline]
    pub unsafe fn addListener(&self, aListener: Option<&nsIWindowMediatorListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeListener (in nsIWindowMediatorListener aListener); */
    #[inline]
    pub unsafe fn removeListener(&self, aListener: Option<&nsIWindowMediatorListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIWindowMediator_44 {
    vtable: *const nsIWindowMediator_44VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWindowMediator_44 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb9ed4063, 0x39a2, 0x4302,
            [0x8e, 0x5c, 0x72, 0x87, 0xee, 0xf0, 0x21, 0xfe])
    }
}

unsafe impl RefCounted for nsIWindowMediator_44 {
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
pub trait nsIWindowMediator_44Coerce {
    fn coerce_from(v: &nsIWindowMediator_44) -> &Self;
}

impl nsIWindowMediator_44Coerce for nsIWindowMediator_44 {
    #[inline]
    fn coerce_from(v: &nsIWindowMediator_44) -> &Self {
        v
    }
}

impl nsIWindowMediator_44 {
    #[inline]
    pub fn coerce<T: nsIWindowMediator_44Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWindowMediator_44 {
    type Target = nsIWindowMediator;
    #[inline]
    fn deref(&self) -> &nsIWindowMediator {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIWindowMediatorCoerce> nsIWindowMediator_44Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowMediator_44) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWindowMediator_44VTable {
    pub __base: nsIWindowMediatorVTable,

    /* mozIDOMWindowProxy getMostRecentNonPBWindow (in wstring aWindowType); */
    pub getMostRecentNonPBWindow: unsafe extern "C" fn (this: *const nsIWindowMediator_44, aWindowType: *const libc::int16_t, _retval: *mut *const mozIDOMWindowProxy) -> nsresult,

}


impl nsIWindowMediator_44 {
    /* mozIDOMWindowProxy getMostRecentNonPBWindow (in wstring aWindowType); */
    #[inline]
    pub unsafe fn getMostRecentNonPBWindow(&self, aWindowType: *const libc::int16_t) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getMostRecentNonPBWindow)(self as *const _, aWindowType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



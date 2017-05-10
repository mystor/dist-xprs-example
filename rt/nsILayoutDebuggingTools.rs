//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILayoutDebuggingTools.idl
//


#[repr(C)]
pub struct nsILayoutDebuggingTools {
    vtable: *const nsILayoutDebuggingToolsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILayoutDebuggingTools {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf336d8d3, 0x9721, 0x4ad3,
            [0x85, 0xd0, 0xa7, 0x01, 0x8c, 0x0a, 0x33, 0x83])
    }
}

unsafe impl RefCounted for nsILayoutDebuggingTools {
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
pub trait nsILayoutDebuggingToolsCoerce {
    fn coerce_from(v: &nsILayoutDebuggingTools) -> &Self;
}

impl nsILayoutDebuggingToolsCoerce for nsILayoutDebuggingTools {
    #[inline]
    fn coerce_from(v: &nsILayoutDebuggingTools) -> &Self {
        v
    }
}

impl nsILayoutDebuggingTools {
    #[inline]
    pub fn coerce<T: nsILayoutDebuggingToolsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILayoutDebuggingTools {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILayoutDebuggingToolsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILayoutDebuggingTools) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILayoutDebuggingToolsVTable {
    pub __base: nsISupportsVTable,

    /* void init (in mozIDOMWindow win); */
    pub init: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, win: *const mozIDOMWindow) -> nsresult,

    /* void newURILoaded (); */
    pub newURILoaded: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools) -> nsresult,

    /* attribute boolean visualDebugging; */
    pub get_visualDebugging: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aVisualDebugging: *mut bool) -> nsresult,
    pub set_visualDebugging: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aVisualDebugging: bool) -> nsresult,

    /* attribute boolean visualEventDebugging; */
    pub get_visualEventDebugging: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aVisualEventDebugging: *mut bool) -> nsresult,
    pub set_visualEventDebugging: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aVisualEventDebugging: bool) -> nsresult,

    /* attribute boolean paintFlashing; */
    pub get_paintFlashing: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aPaintFlashing: *mut bool) -> nsresult,
    pub set_paintFlashing: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aPaintFlashing: bool) -> nsresult,

    /* attribute boolean paintDumping; */
    pub get_paintDumping: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aPaintDumping: *mut bool) -> nsresult,
    pub set_paintDumping: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aPaintDumping: bool) -> nsresult,

    /* attribute boolean invalidateDumping; */
    pub get_invalidateDumping: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aInvalidateDumping: *mut bool) -> nsresult,
    pub set_invalidateDumping: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aInvalidateDumping: bool) -> nsresult,

    /* attribute boolean eventDumping; */
    pub get_eventDumping: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aEventDumping: *mut bool) -> nsresult,
    pub set_eventDumping: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aEventDumping: bool) -> nsresult,

    /* attribute boolean motionEventDumping; */
    pub get_motionEventDumping: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aMotionEventDumping: *mut bool) -> nsresult,
    pub set_motionEventDumping: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aMotionEventDumping: bool) -> nsresult,

    /* attribute boolean crossingEventDumping; */
    pub get_crossingEventDumping: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aCrossingEventDumping: *mut bool) -> nsresult,
    pub set_crossingEventDumping: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aCrossingEventDumping: bool) -> nsresult,

    /* attribute boolean reflowCounts; */
    pub get_reflowCounts: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aReflowCounts: *mut bool) -> nsresult,
    pub set_reflowCounts: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools, aReflowCounts: bool) -> nsresult,

    /* void dumpWebShells (); */
    pub dumpWebShells: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools) -> nsresult,

    /* void dumpContent (); */
    pub dumpContent: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools) -> nsresult,

    /* void dumpFrames (); */
    pub dumpFrames: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools) -> nsresult,

    /* void dumpViews (); */
    pub dumpViews: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools) -> nsresult,

    /* void dumpStyleSheets (); */
    pub dumpStyleSheets: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools) -> nsresult,

    /* void dumpStyleContexts (); */
    pub dumpStyleContexts: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools) -> nsresult,

    /* void dumpReflowStats (); */
    pub dumpReflowStats: unsafe extern "C" fn (this: *const nsILayoutDebuggingTools) -> nsresult,

}


impl nsILayoutDebuggingTools {
    /* void init (in mozIDOMWindow win); */
    #[inline]
    pub unsafe fn init(&self, win: Option<&mozIDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, win.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void newURILoaded (); */
    #[inline]
    pub unsafe fn newURILoaded(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).newURILoaded)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean visualDebugging; */
    #[inline]
    pub unsafe fn get_visualDebugging(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_visualDebugging)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_visualDebugging(&self, aVisualDebugging: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_visualDebugging)(self as *const _, aVisualDebugging) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean visualEventDebugging; */
    #[inline]
    pub unsafe fn get_visualEventDebugging(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_visualEventDebugging)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_visualEventDebugging(&self, aVisualEventDebugging: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_visualEventDebugging)(self as *const _, aVisualEventDebugging) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean paintFlashing; */
    #[inline]
    pub unsafe fn get_paintFlashing(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_paintFlashing)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_paintFlashing(&self, aPaintFlashing: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_paintFlashing)(self as *const _, aPaintFlashing) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean paintDumping; */
    #[inline]
    pub unsafe fn get_paintDumping(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_paintDumping)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_paintDumping(&self, aPaintDumping: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_paintDumping)(self as *const _, aPaintDumping) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean invalidateDumping; */
    #[inline]
    pub unsafe fn get_invalidateDumping(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_invalidateDumping)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_invalidateDumping(&self, aInvalidateDumping: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_invalidateDumping)(self as *const _, aInvalidateDumping) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean eventDumping; */
    #[inline]
    pub unsafe fn get_eventDumping(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_eventDumping)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_eventDumping(&self, aEventDumping: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_eventDumping)(self as *const _, aEventDumping) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean motionEventDumping; */
    #[inline]
    pub unsafe fn get_motionEventDumping(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_motionEventDumping)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_motionEventDumping(&self, aMotionEventDumping: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_motionEventDumping)(self as *const _, aMotionEventDumping) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean crossingEventDumping; */
    #[inline]
    pub unsafe fn get_crossingEventDumping(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_crossingEventDumping)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_crossingEventDumping(&self, aCrossingEventDumping: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_crossingEventDumping)(self as *const _, aCrossingEventDumping) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean reflowCounts; */
    #[inline]
    pub unsafe fn get_reflowCounts(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_reflowCounts)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_reflowCounts(&self, aReflowCounts: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_reflowCounts)(self as *const _, aReflowCounts) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dumpWebShells (); */
    #[inline]
    pub unsafe fn dumpWebShells(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).dumpWebShells)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dumpContent (); */
    #[inline]
    pub unsafe fn dumpContent(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).dumpContent)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dumpFrames (); */
    #[inline]
    pub unsafe fn dumpFrames(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).dumpFrames)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dumpViews (); */
    #[inline]
    pub unsafe fn dumpViews(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).dumpViews)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dumpStyleSheets (); */
    #[inline]
    pub unsafe fn dumpStyleSheets(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).dumpStyleSheets)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dumpStyleContexts (); */
    #[inline]
    pub unsafe fn dumpStyleContexts(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).dumpStyleContexts)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dumpReflowStats (); */
    #[inline]
    pub unsafe fn dumpReflowStats(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).dumpReflowStats)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



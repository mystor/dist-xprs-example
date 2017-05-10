//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMUIEvent.idl
//


pub mod nsIDOMUIEvent_consts {
    pub const SCROLL_PAGE_UP: i64 = -32768;
    pub const SCROLL_PAGE_DOWN: i64 = 32768;
}


#[repr(C)]
pub struct nsIDOMUIEvent {
    vtable: *const nsIDOMUIEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMUIEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x85ae52eb, 0x37fa, 0x4fd9,
            [0xa2, 0xe6, 0xc7, 0xd0, 0xf2, 0xa5, 0x21, 0xb3])
    }
}

unsafe impl RefCounted for nsIDOMUIEvent {
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
pub trait nsIDOMUIEventCoerce {
    fn coerce_from(v: &nsIDOMUIEvent) -> &Self;
}

impl nsIDOMUIEventCoerce for nsIDOMUIEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMUIEvent) -> &Self {
        v
    }
}

impl nsIDOMUIEvent {
    #[inline]
    pub fn coerce<T: nsIDOMUIEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMUIEvent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMUIEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMUIEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMUIEventVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute mozIDOMWindowProxy view; */
    pub get_view: unsafe extern "C" fn (this: *const nsIDOMUIEvent, aView: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* readonly attribute long detail; */
    pub get_detail: unsafe extern "C" fn (this: *const nsIDOMUIEvent, aDetail: *mut libc::int32_t) -> nsresult,

    /* void initUIEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in mozIDOMWindow viewArg, in long detailArg); */
    pub initUIEvent: unsafe extern "C" fn (this: *const nsIDOMUIEvent, typeArg: *const nsAString, canBubbleArg: bool, cancelableArg: bool, viewArg: *const mozIDOMWindow, detailArg: libc::int32_t) -> nsresult,

    /* readonly attribute long layerX; */
    pub get_layerX: unsafe extern "C" fn (this: *const nsIDOMUIEvent, aLayerX: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long layerY; */
    pub get_layerY: unsafe extern "C" fn (this: *const nsIDOMUIEvent, aLayerY: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long pageX; */
    pub get_pageX: unsafe extern "C" fn (this: *const nsIDOMUIEvent, aPageX: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long pageY; */
    pub get_pageY: unsafe extern "C" fn (this: *const nsIDOMUIEvent, aPageY: *mut libc::int32_t) -> nsresult,

    /* readonly attribute unsigned long which; */
    pub get_which: unsafe extern "C" fn (this: *const nsIDOMUIEvent, aWhich: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIDOMNode rangeParent; */
    pub get_rangeParent: unsafe extern "C" fn (this: *const nsIDOMUIEvent, aRangeParent: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute long rangeOffset; */
    pub get_rangeOffset: unsafe extern "C" fn (this: *const nsIDOMUIEvent, aRangeOffset: *mut libc::int32_t) -> nsresult,

    /* [nostdcall,notxpcom] EventPtr AsEvent (); */
    /// Unable to call function as its signature contains a non-rust type
    pub AsEvent: *const ::libc::c_void,

}


impl nsIDOMUIEvent {
    /* readonly attribute mozIDOMWindowProxy view; */
    #[inline]
    pub unsafe fn get_view(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_view)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long detail; */
    #[inline]
    pub unsafe fn get_detail(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_detail)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void initUIEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in mozIDOMWindow viewArg, in long detailArg); */
    #[inline]
    pub unsafe fn initUIEvent(&self, typeArg: &[u16], canBubbleArg: bool, cancelableArg: bool, viewArg: Option<&mozIDOMWindow>, detailArg: libc::int32_t) -> Result<(), nsresult> {
        let typeArg = nsString::from(typeArg);
        match ((*self.vtable).initUIEvent)(self as *const _, &*typeArg, canBubbleArg, cancelableArg, viewArg.map_or(::std::ptr::null(), |x| x as *const _), detailArg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long layerX; */
    #[inline]
    pub unsafe fn get_layerX(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_layerX)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long layerY; */
    #[inline]
    pub unsafe fn get_layerY(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_layerY)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long pageX; */
    #[inline]
    pub unsafe fn get_pageX(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_pageX)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long pageY; */
    #[inline]
    pub unsafe fn get_pageY(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_pageY)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long which; */
    #[inline]
    pub unsafe fn get_which(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_which)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMNode rangeParent; */
    #[inline]
    pub unsafe fn get_rangeParent(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_rangeParent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long rangeOffset; */
    #[inline]
    pub unsafe fn get_rangeOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_rangeOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [nostdcall,notxpcom] EventPtr AsEvent (); */


}



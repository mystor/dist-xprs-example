//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClipboardDragDropHooks.idl
//


#[repr(C)]
pub struct nsIClipboardDragDropHooks {
    vtable: *const nsIClipboardDragDropHooksVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIClipboardDragDropHooks {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe03e6c5e, 0x0d84, 0x4c0b,
            [0x87, 0x39, 0xe6, 0xb8, 0xd5, 0x19, 0x22, 0xde])
    }
}

unsafe impl RefCounted for nsIClipboardDragDropHooks {
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
pub trait nsIClipboardDragDropHooksCoerce {
    fn coerce_from(v: &nsIClipboardDragDropHooks) -> &Self;
}

impl nsIClipboardDragDropHooksCoerce for nsIClipboardDragDropHooks {
    #[inline]
    fn coerce_from(v: &nsIClipboardDragDropHooks) -> &Self {
        v
    }
}

impl nsIClipboardDragDropHooks {
    #[inline]
    pub fn coerce<T: nsIClipboardDragDropHooksCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIClipboardDragDropHooks {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIClipboardDragDropHooksCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClipboardDragDropHooks) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIClipboardDragDropHooksVTable {
    pub __base: nsISupportsVTable,

    /* boolean allowStartDrag (in nsIDOMEvent event); */
    pub allowStartDrag: unsafe extern "C" fn (this: *const nsIClipboardDragDropHooks, event: *const nsIDOMEvent, _retval: *mut bool) -> nsresult,

    /* boolean allowDrop (in nsIDOMEvent event, in nsIDragSession session); */
    pub allowDrop: unsafe extern "C" fn (this: *const nsIClipboardDragDropHooks, event: *const nsIDOMEvent, session: *const nsIDragSession, _retval: *mut bool) -> nsresult,

    /* boolean onCopyOrDrag (in nsIDOMEvent aEvent, in nsITransferable trans); */
    pub onCopyOrDrag: unsafe extern "C" fn (this: *const nsIClipboardDragDropHooks, aEvent: *const nsIDOMEvent, trans: *const nsITransferable, _retval: *mut bool) -> nsresult,

    /* boolean onPasteOrDrop (in nsIDOMEvent event, in nsITransferable trans); */
    pub onPasteOrDrop: unsafe extern "C" fn (this: *const nsIClipboardDragDropHooks, event: *const nsIDOMEvent, trans: *const nsITransferable, _retval: *mut bool) -> nsresult,

}


impl nsIClipboardDragDropHooks {
    /* boolean allowStartDrag (in nsIDOMEvent event); */
    #[inline]
    pub unsafe fn allowStartDrag(&self, event: Option<&nsIDOMEvent>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).allowStartDrag)(self as *const _, event.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean allowDrop (in nsIDOMEvent event, in nsIDragSession session); */
    #[inline]
    pub unsafe fn allowDrop(&self, event: Option<&nsIDOMEvent>, session: Option<&nsIDragSession>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).allowDrop)(self as *const _, event.map_or(::std::ptr::null(), |x| x as *const _), session.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean onCopyOrDrag (in nsIDOMEvent aEvent, in nsITransferable trans); */
    #[inline]
    pub unsafe fn onCopyOrDrag(&self, aEvent: Option<&nsIDOMEvent>, trans: Option<&nsITransferable>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).onCopyOrDrag)(self as *const _, aEvent.map_or(::std::ptr::null(), |x| x as *const _), trans.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean onPasteOrDrop (in nsIDOMEvent event, in nsITransferable trans); */
    #[inline]
    pub unsafe fn onPasteOrDrop(&self, event: Option<&nsIDOMEvent>, trans: Option<&nsITransferable>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).onPasteOrDrop)(self as *const _, event.map_or(::std::ptr::null(), |x| x as *const _), trans.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



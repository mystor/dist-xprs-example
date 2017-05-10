//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContextMenuListener.idl
//


pub mod nsIContextMenuListener_consts {
    pub const CONTEXT_NONE: i64 = 0;
    pub const CONTEXT_LINK: i64 = 1;
    pub const CONTEXT_IMAGE: i64 = 2;
    pub const CONTEXT_DOCUMENT: i64 = 4;
    pub const CONTEXT_TEXT: i64 = 8;
    pub const CONTEXT_INPUT: i64 = 16;
}


#[repr(C)]
pub struct nsIContextMenuListener {
    vtable: *const nsIContextMenuListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContextMenuListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3478b6b0, 0x3875, 0x11d4,
            [0x94, 0xef, 0x00, 0x20, 0x18, 0x3b, 0xf1, 0x81])
    }
}

unsafe impl RefCounted for nsIContextMenuListener {
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
pub trait nsIContextMenuListenerCoerce {
    fn coerce_from(v: &nsIContextMenuListener) -> &Self;
}

impl nsIContextMenuListenerCoerce for nsIContextMenuListener {
    #[inline]
    fn coerce_from(v: &nsIContextMenuListener) -> &Self {
        v
    }
}

impl nsIContextMenuListener {
    #[inline]
    pub fn coerce<T: nsIContextMenuListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContextMenuListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContextMenuListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContextMenuListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContextMenuListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onShowContextMenu (in unsigned long aContextFlags, in nsIDOMEvent aEvent, in nsIDOMNode aNode); */
    pub onShowContextMenu: unsafe extern "C" fn (this: *const nsIContextMenuListener, aContextFlags: libc::uint32_t, aEvent: *const nsIDOMEvent, aNode: *const nsIDOMNode) -> nsresult,

}


impl nsIContextMenuListener {
    /* void onShowContextMenu (in unsigned long aContextFlags, in nsIDOMEvent aEvent, in nsIDOMNode aNode); */
    #[inline]
    pub unsafe fn onShowContextMenu(&self, aContextFlags: libc::uint32_t, aEvent: Option<&nsIDOMEvent>, aNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).onShowContextMenu)(self as *const _, aContextFlags, aEvent.map_or(::std::ptr::null(), |x| x as *const _), aNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISelectionListener.idl
//


pub mod nsISelectionListener_consts {
    pub const NO_REASON: i64 = 0;
    pub const DRAG_REASON: i64 = 1;
    pub const MOUSEDOWN_REASON: i64 = 2;
    pub const MOUSEUP_REASON: i64 = 4;
    pub const KEYPRESS_REASON: i64 = 8;
    pub const SELECTALL_REASON: i64 = 16;
    pub const COLLAPSETOSTART_REASON: i64 = 32;
    pub const COLLAPSETOEND_REASON: i64 = 64;
    pub const IME_REASON: i64 = 128;
}


#[repr(C)]
pub struct nsISelectionListener {
    vtable: *const nsISelectionListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISelectionListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x45686299, 0xae2b, 0x46bc,
            [0x95, 0x02, 0xc5, 0x6c, 0x35, 0x69, 0x1a, 0xb9])
    }
}

unsafe impl RefCounted for nsISelectionListener {
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
pub trait nsISelectionListenerCoerce {
    fn coerce_from(v: &nsISelectionListener) -> &Self;
}

impl nsISelectionListenerCoerce for nsISelectionListener {
    #[inline]
    fn coerce_from(v: &nsISelectionListener) -> &Self {
        v
    }
}

impl nsISelectionListener {
    #[inline]
    pub fn coerce<T: nsISelectionListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISelectionListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISelectionListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISelectionListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISelectionListenerVTable {
    pub __base: nsISupportsVTable,

    /* void notifySelectionChanged (in nsIDOMDocument doc, in nsISelection sel, in short reason); */
    pub notifySelectionChanged: unsafe extern "C" fn (this: *const nsISelectionListener, doc: *const nsIDOMDocument, sel: *const nsISelection, reason: libc::int16_t) -> nsresult,

}


impl nsISelectionListener {
    /* void notifySelectionChanged (in nsIDOMDocument doc, in nsISelection sel, in short reason); */
    #[inline]
    pub unsafe fn notifySelectionChanged(&self, doc: Option<&nsIDOMDocument>, sel: Option<&nsISelection>, reason: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).notifySelectionChanged)(self as *const _, doc.map_or(::std::ptr::null(), |x| x as *const _), sel.map_or(::std::ptr::null(), |x| x as *const _), reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



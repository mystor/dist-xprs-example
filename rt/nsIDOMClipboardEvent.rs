//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMClipboardEvent.idl
//


#[repr(C)]
pub struct nsIDOMClipboardEvent {
    vtable: *const nsIDOMClipboardEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMClipboardEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb54d6144, 0x3980, 0x4895,
            [0x83, 0xc7, 0x82, 0xf1, 0x58, 0xbc, 0x1c, 0xf5])
    }
}

unsafe impl RefCounted for nsIDOMClipboardEvent {
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
pub trait nsIDOMClipboardEventCoerce {
    fn coerce_from(v: &nsIDOMClipboardEvent) -> &Self;
}

impl nsIDOMClipboardEventCoerce for nsIDOMClipboardEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMClipboardEvent) -> &Self {
        v
    }
}

impl nsIDOMClipboardEvent {
    #[inline]
    pub fn coerce<T: nsIDOMClipboardEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMClipboardEvent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMClipboardEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMClipboardEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMClipboardEventVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMDataTransfer clipboardData; */
    pub get_clipboardData: unsafe extern "C" fn (this: *const nsIDOMClipboardEvent, aClipboardData: *mut *const nsIDOMDataTransfer) -> nsresult,

    /* [noscript] void initClipboardEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in nsIDOMDataTransfer clipboardData); */
    pub initClipboardEvent: unsafe extern "C" fn (this: *const nsIDOMClipboardEvent, typeArg: *const nsAString, canBubbleArg: bool, cancelableArg: bool, clipboardData: *const nsIDOMDataTransfer) -> nsresult,

}


impl nsIDOMClipboardEvent {
    /* readonly attribute nsIDOMDataTransfer clipboardData; */
    #[inline]
    pub unsafe fn get_clipboardData(&self, ) -> Result<Option<RefPtr<nsIDOMDataTransfer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_clipboardData)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] void initClipboardEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in nsIDOMDataTransfer clipboardData); */
    #[inline]
    pub unsafe fn initClipboardEvent(&self, typeArg: &[u16], canBubbleArg: bool, cancelableArg: bool, clipboardData: Option<&nsIDOMDataTransfer>) -> Result<(), nsresult> {
        let typeArg = nsString::from(typeArg);
        match ((*self.vtable).initClipboardEvent)(self as *const _, &*typeArg, canBubbleArg, cancelableArg, clipboardData.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



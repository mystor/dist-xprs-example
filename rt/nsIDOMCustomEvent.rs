//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCustomEvent.idl
//


#[repr(C)]
pub struct nsIDOMCustomEvent {
    vtable: *const nsIDOMCustomEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCustomEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5be16b03, 0x36f9, 0x4ca8,
            [0xb2, 0xc5, 0x0d, 0xaa, 0xdf, 0x3c, 0xd1, 0xb3])
    }
}

unsafe impl RefCounted for nsIDOMCustomEvent {
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
pub trait nsIDOMCustomEventCoerce {
    fn coerce_from(v: &nsIDOMCustomEvent) -> &Self;
}

impl nsIDOMCustomEventCoerce for nsIDOMCustomEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMCustomEvent) -> &Self {
        v
    }
}

impl nsIDOMCustomEvent {
    #[inline]
    pub fn coerce<T: nsIDOMCustomEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCustomEvent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCustomEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCustomEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCustomEventVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIVariant detail; */
    pub get_detail: unsafe extern "C" fn (this: *const nsIDOMCustomEvent, aDetail: *mut *const nsIVariant) -> nsresult,

    /* void initCustomEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in nsIVariant detailArg); */
    pub initCustomEvent: unsafe extern "C" fn (this: *const nsIDOMCustomEvent, typeArg: *const nsAString, canBubbleArg: bool, cancelableArg: bool, detailArg: *const nsIVariant) -> nsresult,

}


impl nsIDOMCustomEvent {
    /* readonly attribute nsIVariant detail; */
    #[inline]
    pub unsafe fn get_detail(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_detail)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void initCustomEvent (in DOMString typeArg, in boolean canBubbleArg, in boolean cancelableArg, in nsIVariant detailArg); */
    #[inline]
    pub unsafe fn initCustomEvent(&self, typeArg: &[u16], canBubbleArg: bool, cancelableArg: bool, detailArg: Option<&nsIVariant>) -> Result<(), nsresult> {
        let typeArg = nsString::from(typeArg);
        match ((*self.vtable).initCustomEvent)(self as *const _, &*typeArg, canBubbleArg, cancelableArg, detailArg.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



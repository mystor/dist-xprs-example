//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMAnimationEvent.idl
//


#[repr(C)]
pub struct nsIDOMAnimationEvent {
    vtable: *const nsIDOMAnimationEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMAnimationEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xce6d1db3, 0x53b8, 0x4ade,
            [0x9b, 0xaa, 0x70, 0xf4, 0x94, 0x72, 0x00, 0xa2])
    }
}

unsafe impl RefCounted for nsIDOMAnimationEvent {
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
pub trait nsIDOMAnimationEventCoerce {
    fn coerce_from(v: &nsIDOMAnimationEvent) -> &Self;
}

impl nsIDOMAnimationEventCoerce for nsIDOMAnimationEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMAnimationEvent) -> &Self {
        v
    }
}

impl nsIDOMAnimationEvent {
    #[inline]
    pub fn coerce<T: nsIDOMAnimationEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMAnimationEvent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMAnimationEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMAnimationEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMAnimationEventVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString animationName; */
    pub get_animationName: unsafe extern "C" fn (this: *const nsIDOMAnimationEvent, aAnimationName: *mut nsAString) -> nsresult,

    /* readonly attribute float elapsedTime; */
    pub get_elapsedTime: unsafe extern "C" fn (this: *const nsIDOMAnimationEvent, aElapsedTime: *mut libc::c_float) -> nsresult,

    /* readonly attribute DOMString pseudoElement; */
    pub get_pseudoElement: unsafe extern "C" fn (this: *const nsIDOMAnimationEvent, aPseudoElement: *mut nsAString) -> nsresult,

}


impl nsIDOMAnimationEvent {
    /* readonly attribute DOMString animationName; */
    #[inline]
    pub unsafe fn get_animationName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_animationName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute float elapsedTime; */
    #[inline]
    pub unsafe fn get_elapsedTime(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_elapsedTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString pseudoElement; */
    #[inline]
    pub unsafe fn get_pseudoElement(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_pseudoElement)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



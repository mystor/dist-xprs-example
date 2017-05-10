//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISliderListener.idl
//


#[repr(C)]
pub struct nsISliderListener {
    vtable: *const nsISliderListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISliderListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe5b3074e, 0xee18, 0x4538,
            [0x83, 0xb9, 0x24, 0x87, 0xd9, 0x0a, 0x2a, 0x34])
    }
}

unsafe impl RefCounted for nsISliderListener {
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
pub trait nsISliderListenerCoerce {
    fn coerce_from(v: &nsISliderListener) -> &Self;
}

impl nsISliderListenerCoerce for nsISliderListener {
    #[inline]
    fn coerce_from(v: &nsISliderListener) -> &Self {
        v
    }
}

impl nsISliderListener {
    #[inline]
    pub fn coerce<T: nsISliderListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISliderListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISliderListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISliderListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISliderListenerVTable {
    pub __base: nsISupportsVTable,

    /* void valueChanged (in AString which, in long newValue, in boolean userChanged); */
    pub valueChanged: unsafe extern "C" fn (this: *const nsISliderListener, which: *const nsAString, newValue: libc::int32_t, userChanged: bool) -> nsresult,

    /* void dragStateChanged (in boolean isDragging); */
    pub dragStateChanged: unsafe extern "C" fn (this: *const nsISliderListener, isDragging: bool) -> nsresult,

}


impl nsISliderListener {
    /* void valueChanged (in AString which, in long newValue, in boolean userChanged); */
    #[inline]
    pub unsafe fn valueChanged(&self, which: &[u16], newValue: libc::int32_t, userChanged: bool) -> Result<(), nsresult> {
        let which = nsString::from(which);
        match ((*self.vtable).valueChanged)(self as *const _, &*which, newValue, userChanged) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dragStateChanged (in boolean isDragging); */
    #[inline]
    pub unsafe fn dragStateChanged(&self, isDragging: bool) -> Result<(), nsresult> {

        match ((*self.vtable).dragStateChanged)(self as *const _, isDragging) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



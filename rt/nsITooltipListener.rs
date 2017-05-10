//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITooltipListener.idl
//


#[repr(C)]
pub struct nsITooltipListener {
    vtable: *const nsITooltipListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITooltipListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x44b78386, 0x1dd2, 0x11b2,
            [0x9a, 0xd2, 0xe4, 0xee, 0xe2, 0xca, 0x19, 0x16])
    }
}

unsafe impl RefCounted for nsITooltipListener {
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
pub trait nsITooltipListenerCoerce {
    fn coerce_from(v: &nsITooltipListener) -> &Self;
}

impl nsITooltipListenerCoerce for nsITooltipListener {
    #[inline]
    fn coerce_from(v: &nsITooltipListener) -> &Self {
        v
    }
}

impl nsITooltipListener {
    #[inline]
    pub fn coerce<T: nsITooltipListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITooltipListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITooltipListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITooltipListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITooltipListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onShowTooltip (in long aXCoords, in long aYCoords, in wstring aTipText, in wstring aTipDir); */
    pub onShowTooltip: unsafe extern "C" fn (this: *const nsITooltipListener, aXCoords: libc::int32_t, aYCoords: libc::int32_t, aTipText: *const libc::int16_t, aTipDir: *const libc::int16_t) -> nsresult,

    /* void onHideTooltip (); */
    pub onHideTooltip: unsafe extern "C" fn (this: *const nsITooltipListener) -> nsresult,

}


impl nsITooltipListener {
    /* void onShowTooltip (in long aXCoords, in long aYCoords, in wstring aTipText, in wstring aTipDir); */
    #[inline]
    pub unsafe fn onShowTooltip(&self, aXCoords: libc::int32_t, aYCoords: libc::int32_t, aTipText: *const libc::int16_t, aTipDir: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).onShowTooltip)(self as *const _, aXCoords, aYCoords, aTipText, aTipDir) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onHideTooltip (); */
    #[inline]
    pub unsafe fn onHideTooltip(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onHideTooltip)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocumentStateListener.idl
//


#[repr(C)]
pub struct nsIDocumentStateListener {
    vtable: *const nsIDocumentStateListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDocumentStateListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x050cdc00, 0x3b8e, 0x11d3,
            [0x9c, 0xe4, 0xa4, 0x58, 0xf4, 0x54, 0xfc, 0xbc])
    }
}

unsafe impl RefCounted for nsIDocumentStateListener {
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
pub trait nsIDocumentStateListenerCoerce {
    fn coerce_from(v: &nsIDocumentStateListener) -> &Self;
}

impl nsIDocumentStateListenerCoerce for nsIDocumentStateListener {
    #[inline]
    fn coerce_from(v: &nsIDocumentStateListener) -> &Self {
        v
    }
}

impl nsIDocumentStateListener {
    #[inline]
    pub fn coerce<T: nsIDocumentStateListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDocumentStateListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDocumentStateListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocumentStateListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDocumentStateListenerVTable {
    pub __base: nsISupportsVTable,

    /* void NotifyDocumentCreated (); */
    pub NotifyDocumentCreated: unsafe extern "C" fn (this: *const nsIDocumentStateListener) -> nsresult,

    /* void NotifyDocumentWillBeDestroyed (); */
    pub NotifyDocumentWillBeDestroyed: unsafe extern "C" fn (this: *const nsIDocumentStateListener) -> nsresult,

    /* void NotifyDocumentStateChanged (in boolean nowDirty); */
    pub NotifyDocumentStateChanged: unsafe extern "C" fn (this: *const nsIDocumentStateListener, nowDirty: bool) -> nsresult,

}


impl nsIDocumentStateListener {
    /* void NotifyDocumentCreated (); */
    #[inline]
    pub unsafe fn NotifyDocumentCreated(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).NotifyDocumentCreated)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void NotifyDocumentWillBeDestroyed (); */
    #[inline]
    pub unsafe fn NotifyDocumentWillBeDestroyed(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).NotifyDocumentWillBeDestroyed)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void NotifyDocumentStateChanged (in boolean nowDirty); */
    #[inline]
    pub unsafe fn NotifyDocumentStateChanged(&self, nowDirty: bool) -> Result<(), nsresult> {

        match ((*self.vtable).NotifyDocumentStateChanged)(self as *const _, nowDirty) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



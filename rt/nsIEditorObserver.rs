//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditorObserver.idl
//


#[repr(C)]
pub struct nsIEditorObserver {
    vtable: *const nsIEditorObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEditorObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf3ee57a6, 0x890c, 0x4ce0,
            [0xa5, 0x84, 0x8a, 0x84, 0xbb, 0xa0, 0x29, 0x2e])
    }
}

unsafe impl RefCounted for nsIEditorObserver {
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
pub trait nsIEditorObserverCoerce {
    fn coerce_from(v: &nsIEditorObserver) -> &Self;
}

impl nsIEditorObserverCoerce for nsIEditorObserver {
    #[inline]
    fn coerce_from(v: &nsIEditorObserver) -> &Self {
        v
    }
}

impl nsIEditorObserver {
    #[inline]
    pub fn coerce<T: nsIEditorObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEditorObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEditorObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditorObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEditorObserverVTable {
    pub __base: nsISupportsVTable,

    /* void EditAction (); */
    pub EditAction: unsafe extern "C" fn (this: *const nsIEditorObserver) -> nsresult,

    /* void BeforeEditAction (); */
    pub BeforeEditAction: unsafe extern "C" fn (this: *const nsIEditorObserver) -> nsresult,

    /* void CancelEditAction (); */
    pub CancelEditAction: unsafe extern "C" fn (this: *const nsIEditorObserver) -> nsresult,

}


impl nsIEditorObserver {
    /* void EditAction (); */
    #[inline]
    pub unsafe fn EditAction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).EditAction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void BeforeEditAction (); */
    #[inline]
    pub unsafe fn BeforeEditAction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).BeforeEditAction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void CancelEditAction (); */
    #[inline]
    pub unsafe fn CancelEditAction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).CancelEditAction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



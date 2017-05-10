//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIReflowObserver.idl
//


#[repr(C)]
pub struct nsIReflowObserver {
    vtable: *const nsIReflowObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIReflowObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x832e692c, 0xc4a6, 0x11e2,
            [0x8f, 0xd1, 0xdc, 0xe6, 0x78, 0x95, 0x7a, 0x39])
    }
}

unsafe impl RefCounted for nsIReflowObserver {
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
pub trait nsIReflowObserverCoerce {
    fn coerce_from(v: &nsIReflowObserver) -> &Self;
}

impl nsIReflowObserverCoerce for nsIReflowObserver {
    #[inline]
    fn coerce_from(v: &nsIReflowObserver) -> &Self {
        v
    }
}

impl nsIReflowObserver {
    #[inline]
    pub fn coerce<T: nsIReflowObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIReflowObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIReflowObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIReflowObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIReflowObserverVTable {
    pub __base: nsISupportsVTable,

    /* void reflow (in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
    pub reflow: unsafe extern "C" fn (this: *const nsIReflowObserver, start: DOMHighResTimeStamp, end: DOMHighResTimeStamp) -> nsresult,

    /* void reflowInterruptible (in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
    pub reflowInterruptible: unsafe extern "C" fn (this: *const nsIReflowObserver, start: DOMHighResTimeStamp, end: DOMHighResTimeStamp) -> nsresult,

}


impl nsIReflowObserver {
    /* void reflow (in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
    #[inline]
    pub unsafe fn reflow(&self, start: DOMHighResTimeStamp, end: DOMHighResTimeStamp) -> Result<(), nsresult> {

        match ((*self.vtable).reflow)(self as *const _, start, end) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reflowInterruptible (in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
    #[inline]
    pub unsafe fn reflowInterruptible(&self, start: DOMHighResTimeStamp, end: DOMHighResTimeStamp) -> Result<(), nsresult> {

        match ((*self.vtable).reflowInterruptible)(self as *const _, start, end) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



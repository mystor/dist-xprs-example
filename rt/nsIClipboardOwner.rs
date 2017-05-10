//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClipboardOwner.idl
//


#[repr(C)]
pub struct nsIClipboardOwner {
    vtable: *const nsIClipboardOwnerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIClipboardOwner {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5a31c7a1, 0xe122, 0x11d2,
            [0x9a, 0x57, 0x00, 0x00, 0x64, 0x65, 0x73, 0x74])
    }
}

unsafe impl RefCounted for nsIClipboardOwner {
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
pub trait nsIClipboardOwnerCoerce {
    fn coerce_from(v: &nsIClipboardOwner) -> &Self;
}

impl nsIClipboardOwnerCoerce for nsIClipboardOwner {
    #[inline]
    fn coerce_from(v: &nsIClipboardOwner) -> &Self {
        v
    }
}

impl nsIClipboardOwner {
    #[inline]
    pub fn coerce<T: nsIClipboardOwnerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIClipboardOwner {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIClipboardOwnerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClipboardOwner) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIClipboardOwnerVTable {
    pub __base: nsISupportsVTable,

    /* void LosingOwnership (in nsITransferable aTransferable); */
    pub LosingOwnership: unsafe extern "C" fn (this: *const nsIClipboardOwner, aTransferable: *const nsITransferable) -> nsresult,

}


impl nsIClipboardOwner {
    /* void LosingOwnership (in nsITransferable aTransferable); */
    #[inline]
    pub unsafe fn LosingOwnership(&self, aTransferable: Option<&nsITransferable>) -> Result<(), nsresult> {

        match ((*self.vtable).LosingOwnership)(self as *const _, aTransferable.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



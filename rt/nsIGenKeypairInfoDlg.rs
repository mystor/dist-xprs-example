//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGenKeypairInfoDlg.idl
//


#[repr(C)]
pub struct nsIGeneratingKeypairInfoDialogs {
    vtable: *const nsIGeneratingKeypairInfoDialogsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGeneratingKeypairInfoDialogs {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x11bf5cdc, 0x1dd2, 0x11b2,
            [0xba, 0x6a, 0xc7, 0x6a, 0xfb, 0x32, 0x6f, 0xa1])
    }
}

unsafe impl RefCounted for nsIGeneratingKeypairInfoDialogs {
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
pub trait nsIGeneratingKeypairInfoDialogsCoerce {
    fn coerce_from(v: &nsIGeneratingKeypairInfoDialogs) -> &Self;
}

impl nsIGeneratingKeypairInfoDialogsCoerce for nsIGeneratingKeypairInfoDialogs {
    #[inline]
    fn coerce_from(v: &nsIGeneratingKeypairInfoDialogs) -> &Self {
        v
    }
}

impl nsIGeneratingKeypairInfoDialogs {
    #[inline]
    pub fn coerce<T: nsIGeneratingKeypairInfoDialogsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGeneratingKeypairInfoDialogs {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGeneratingKeypairInfoDialogsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGeneratingKeypairInfoDialogs) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGeneratingKeypairInfoDialogsVTable {
    pub __base: nsISupportsVTable,

    /* void displayGeneratingKeypairInfo (in nsIInterfaceRequestor ctx, in nsIKeygenThread runnable); */
    pub displayGeneratingKeypairInfo: unsafe extern "C" fn (this: *const nsIGeneratingKeypairInfoDialogs, ctx: *const nsIInterfaceRequestor, runnable: *const nsIKeygenThread) -> nsresult,

}


impl nsIGeneratingKeypairInfoDialogs {
    /* void displayGeneratingKeypairInfo (in nsIInterfaceRequestor ctx, in nsIKeygenThread runnable); */
    #[inline]
    pub unsafe fn displayGeneratingKeypairInfo(&self, ctx: Option<&nsIInterfaceRequestor>, runnable: Option<&nsIKeygenThread>) -> Result<(), nsresult> {

        match ((*self.vtable).displayGeneratingKeypairInfo)(self as *const _, ctx.map_or(::std::ptr::null(), |x| x as *const _), runnable.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



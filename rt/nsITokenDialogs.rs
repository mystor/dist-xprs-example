//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITokenDialogs.idl
//


#[repr(C)]
pub struct nsITokenDialogs {
    vtable: *const nsITokenDialogsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITokenDialogs {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa1cbc159, 0x468c, 0x495d,
            [0x80, 0x68, 0x61, 0xdd, 0x53, 0x8c, 0xbc, 0xca])
    }
}

unsafe impl RefCounted for nsITokenDialogs {
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
pub trait nsITokenDialogsCoerce {
    fn coerce_from(v: &nsITokenDialogs) -> &Self;
}

impl nsITokenDialogsCoerce for nsITokenDialogs {
    #[inline]
    fn coerce_from(v: &nsITokenDialogs) -> &Self {
        v
    }
}

impl nsITokenDialogs {
    #[inline]
    pub fn coerce<T: nsITokenDialogsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITokenDialogs {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITokenDialogsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITokenDialogs) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITokenDialogsVTable {
    pub __base: nsISupportsVTable,

    /* void ChooseToken (in nsIInterfaceRequestor ctx, [array, size_is (count)] in wstring tokenNameList, in unsigned long count, out AString tokenName, out boolean canceled); */
    /// Unable to call function as its signature contains a non-rust type
    pub ChooseToken: *const ::libc::c_void,

    /* void displayProtectedAuth (in nsIInterfaceRequestor ctx, in nsIProtectedAuthThread runnable); */
    pub displayProtectedAuth: unsafe extern "C" fn (this: *const nsITokenDialogs, ctx: *const nsIInterfaceRequestor, runnable: *const nsIProtectedAuthThread) -> nsresult,

}


impl nsITokenDialogs {
    /* void ChooseToken (in nsIInterfaceRequestor ctx, [array, size_is (count)] in wstring tokenNameList, in unsigned long count, out AString tokenName, out boolean canceled); */


    /* void displayProtectedAuth (in nsIInterfaceRequestor ctx, in nsIProtectedAuthThread runnable); */
    #[inline]
    pub unsafe fn displayProtectedAuth(&self, ctx: Option<&nsIInterfaceRequestor>, runnable: Option<&nsIProtectedAuthThread>) -> Result<(), nsresult> {

        match ((*self.vtable).displayProtectedAuth)(self as *const _, ctx.map_or(::std::ptr::null(), |x| x as *const _), runnable.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



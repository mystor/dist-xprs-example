//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPromptFactory.idl
//


#[repr(C)]
pub struct nsIPromptFactory {
    vtable: *const nsIPromptFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPromptFactory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2803541c, 0xc96a, 0x4ff1,
            [0xbd, 0x7c, 0x9c, 0xb5, 0x66, 0xd4, 0x6a, 0xeb])
    }
}

unsafe impl RefCounted for nsIPromptFactory {
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
pub trait nsIPromptFactoryCoerce {
    fn coerce_from(v: &nsIPromptFactory) -> &Self;
}

impl nsIPromptFactoryCoerce for nsIPromptFactory {
    #[inline]
    fn coerce_from(v: &nsIPromptFactory) -> &Self {
        v
    }
}

impl nsIPromptFactory {
    #[inline]
    pub fn coerce<T: nsIPromptFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPromptFactory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPromptFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPromptFactory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPromptFactoryVTable {
    pub __base: nsISupportsVTable,

    /* void getPrompt (in mozIDOMWindowProxy aParent, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    pub getPrompt: unsafe extern "C" fn (this: *const nsIPromptFactory, aParent: *const mozIDOMWindowProxy, iid: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

}


impl nsIPromptFactory {
    /* void getPrompt (in mozIDOMWindowProxy aParent, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn getPrompt<T: XpCom>(&self, aParent: Option<&mozIDOMWindowProxy>) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).getPrompt)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPIPromptService.idl
//


#[repr(C)]
pub struct nsPIPromptService {
    vtable: *const nsPIPromptServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsPIPromptService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc60a1955, 0x6cb3, 0x4827,
            [0x8e, 0xf8, 0x4f, 0x5c, 0x66, 0x8a, 0xf0, 0xb3])
    }
}

unsafe impl RefCounted for nsPIPromptService {
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
pub trait nsPIPromptServiceCoerce {
    fn coerce_from(v: &nsPIPromptService) -> &Self;
}

impl nsPIPromptServiceCoerce for nsPIPromptService {
    #[inline]
    fn coerce_from(v: &nsPIPromptService) -> &Self {
        v
    }
}

impl nsPIPromptService {
    #[inline]
    pub fn coerce<T: nsPIPromptServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsPIPromptService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsPIPromptServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPIPromptService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsPIPromptServiceVTable {
    pub __base: nsISupportsVTable,

    /* void doDialog (in nsIDOMWindow aParent, in nsIDialogParamBlock aParamBlock, in string aChromeURL); */
    pub doDialog: unsafe extern "C" fn (this: *const nsPIPromptService, aParent: *const nsIDOMWindow, aParamBlock: *const nsIDialogParamBlock, aChromeURL: *const libc::c_char) -> nsresult,

}


impl nsPIPromptService {
    /* void doDialog (in nsIDOMWindow aParent, in nsIDialogParamBlock aParamBlock, in string aChromeURL); */
    #[inline]
    pub unsafe fn doDialog(&self, aParent: Option<&nsIDOMWindow>, aParamBlock: Option<&nsIDialogParamBlock>, aChromeURL: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).doDialog)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aParamBlock.map_or(::std::ptr::null(), |x| x as *const _), aChromeURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIControllerContext.idl
//


#[repr(C)]
pub struct nsIControllerContext {
    vtable: *const nsIControllerContextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIControllerContext {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x47b82b60, 0xa36f, 0x4167,
            [0x80, 0x72, 0x6f, 0x42, 0x11, 0x51, 0xed, 0x50])
    }
}

unsafe impl RefCounted for nsIControllerContext {
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
pub trait nsIControllerContextCoerce {
    fn coerce_from(v: &nsIControllerContext) -> &Self;
}

impl nsIControllerContextCoerce for nsIControllerContext {
    #[inline]
    fn coerce_from(v: &nsIControllerContext) -> &Self {
        v
    }
}

impl nsIControllerContext {
    #[inline]
    pub fn coerce<T: nsIControllerContextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIControllerContext {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIControllerContextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIControllerContext) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIControllerContextVTable {
    pub __base: nsISupportsVTable,

    /* void init (in nsIControllerCommandTable aCommandTable); */
    pub init: unsafe extern "C" fn (this: *const nsIControllerContext, aCommandTable: *const nsIControllerCommandTable) -> nsresult,

    /* void setCommandContext (in nsISupports aCommandContext); */
    pub setCommandContext: unsafe extern "C" fn (this: *const nsIControllerContext, aCommandContext: *const nsISupports) -> nsresult,

}


impl nsIControllerContext {
    /* void init (in nsIControllerCommandTable aCommandTable); */
    #[inline]
    pub unsafe fn init(&self, aCommandTable: Option<&nsIControllerCommandTable>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aCommandTable.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCommandContext (in nsISupports aCommandContext); */
    #[inline]
    pub unsafe fn setCommandContext(&self, aCommandContext: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).setCommandContext)(self as *const _, aCommandContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



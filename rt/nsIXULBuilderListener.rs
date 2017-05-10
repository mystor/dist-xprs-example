//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULBuilderListener.idl
//


#[repr(C)]
pub struct nsIXULBuilderListener {
    vtable: *const nsIXULBuilderListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULBuilderListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xac46be8f, 0xc863, 0x4c23,
            [0x84, 0xa2, 0xd0, 0xfc, 0xc8, 0xdf, 0xa9, 0xf4])
    }
}

unsafe impl RefCounted for nsIXULBuilderListener {
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
pub trait nsIXULBuilderListenerCoerce {
    fn coerce_from(v: &nsIXULBuilderListener) -> &Self;
}

impl nsIXULBuilderListenerCoerce for nsIXULBuilderListener {
    #[inline]
    fn coerce_from(v: &nsIXULBuilderListener) -> &Self {
        v
    }
}

impl nsIXULBuilderListener {
    #[inline]
    pub fn coerce<T: nsIXULBuilderListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULBuilderListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXULBuilderListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULBuilderListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULBuilderListenerVTable {
    pub __base: nsISupportsVTable,

    /* void willRebuild (in nsIXULTemplateBuilder aBuilder); */
    pub willRebuild: unsafe extern "C" fn (this: *const nsIXULBuilderListener, aBuilder: *const nsIXULTemplateBuilder) -> nsresult,

    /* void didRebuild (in nsIXULTemplateBuilder aBuilder); */
    pub didRebuild: unsafe extern "C" fn (this: *const nsIXULBuilderListener, aBuilder: *const nsIXULTemplateBuilder) -> nsresult,

}


impl nsIXULBuilderListener {
    /* void willRebuild (in nsIXULTemplateBuilder aBuilder); */
    #[inline]
    pub unsafe fn willRebuild(&self, aBuilder: Option<&nsIXULTemplateBuilder>) -> Result<(), nsresult> {

        match ((*self.vtable).willRebuild)(self as *const _, aBuilder.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void didRebuild (in nsIXULTemplateBuilder aBuilder); */
    #[inline]
    pub unsafe fn didRebuild(&self, aBuilder: Option<&nsIXULTemplateBuilder>) -> Result<(), nsresult> {

        match ((*self.vtable).didRebuild)(self as *const _, aBuilder.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheListener.idl
//


#[repr(C)]
pub struct nsICacheListener {
    vtable: *const nsICacheListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8eadf2ed, 0x8cac, 0x4961,
            [0x80, 0x25, 0x6d, 0xa6, 0xd5, 0x82, 0x7e, 0x74])
    }
}

unsafe impl RefCounted for nsICacheListener {
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
pub trait nsICacheListenerCoerce {
    fn coerce_from(v: &nsICacheListener) -> &Self;
}

impl nsICacheListenerCoerce for nsICacheListener {
    #[inline]
    fn coerce_from(v: &nsICacheListener) -> &Self {
        v
    }
}

impl nsICacheListener {
    #[inline]
    pub fn coerce<T: nsICacheListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onCacheEntryAvailable (in nsICacheEntryDescriptor descriptor, in nsCacheAccessMode accessGranted, in nsresult status); */
    pub onCacheEntryAvailable: unsafe extern "C" fn (this: *const nsICacheListener, descriptor: *const nsICacheEntryDescriptor, accessGranted: nsCacheAccessMode, status: nsresult) -> nsresult,

    /* void onCacheEntryDoomed (in nsresult status); */
    pub onCacheEntryDoomed: unsafe extern "C" fn (this: *const nsICacheListener, status: nsresult) -> nsresult,

}


impl nsICacheListener {
    /* void onCacheEntryAvailable (in nsICacheEntryDescriptor descriptor, in nsCacheAccessMode accessGranted, in nsresult status); */
    #[inline]
    pub unsafe fn onCacheEntryAvailable(&self, descriptor: Option<&nsICacheEntryDescriptor>, accessGranted: nsCacheAccessMode, status: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onCacheEntryAvailable)(self as *const _, descriptor.map_or(::std::ptr::null(), |x| x as *const _), accessGranted, status) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onCacheEntryDoomed (in nsresult status); */
    #[inline]
    pub unsafe fn onCacheEntryDoomed(&self, status: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onCacheEntryDoomed)(self as *const _, status) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



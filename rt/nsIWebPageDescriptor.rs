//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebPageDescriptor.idl
//


pub mod nsIWebPageDescriptor_consts {
    pub const DISPLAY_AS_SOURCE: i64 = 1;
    pub const DISPLAY_NORMAL: i64 = 2;
}


#[repr(C)]
pub struct nsIWebPageDescriptor {
    vtable: *const nsIWebPageDescriptorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebPageDescriptor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6f30b676, 0x3710, 0x4c2c,
            [0x80, 0xb1, 0x03, 0x95, 0xfb, 0x26, 0x51, 0x6e])
    }
}

unsafe impl RefCounted for nsIWebPageDescriptor {
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
pub trait nsIWebPageDescriptorCoerce {
    fn coerce_from(v: &nsIWebPageDescriptor) -> &Self;
}

impl nsIWebPageDescriptorCoerce for nsIWebPageDescriptor {
    #[inline]
    fn coerce_from(v: &nsIWebPageDescriptor) -> &Self {
        v
    }
}

impl nsIWebPageDescriptor {
    #[inline]
    pub fn coerce<T: nsIWebPageDescriptorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebPageDescriptor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebPageDescriptorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebPageDescriptor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebPageDescriptorVTable {
    pub __base: nsISupportsVTable,

    /* void loadPage (in nsISupports aPageDescriptor, in unsigned long aDisplayType); */
    pub loadPage: unsafe extern "C" fn (this: *const nsIWebPageDescriptor, aPageDescriptor: *const nsISupports, aDisplayType: libc::uint32_t) -> nsresult,

    /* readonly attribute nsISupports currentDescriptor; */
    pub get_currentDescriptor: unsafe extern "C" fn (this: *const nsIWebPageDescriptor, aCurrentDescriptor: *mut *const nsISupports) -> nsresult,

}


impl nsIWebPageDescriptor {
    /* void loadPage (in nsISupports aPageDescriptor, in unsigned long aDisplayType); */
    #[inline]
    pub unsafe fn loadPage(&self, aPageDescriptor: Option<&nsISupports>, aDisplayType: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).loadPage)(self as *const _, aPageDescriptor.map_or(::std::ptr::null(), |x| x as *const _), aDisplayType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsISupports currentDescriptor; */
    #[inline]
    pub unsafe fn get_currentDescriptor(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentDescriptor)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



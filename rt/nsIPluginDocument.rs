//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPluginDocument.idl
//


#[repr(C)]
pub struct nsIPluginDocument {
    vtable: *const nsIPluginDocumentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPluginDocument {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa93a0f0f, 0x24f0, 0x4206,
            [0xa2, 0x1b, 0x56, 0xa4, 0x3d, 0xcb, 0xdd, 0x88])
    }
}

unsafe impl RefCounted for nsIPluginDocument {
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
pub trait nsIPluginDocumentCoerce {
    fn coerce_from(v: &nsIPluginDocument) -> &Self;
}

impl nsIPluginDocumentCoerce for nsIPluginDocument {
    #[inline]
    fn coerce_from(v: &nsIPluginDocument) -> &Self {
        v
    }
}

impl nsIPluginDocument {
    #[inline]
    pub fn coerce<T: nsIPluginDocumentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPluginDocument {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPluginDocumentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPluginDocument) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPluginDocumentVTable {
    pub __base: nsISupportsVTable,

    /* void print (); */
    pub print: unsafe extern "C" fn (this: *const nsIPluginDocument) -> nsresult,

}


impl nsIPluginDocument {
    /* void print (); */
    #[inline]
    pub unsafe fn print(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).print)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



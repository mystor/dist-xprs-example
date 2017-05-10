//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClipboardDragDropHookList.idl
//


#[repr(C)]
pub struct nsIClipboardDragDropHookList {
    vtable: *const nsIClipboardDragDropHookListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIClipboardDragDropHookList {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x876a2015, 0x6b66, 0x11d7,
            [0x8f, 0x18, 0x00, 0x03, 0x93, 0x8a, 0x9d, 0x96])
    }
}

unsafe impl RefCounted for nsIClipboardDragDropHookList {
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
pub trait nsIClipboardDragDropHookListCoerce {
    fn coerce_from(v: &nsIClipboardDragDropHookList) -> &Self;
}

impl nsIClipboardDragDropHookListCoerce for nsIClipboardDragDropHookList {
    #[inline]
    fn coerce_from(v: &nsIClipboardDragDropHookList) -> &Self {
        v
    }
}

impl nsIClipboardDragDropHookList {
    #[inline]
    pub fn coerce<T: nsIClipboardDragDropHookListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIClipboardDragDropHookList {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIClipboardDragDropHookListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClipboardDragDropHookList) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIClipboardDragDropHookListVTable {
    pub __base: nsISupportsVTable,

    /* void addClipboardDragDropHooks (in nsIClipboardDragDropHooks aHooks); */
    pub addClipboardDragDropHooks: unsafe extern "C" fn (this: *const nsIClipboardDragDropHookList, aHooks: *const nsIClipboardDragDropHooks) -> nsresult,

    /* void removeClipboardDragDropHooks (in nsIClipboardDragDropHooks aHooks); */
    pub removeClipboardDragDropHooks: unsafe extern "C" fn (this: *const nsIClipboardDragDropHookList, aHooks: *const nsIClipboardDragDropHooks) -> nsresult,

    /* nsISimpleEnumerator getHookEnumerator (); */
    pub getHookEnumerator: unsafe extern "C" fn (this: *const nsIClipboardDragDropHookList, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsIClipboardDragDropHookList {
    /* void addClipboardDragDropHooks (in nsIClipboardDragDropHooks aHooks); */
    #[inline]
    pub unsafe fn addClipboardDragDropHooks(&self, aHooks: Option<&nsIClipboardDragDropHooks>) -> Result<(), nsresult> {

        match ((*self.vtable).addClipboardDragDropHooks)(self as *const _, aHooks.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeClipboardDragDropHooks (in nsIClipboardDragDropHooks aHooks); */
    #[inline]
    pub unsafe fn removeClipboardDragDropHooks(&self, aHooks: Option<&nsIClipboardDragDropHooks>) -> Result<(), nsresult> {

        match ((*self.vtable).removeClipboardDragDropHooks)(self as *const _, aHooks.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISimpleEnumerator getHookEnumerator (); */
    #[inline]
    pub unsafe fn getHookEnumerator(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getHookEnumerator)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}



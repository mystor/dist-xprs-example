//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISHContainer.idl
//


#[repr(C)]
pub struct nsISHContainer {
    vtable: *const nsISHContainerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISHContainer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x67dd0357, 0x8372, 0x4122,
            [0xbf, 0xf6, 0x21, 0x74, 0x35, 0xe8, 0xb7, 0xe4])
    }
}

unsafe impl RefCounted for nsISHContainer {
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
pub trait nsISHContainerCoerce {
    fn coerce_from(v: &nsISHContainer) -> &Self;
}

impl nsISHContainerCoerce for nsISHContainer {
    #[inline]
    fn coerce_from(v: &nsISHContainer) -> &Self {
        v
    }
}

impl nsISHContainer {
    #[inline]
    pub fn coerce<T: nsISHContainerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISHContainer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISHContainerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISHContainer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISHContainerVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long childCount; */
    pub get_childCount: unsafe extern "C" fn (this: *const nsISHContainer, aChildCount: *mut libc::int32_t) -> nsresult,

    /* void AddChild (in nsISHEntry child, in long offset); */
    pub AddChild: unsafe extern "C" fn (this: *const nsISHContainer, child: *const nsISHEntry, offset: libc::int32_t) -> nsresult,

    /* void RemoveChild (in nsISHEntry child); */
    pub RemoveChild: unsafe extern "C" fn (this: *const nsISHContainer, child: *const nsISHEntry) -> nsresult,

    /* nsISHEntry GetChildAt (in long index); */
    pub GetChildAt: unsafe extern "C" fn (this: *const nsISHContainer, index: libc::int32_t, _retval: *mut *const nsISHEntry) -> nsresult,

    /* void ReplaceChild (in nsISHEntry aNewChild); */
    pub ReplaceChild: unsafe extern "C" fn (this: *const nsISHContainer, aNewChild: *const nsISHEntry) -> nsresult,

}


impl nsISHContainer {
    /* readonly attribute long childCount; */
    #[inline]
    pub unsafe fn get_childCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_childCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void AddChild (in nsISHEntry child, in long offset); */
    #[inline]
    pub unsafe fn AddChild(&self, child: Option<&nsISHEntry>, offset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).AddChild)(self as *const _, child.map_or(::std::ptr::null(), |x| x as *const _), offset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void RemoveChild (in nsISHEntry child); */
    #[inline]
    pub unsafe fn RemoveChild(&self, child: Option<&nsISHEntry>) -> Result<(), nsresult> {

        match ((*self.vtable).RemoveChild)(self as *const _, child.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISHEntry GetChildAt (in long index); */
    #[inline]
    pub unsafe fn GetChildAt(&self, index: libc::int32_t) -> Result<Option<RefPtr<nsISHEntry>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetChildAt)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void ReplaceChild (in nsISHEntry aNewChild); */
    #[inline]
    pub unsafe fn ReplaceChild(&self, aNewChild: Option<&nsISHEntry>) -> Result<(), nsresult> {

        match ((*self.vtable).ReplaceChild)(self as *const _, aNewChild.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



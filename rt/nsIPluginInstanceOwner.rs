//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPluginInstanceOwner.idl
//


#[repr(C)]
pub struct nsIPluginInstanceOwner {
    vtable: *const nsIPluginInstanceOwnerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPluginInstanceOwner {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7d65452e, 0xc167, 0x4cba,
            [0xa0, 0xe3, 0xdd, 0xc6, 0x1b, 0xdd, 0xe8, 0xc3])
    }
}

unsafe impl RefCounted for nsIPluginInstanceOwner {
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
pub trait nsIPluginInstanceOwnerCoerce {
    fn coerce_from(v: &nsIPluginInstanceOwner) -> &Self;
}

impl nsIPluginInstanceOwnerCoerce for nsIPluginInstanceOwner {
    #[inline]
    fn coerce_from(v: &nsIPluginInstanceOwner) -> &Self {
        v
    }
}

impl nsIPluginInstanceOwner {
    #[inline]
    pub fn coerce<T: nsIPluginInstanceOwnerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPluginInstanceOwner {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPluginInstanceOwnerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPluginInstanceOwner) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPluginInstanceOwnerVTable {
    pub __base: nsISupportsVTable,

    /* void setInstance (in nsNPAPIPluginInstancePtr aInstance); */
    /// Unable to call function as its signature contains a non-rust type
    pub setInstance: *const ::libc::c_void,

    /* nsNPAPIPluginInstancePtr getInstance (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getInstance: *const ::libc::c_void,

    /* void getWindow (in NPWindowStarRef aWindow); */
    /// Unable to call function as its signature contains a non-rust type
    pub getWindow: *const ::libc::c_void,

    /* readonly attribute int32_t mode; */
    pub get_mode: unsafe extern "C" fn (this: *const nsIPluginInstanceOwner, aMode: *mut int32_t) -> nsresult,

    /* void createWidget (); */
    pub createWidget: unsafe extern "C" fn (this: *const nsIPluginInstanceOwner) -> nsresult,

    /* readonly attribute nsIDocument document; */
    pub get_document: unsafe extern "C" fn (this: *const nsIPluginInstanceOwner, aDocument: *mut *const nsIDocument) -> nsresult,

    /* void invalidateRect (in NPRectPtr aRect); */
    /// Unable to call function as its signature contains a non-rust type
    pub invalidateRect: *const ::libc::c_void,

    /* void invalidateRegion (in NPRegion aRegion); */
    /// Unable to call function as its signature contains a non-rust type
    pub invalidateRegion: *const ::libc::c_void,

    /* void redrawPlugin (); */
    pub redrawPlugin: unsafe extern "C" fn (this: *const nsIPluginInstanceOwner) -> nsresult,

    /* void getNetscapeWindow (in voidPtr aValue); */
    pub getNetscapeWindow: unsafe extern "C" fn (this: *const nsIPluginInstanceOwner, aValue: *const libc::c_void) -> nsresult,

    /* void setEventModel (in int32_t eventModel); */
    pub setEventModel: unsafe extern "C" fn (this: *const nsIPluginInstanceOwner, eventModel: int32_t) -> nsresult,

    /* void callSetWindow (); */
    pub callSetWindow: unsafe extern "C" fn (this: *const nsIPluginInstanceOwner) -> nsresult,

    /* double getContentsScaleFactor (); */
    pub getContentsScaleFactor: unsafe extern "C" fn (this: *const nsIPluginInstanceOwner, _retval: *mut libc::c_double) -> nsresult,

}


impl nsIPluginInstanceOwner {
    /* void setInstance (in nsNPAPIPluginInstancePtr aInstance); */


    /* nsNPAPIPluginInstancePtr getInstance (); */


    /* void getWindow (in NPWindowStarRef aWindow); */


    /* readonly attribute int32_t mode; */
    #[inline]
    pub unsafe fn get_mode(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_mode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void createWidget (); */
    #[inline]
    pub unsafe fn createWidget(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).createWidget)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDocument document; */
    #[inline]
    pub unsafe fn get_document(&self, ) -> Result<Option<RefPtr<nsIDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_document)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void invalidateRect (in NPRectPtr aRect); */


    /* void invalidateRegion (in NPRegion aRegion); */


    /* void redrawPlugin (); */
    #[inline]
    pub unsafe fn redrawPlugin(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).redrawPlugin)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getNetscapeWindow (in voidPtr aValue); */
    #[inline]
    pub unsafe fn getNetscapeWindow(&self, aValue: *const libc::c_void) -> Result<(), nsresult> {

        match ((*self.vtable).getNetscapeWindow)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setEventModel (in int32_t eventModel); */
    #[inline]
    pub unsafe fn setEventModel(&self, eventModel: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setEventModel)(self as *const _, eventModel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void callSetWindow (); */
    #[inline]
    pub unsafe fn callSetWindow(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).callSetWindow)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* double getContentsScaleFactor (); */
    #[inline]
    pub unsafe fn getContentsScaleFactor(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).getContentsScaleFactor)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



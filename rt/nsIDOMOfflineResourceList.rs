//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMOfflineResourceList.idl
//


pub mod nsIDOMOfflineResourceList_consts {
    pub const UNCACHED: i64 = 0;
    pub const IDLE: i64 = 1;
    pub const CHECKING: i64 = 2;
    pub const DOWNLOADING: i64 = 3;
    pub const UPDATEREADY: i64 = 4;
    pub const OBSOLETE: i64 = 5;
}


#[repr(C)]
pub struct nsIDOMOfflineResourceList {
    vtable: *const nsIDOMOfflineResourceListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMOfflineResourceList {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6044702d, 0xe4a9, 0x420c,
            [0xb7, 0x11, 0x55, 0x8b, 0x7d, 0x6a, 0x3b, 0x9f])
    }
}

unsafe impl RefCounted for nsIDOMOfflineResourceList {
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
pub trait nsIDOMOfflineResourceListCoerce {
    fn coerce_from(v: &nsIDOMOfflineResourceList) -> &Self;
}

impl nsIDOMOfflineResourceListCoerce for nsIDOMOfflineResourceList {
    #[inline]
    fn coerce_from(v: &nsIDOMOfflineResourceList) -> &Self {
        v
    }
}

impl nsIDOMOfflineResourceList {
    #[inline]
    pub fn coerce<T: nsIDOMOfflineResourceListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMOfflineResourceList {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMOfflineResourceListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMOfflineResourceList) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMOfflineResourceListVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsISupports mozItems; */
    pub get_mozItems: unsafe extern "C" fn (this: *const nsIDOMOfflineResourceList, aMozItems: *mut *const nsISupports) -> nsresult,

    /* boolean mozHasItem (in DOMString uri); */
    pub mozHasItem: unsafe extern "C" fn (this: *const nsIDOMOfflineResourceList, uri: *const nsAString, _retval: *mut bool) -> nsresult,

    /* readonly attribute unsigned long mozLength; */
    pub get_mozLength: unsafe extern "C" fn (this: *const nsIDOMOfflineResourceList, aMozLength: *mut libc::uint32_t) -> nsresult,

    /* DOMString mozItem (in unsigned long index); */
    pub mozItem: unsafe extern "C" fn (this: *const nsIDOMOfflineResourceList, index: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* void mozAdd (in DOMString uri); */
    pub mozAdd: unsafe extern "C" fn (this: *const nsIDOMOfflineResourceList, uri: *const nsAString) -> nsresult,

    /* void mozRemove (in DOMString uri); */
    pub mozRemove: unsafe extern "C" fn (this: *const nsIDOMOfflineResourceList, uri: *const nsAString) -> nsresult,

    /* readonly attribute unsigned short status; */
    pub get_status: unsafe extern "C" fn (this: *const nsIDOMOfflineResourceList, aStatus: *mut libc::uint16_t) -> nsresult,

    /* void update (); */
    pub update: unsafe extern "C" fn (this: *const nsIDOMOfflineResourceList) -> nsresult,

    /* void swapCache (); */
    pub swapCache: unsafe extern "C" fn (this: *const nsIDOMOfflineResourceList) -> nsresult,

    /* [implicit_jscontext] attribute jsval onchecking; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onchecking: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_onchecking: *const ::libc::c_void,

    /* [implicit_jscontext] attribute jsval onerror; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onerror: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_onerror: *const ::libc::c_void,

    /* [implicit_jscontext] attribute jsval onnoupdate; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onnoupdate: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_onnoupdate: *const ::libc::c_void,

    /* [implicit_jscontext] attribute jsval ondownloading; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_ondownloading: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_ondownloading: *const ::libc::c_void,

    /* [implicit_jscontext] attribute jsval onprogress; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onprogress: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_onprogress: *const ::libc::c_void,

    /* [implicit_jscontext] attribute jsval onupdateready; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onupdateready: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_onupdateready: *const ::libc::c_void,

    /* [implicit_jscontext] attribute jsval oncached; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_oncached: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_oncached: *const ::libc::c_void,

    /* [implicit_jscontext] attribute jsval onobsolete; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onobsolete: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_onobsolete: *const ::libc::c_void,

}


impl nsIDOMOfflineResourceList {
    /* readonly attribute nsISupports mozItems; */
    #[inline]
    pub unsafe fn get_mozItems(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_mozItems)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean mozHasItem (in DOMString uri); */
    #[inline]
    pub unsafe fn mozHasItem(&self, uri: &[u16]) -> Result<bool, nsresult> {
        let uri = nsString::from(uri);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).mozHasItem)(self as *const _, &*uri, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long mozLength; */
    #[inline]
    pub unsafe fn get_mozLength(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_mozLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* DOMString mozItem (in unsigned long index); */
    #[inline]
    pub unsafe fn mozItem(&self, index: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).mozItem)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void mozAdd (in DOMString uri); */
    #[inline]
    pub unsafe fn mozAdd(&self, uri: &[u16]) -> Result<(), nsresult> {
        let uri = nsString::from(uri);
        match ((*self.vtable).mozAdd)(self as *const _, &*uri) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void mozRemove (in DOMString uri); */
    #[inline]
    pub unsafe fn mozRemove(&self, uri: &[u16]) -> Result<(), nsresult> {
        let uri = nsString::from(uri);
        match ((*self.vtable).mozRemove)(self as *const _, &*uri) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned short status; */
    #[inline]
    pub unsafe fn get_status(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_status)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void update (); */
    #[inline]
    pub unsafe fn update(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).update)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void swapCache (); */
    #[inline]
    pub unsafe fn swapCache(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).swapCache)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] attribute jsval onchecking; */



    /* [implicit_jscontext] attribute jsval onerror; */



    /* [implicit_jscontext] attribute jsval onnoupdate; */



    /* [implicit_jscontext] attribute jsval ondownloading; */



    /* [implicit_jscontext] attribute jsval onprogress; */



    /* [implicit_jscontext] attribute jsval onupdateready; */



    /* [implicit_jscontext] attribute jsval oncached; */



    /* [implicit_jscontext] attribute jsval onobsolete; */



}



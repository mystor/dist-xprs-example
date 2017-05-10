//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIControllers.idl
//


#[repr(C)]
pub struct nsIControllers {
    vtable: *const nsIControllersVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIControllers {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf36e3ec1, 0x9197, 0x4ad8,
            [0x8d, 0x4c, 0xd3, 0xb1, 0x92, 0x7f, 0xd6, 0xdf])
    }
}

unsafe impl RefCounted for nsIControllers {
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
pub trait nsIControllersCoerce {
    fn coerce_from(v: &nsIControllers) -> &Self;
}

impl nsIControllersCoerce for nsIControllers {
    #[inline]
    fn coerce_from(v: &nsIControllers) -> &Self {
        v
    }
}

impl nsIControllers {
    #[inline]
    pub fn coerce<T: nsIControllersCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIControllers {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIControllersCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIControllers) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIControllersVTable {
    pub __base: nsISupportsVTable,

    /* nsIController getControllerForCommand (in string command); */
    pub getControllerForCommand: unsafe extern "C" fn (this: *const nsIControllers, command: *const libc::c_char, _retval: *mut *const nsIController) -> nsresult,

    /* void insertControllerAt (in unsigned long index, in nsIController controller); */
    pub insertControllerAt: unsafe extern "C" fn (this: *const nsIControllers, index: libc::uint32_t, controller: *const nsIController) -> nsresult,

    /* nsIController removeControllerAt (in unsigned long index); */
    pub removeControllerAt: unsafe extern "C" fn (this: *const nsIControllers, index: libc::uint32_t, _retval: *mut *const nsIController) -> nsresult,

    /* nsIController getControllerAt (in unsigned long index); */
    pub getControllerAt: unsafe extern "C" fn (this: *const nsIControllers, index: libc::uint32_t, _retval: *mut *const nsIController) -> nsresult,

    /* void appendController (in nsIController controller); */
    pub appendController: unsafe extern "C" fn (this: *const nsIControllers, controller: *const nsIController) -> nsresult,

    /* void removeController (in nsIController controller); */
    pub removeController: unsafe extern "C" fn (this: *const nsIControllers, controller: *const nsIController) -> nsresult,

    /* unsigned long getControllerId (in nsIController controller); */
    pub getControllerId: unsafe extern "C" fn (this: *const nsIControllers, controller: *const nsIController, _retval: *mut libc::uint32_t) -> nsresult,

    /* nsIController getControllerById (in unsigned long controllerID); */
    pub getControllerById: unsafe extern "C" fn (this: *const nsIControllers, controllerID: libc::uint32_t, _retval: *mut *const nsIController) -> nsresult,

    /* unsigned long getControllerCount (); */
    pub getControllerCount: unsafe extern "C" fn (this: *const nsIControllers, _retval: *mut libc::uint32_t) -> nsresult,

}


impl nsIControllers {
    /* nsIController getControllerForCommand (in string command); */
    #[inline]
    pub unsafe fn getControllerForCommand(&self, command: *const libc::c_char) -> Result<Option<RefPtr<nsIController>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getControllerForCommand)(self as *const _, command, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void insertControllerAt (in unsigned long index, in nsIController controller); */
    #[inline]
    pub unsafe fn insertControllerAt(&self, index: libc::uint32_t, controller: Option<&nsIController>) -> Result<(), nsresult> {

        match ((*self.vtable).insertControllerAt)(self as *const _, index, controller.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIController removeControllerAt (in unsigned long index); */
    #[inline]
    pub unsafe fn removeControllerAt(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIController>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).removeControllerAt)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIController getControllerAt (in unsigned long index); */
    #[inline]
    pub unsafe fn getControllerAt(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIController>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getControllerAt)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void appendController (in nsIController controller); */
    #[inline]
    pub unsafe fn appendController(&self, controller: Option<&nsIController>) -> Result<(), nsresult> {

        match ((*self.vtable).appendController)(self as *const _, controller.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeController (in nsIController controller); */
    #[inline]
    pub unsafe fn removeController(&self, controller: Option<&nsIController>) -> Result<(), nsresult> {

        match ((*self.vtable).removeController)(self as *const _, controller.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* unsigned long getControllerId (in nsIController controller); */
    #[inline]
    pub unsafe fn getControllerId(&self, controller: Option<&nsIController>) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getControllerId)(self as *const _, controller.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIController getControllerById (in unsigned long controllerID); */
    #[inline]
    pub unsafe fn getControllerById(&self, controllerID: libc::uint32_t) -> Result<Option<RefPtr<nsIController>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getControllerById)(self as *const _, controllerID, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* unsigned long getControllerCount (); */
    #[inline]
    pub unsafe fn getControllerCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getControllerCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



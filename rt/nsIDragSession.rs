//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDragSession.idl
//


#[repr(C)]
pub struct nsIDragSession {
    vtable: *const nsIDragSessionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDragSession {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x25bce737, 0x73f0, 0x43c7,
            [0xbc, 0x20, 0xc7, 0x10, 0x44, 0xa7, 0x3c, 0x5a])
    }
}

unsafe impl RefCounted for nsIDragSession {
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
pub trait nsIDragSessionCoerce {
    fn coerce_from(v: &nsIDragSession) -> &Self;
}

impl nsIDragSessionCoerce for nsIDragSession {
    #[inline]
    fn coerce_from(v: &nsIDragSession) -> &Self {
        v
    }
}

impl nsIDragSession {
    #[inline]
    pub fn coerce<T: nsIDragSessionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDragSession {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDragSessionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDragSession) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDragSessionVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean canDrop; */
    pub get_canDrop: unsafe extern "C" fn (this: *const nsIDragSession, aCanDrop: *mut bool) -> nsresult,
    pub set_canDrop: unsafe extern "C" fn (this: *const nsIDragSession, aCanDrop: bool) -> nsresult,

    /* attribute boolean onlyChromeDrop; */
    pub get_onlyChromeDrop: unsafe extern "C" fn (this: *const nsIDragSession, aOnlyChromeDrop: *mut bool) -> nsresult,
    pub set_onlyChromeDrop: unsafe extern "C" fn (this: *const nsIDragSession, aOnlyChromeDrop: bool) -> nsresult,

    /* attribute unsigned long dragAction; */
    pub get_dragAction: unsafe extern "C" fn (this: *const nsIDragSession, aDragAction: *mut libc::uint32_t) -> nsresult,
    pub set_dragAction: unsafe extern "C" fn (this: *const nsIDragSession, aDragAction: libc::uint32_t) -> nsresult,

    /* [noscript] attribute nsSize targetSize; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_targetSize: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_targetSize: *const ::libc::c_void,

    /* readonly attribute unsigned long numDropItems; */
    pub get_numDropItems: unsafe extern "C" fn (this: *const nsIDragSession, aNumDropItems: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIDOMDocument sourceDocument; */
    pub get_sourceDocument: unsafe extern "C" fn (this: *const nsIDragSession, aSourceDocument: *mut *const nsIDOMDocument) -> nsresult,

    /* readonly attribute nsIDOMNode sourceNode; */
    pub get_sourceNode: unsafe extern "C" fn (this: *const nsIDragSession, aSourceNode: *mut *const nsIDOMNode) -> nsresult,

    /* attribute nsIDOMDataTransfer dataTransfer; */
    pub get_dataTransfer: unsafe extern "C" fn (this: *const nsIDragSession, aDataTransfer: *mut *const nsIDOMDataTransfer) -> nsresult,
    pub set_dataTransfer: unsafe extern "C" fn (this: *const nsIDragSession, aDataTransfer: *const nsIDOMDataTransfer) -> nsresult,

    /* void getData (in nsITransferable aTransferable, in unsigned long aItemIndex); */
    pub getData: unsafe extern "C" fn (this: *const nsIDragSession, aTransferable: *const nsITransferable, aItemIndex: libc::uint32_t) -> nsresult,

    /* boolean isDataFlavorSupported (in string aDataFlavor); */
    pub isDataFlavorSupported: unsafe extern "C" fn (this: *const nsIDragSession, aDataFlavor: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* void userCancelled (); */
    pub userCancelled: unsafe extern "C" fn (this: *const nsIDragSession) -> nsresult,

    /* void dragEventDispatchedToChildProcess (); */
    pub dragEventDispatchedToChildProcess: unsafe extern "C" fn (this: *const nsIDragSession) -> nsresult,

    /* void updateDragEffect (); */
    pub updateDragEffect: unsafe extern "C" fn (this: *const nsIDragSession) -> nsresult,

    /* void updateDragImage (in nsIDOMNode aImage, in long aImageX, in long aImageY); */
    pub updateDragImage: unsafe extern "C" fn (this: *const nsIDragSession, aImage: *const nsIDOMNode, aImageX: libc::int32_t, aImageY: libc::int32_t) -> nsresult,

}


impl nsIDragSession {
    /* attribute boolean canDrop; */
    #[inline]
    pub unsafe fn get_canDrop(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canDrop)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_canDrop(&self, aCanDrop: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_canDrop)(self as *const _, aCanDrop) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean onlyChromeDrop; */
    #[inline]
    pub unsafe fn get_onlyChromeDrop(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_onlyChromeDrop)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_onlyChromeDrop(&self, aOnlyChromeDrop: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_onlyChromeDrop)(self as *const _, aOnlyChromeDrop) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long dragAction; */
    #[inline]
    pub unsafe fn get_dragAction(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_dragAction)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_dragAction(&self, aDragAction: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_dragAction)(self as *const _, aDragAction) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute nsSize targetSize; */



    /* readonly attribute unsigned long numDropItems; */
    #[inline]
    pub unsafe fn get_numDropItems(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_numDropItems)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMDocument sourceDocument; */
    #[inline]
    pub unsafe fn get_sourceDocument(&self, ) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sourceDocument)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMNode sourceNode; */
    #[inline]
    pub unsafe fn get_sourceNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sourceNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsIDOMDataTransfer dataTransfer; */
    #[inline]
    pub unsafe fn get_dataTransfer(&self, ) -> Result<Option<RefPtr<nsIDOMDataTransfer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_dataTransfer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_dataTransfer(&self, aDataTransfer: Option<&nsIDOMDataTransfer>) -> Result<(), nsresult> {

        match ((*self.vtable).set_dataTransfer)(self as *const _, aDataTransfer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getData (in nsITransferable aTransferable, in unsigned long aItemIndex); */
    #[inline]
    pub unsafe fn getData(&self, aTransferable: Option<&nsITransferable>, aItemIndex: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).getData)(self as *const _, aTransferable.map_or(::std::ptr::null(), |x| x as *const _), aItemIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isDataFlavorSupported (in string aDataFlavor); */
    #[inline]
    pub unsafe fn isDataFlavorSupported(&self, aDataFlavor: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isDataFlavorSupported)(self as *const _, aDataFlavor, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void userCancelled (); */
    #[inline]
    pub unsafe fn userCancelled(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).userCancelled)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dragEventDispatchedToChildProcess (); */
    #[inline]
    pub unsafe fn dragEventDispatchedToChildProcess(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).dragEventDispatchedToChildProcess)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void updateDragEffect (); */
    #[inline]
    pub unsafe fn updateDragEffect(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).updateDragEffect)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void updateDragImage (in nsIDOMNode aImage, in long aImageX, in long aImageY); */
    #[inline]
    pub unsafe fn updateDragImage(&self, aImage: Option<&nsIDOMNode>, aImageX: libc::int32_t, aImageY: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).updateDragImage)(self as *const _, aImage.map_or(::std::ptr::null(), |x| x as *const _), aImageX, aImageY) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



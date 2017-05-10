//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDataTransfer.idl
//


#[repr(C)]
pub struct nsIDOMDataTransfer {
    vtable: *const nsIDOMDataTransferVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMDataTransfer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x655078bf, 0x1675, 0x4aa0,
            [0xa4, 0x8d, 0xa1, 0x33, 0xe8, 0x64, 0xce, 0x57])
    }
}

unsafe impl RefCounted for nsIDOMDataTransfer {
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
pub trait nsIDOMDataTransferCoerce {
    fn coerce_from(v: &nsIDOMDataTransfer) -> &Self;
}

impl nsIDOMDataTransferCoerce for nsIDOMDataTransfer {
    #[inline]
    fn coerce_from(v: &nsIDOMDataTransfer) -> &Self {
        v
    }
}

impl nsIDOMDataTransfer {
    #[inline]
    pub fn coerce<T: nsIDOMDataTransferCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMDataTransfer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMDataTransferCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMDataTransfer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMDataTransferVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString dropEffect; */
    pub get_dropEffect: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aDropEffect: *mut nsAString) -> nsresult,
    pub set_dropEffect: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aDropEffect: *const nsAString) -> nsresult,

    /* attribute DOMString effectAllowed; */
    pub get_effectAllowed: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aEffectAllowed: *mut nsAString) -> nsresult,
    pub set_effectAllowed: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aEffectAllowed: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMFileList files; */
    pub get_files: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aFiles: *mut *const nsIDOMFileList) -> nsresult,

    /* void setDragImage (in nsIDOMElement image, in long x, in long y); */
    pub setDragImage: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, image: *const nsIDOMElement, x: libc::int32_t, y: libc::int32_t) -> nsresult,

    /* void addElement (in nsIDOMElement element); */
    pub addElement: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, element: *const nsIDOMElement) -> nsresult,

    /* readonly attribute unsigned long mozItemCount; */
    pub get_mozItemCount: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aMozItemCount: *mut libc::uint32_t) -> nsresult,

    /* attribute DOMString mozCursor; */
    pub get_mozCursor: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aMozCursor: *mut nsAString) -> nsresult,
    pub set_mozCursor: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aMozCursor: *const nsAString) -> nsresult,

    /* readonly attribute boolean mozUserCancelled; */
    pub get_mozUserCancelled: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aMozUserCancelled: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMNode mozSourceNode; */
    pub get_mozSourceNode: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aMozSourceNode: *mut *const nsIDOMNode) -> nsresult,

    /* [noscript] attribute unsigned long dropEffectInt; */
    pub get_dropEffectInt: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aDropEffectInt: *mut libc::uint32_t) -> nsresult,
    pub set_dropEffectInt: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aDropEffectInt: libc::uint32_t) -> nsresult,

    /* [noscript] attribute unsigned long effectAllowedInt; */
    pub get_effectAllowedInt: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aEffectAllowedInt: *mut libc::uint32_t) -> nsresult,
    pub set_effectAllowedInt: unsafe extern "C" fn (this: *const nsIDOMDataTransfer, aEffectAllowedInt: libc::uint32_t) -> nsresult,

}


impl nsIDOMDataTransfer {
    /* attribute DOMString dropEffect; */
    #[inline]
    pub unsafe fn get_dropEffect(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_dropEffect)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_dropEffect(&self, aDropEffect: &[u16]) -> Result<(), nsresult> {
        let aDropEffect = nsString::from(aDropEffect);
        match ((*self.vtable).set_dropEffect)(self as *const _, &*aDropEffect) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString effectAllowed; */
    #[inline]
    pub unsafe fn get_effectAllowed(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_effectAllowed)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_effectAllowed(&self, aEffectAllowed: &[u16]) -> Result<(), nsresult> {
        let aEffectAllowed = nsString::from(aEffectAllowed);
        match ((*self.vtable).set_effectAllowed)(self as *const _, &*aEffectAllowed) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMFileList files; */
    #[inline]
    pub unsafe fn get_files(&self, ) -> Result<Option<RefPtr<nsIDOMFileList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_files)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setDragImage (in nsIDOMElement image, in long x, in long y); */
    #[inline]
    pub unsafe fn setDragImage(&self, image: Option<&nsIDOMElement>, x: libc::int32_t, y: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setDragImage)(self as *const _, image.map_or(::std::ptr::null(), |x| x as *const _), x, y) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addElement (in nsIDOMElement element); */
    #[inline]
    pub unsafe fn addElement(&self, element: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).addElement)(self as *const _, element.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long mozItemCount; */
    #[inline]
    pub unsafe fn get_mozItemCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_mozItemCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute DOMString mozCursor; */
    #[inline]
    pub unsafe fn get_mozCursor(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_mozCursor)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_mozCursor(&self, aMozCursor: &[u16]) -> Result<(), nsresult> {
        let aMozCursor = nsString::from(aMozCursor);
        match ((*self.vtable).set_mozCursor)(self as *const _, &*aMozCursor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean mozUserCancelled; */
    #[inline]
    pub unsafe fn get_mozUserCancelled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mozUserCancelled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMNode mozSourceNode; */
    #[inline]
    pub unsafe fn get_mozSourceNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_mozSourceNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] attribute unsigned long dropEffectInt; */
    #[inline]
    pub unsafe fn get_dropEffectInt(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_dropEffectInt)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_dropEffectInt(&self, aDropEffectInt: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_dropEffectInt)(self as *const _, aDropEffectInt) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute unsigned long effectAllowedInt; */
    #[inline]
    pub unsafe fn get_effectAllowedInt(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_effectAllowedInt)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_effectAllowedInt(&self, aEffectAllowedInt: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_effectAllowedInt)(self as *const _, aEffectAllowedInt) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



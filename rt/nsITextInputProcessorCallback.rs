//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITextInputProcessorCallback.idl
//


#[repr(C)]
pub struct nsITextInputProcessorNotification {
    vtable: *const nsITextInputProcessorNotificationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITextInputProcessorNotification {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc0ce1add, 0x82bb, 0x45ab,
            [0xb9, 0x9a, 0x42, 0xcf, 0xba, 0x7f, 0xd5, 0xd7])
    }
}

unsafe impl RefCounted for nsITextInputProcessorNotification {
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
pub trait nsITextInputProcessorNotificationCoerce {
    fn coerce_from(v: &nsITextInputProcessorNotification) -> &Self;
}

impl nsITextInputProcessorNotificationCoerce for nsITextInputProcessorNotification {
    #[inline]
    fn coerce_from(v: &nsITextInputProcessorNotification) -> &Self {
        v
    }
}

impl nsITextInputProcessorNotification {
    #[inline]
    pub fn coerce<T: nsITextInputProcessorNotificationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITextInputProcessorNotification {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITextInputProcessorNotificationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITextInputProcessorNotification) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITextInputProcessorNotificationVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aType: *mut nsACString) -> nsresult,

    /* readonly attribute unsigned long offset; */
    pub get_offset: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aOffset: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute AString text; */
    pub get_text: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aText: *mut nsAString) -> nsresult,

    /* readonly attribute boolean collapsed; */
    pub get_collapsed: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aCollapsed: *mut bool) -> nsresult,

    /* readonly attribute uint32_t length; */
    pub get_length: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aLength: *mut uint32_t) -> nsresult,

    /* readonly attribute boolean reversed; */
    pub get_reversed: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aReversed: *mut bool) -> nsresult,

    /* readonly attribute ACString writingMode; */
    pub get_writingMode: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aWritingMode: *mut nsACString) -> nsresult,

    /* readonly attribute boolean causedByComposition; */
    pub get_causedByComposition: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aCausedByComposition: *mut bool) -> nsresult,

    /* readonly attribute boolean causedBySelectionEvent; */
    pub get_causedBySelectionEvent: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aCausedBySelectionEvent: *mut bool) -> nsresult,

    /* readonly attribute boolean occurredDuringComposition; */
    pub get_occurredDuringComposition: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aOccurredDuringComposition: *mut bool) -> nsresult,

    /* readonly attribute unsigned long removedLength; */
    pub get_removedLength: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aRemovedLength: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long addedLength; */
    pub get_addedLength: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aAddedLength: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute boolean causedOnlyByComposition; */
    pub get_causedOnlyByComposition: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aCausedOnlyByComposition: *mut bool) -> nsresult,

    /* readonly attribute boolean includingChangesDuringComposition; */
    pub get_includingChangesDuringComposition: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aIncludingChangesDuringComposition: *mut bool) -> nsresult,

    /* readonly attribute boolean includingChangesWithoutComposition; */
    pub get_includingChangesWithoutComposition: unsafe extern "C" fn (this: *const nsITextInputProcessorNotification, aIncludingChangesWithoutComposition: *mut bool) -> nsresult,

}


impl nsITextInputProcessorNotification {
    /* readonly attribute ACString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long offset; */
    #[inline]
    pub unsafe fn get_offset(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_offset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString text; */
    #[inline]
    pub unsafe fn get_text(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_text)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean collapsed; */
    #[inline]
    pub unsafe fn get_collapsed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_collapsed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean reversed; */
    #[inline]
    pub unsafe fn get_reversed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_reversed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString writingMode; */
    #[inline]
    pub unsafe fn get_writingMode(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_writingMode)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean causedByComposition; */
    #[inline]
    pub unsafe fn get_causedByComposition(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_causedByComposition)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean causedBySelectionEvent; */
    #[inline]
    pub unsafe fn get_causedBySelectionEvent(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_causedBySelectionEvent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean occurredDuringComposition; */
    #[inline]
    pub unsafe fn get_occurredDuringComposition(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_occurredDuringComposition)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long removedLength; */
    #[inline]
    pub unsafe fn get_removedLength(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_removedLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long addedLength; */
    #[inline]
    pub unsafe fn get_addedLength(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_addedLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean causedOnlyByComposition; */
    #[inline]
    pub unsafe fn get_causedOnlyByComposition(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_causedOnlyByComposition)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean includingChangesDuringComposition; */
    #[inline]
    pub unsafe fn get_includingChangesDuringComposition(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_includingChangesDuringComposition)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean includingChangesWithoutComposition; */
    #[inline]
    pub unsafe fn get_includingChangesWithoutComposition(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_includingChangesWithoutComposition)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsITextInputProcessorCallback {
    vtable: *const nsITextInputProcessorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITextInputProcessorCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x23d5f242, 0xadb5, 0x46f1,
            [0x87, 0x66, 0x90, 0xd1, 0xbf, 0x03, 0x83, 0xdf])
    }
}

unsafe impl RefCounted for nsITextInputProcessorCallback {
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
pub trait nsITextInputProcessorCallbackCoerce {
    fn coerce_from(v: &nsITextInputProcessorCallback) -> &Self;
}

impl nsITextInputProcessorCallbackCoerce for nsITextInputProcessorCallback {
    #[inline]
    fn coerce_from(v: &nsITextInputProcessorCallback) -> &Self {
        v
    }
}

impl nsITextInputProcessorCallback {
    #[inline]
    pub fn coerce<T: nsITextInputProcessorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITextInputProcessorCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITextInputProcessorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITextInputProcessorCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITextInputProcessorCallbackVTable {
    pub __base: nsISupportsVTable,

    /* boolean onNotify (in nsITextInputProcessor aTextInputProcessor, in nsITextInputProcessorNotification aNotification); */
    pub onNotify: unsafe extern "C" fn (this: *const nsITextInputProcessorCallback, aTextInputProcessor: *const nsITextInputProcessor, aNotification: *const nsITextInputProcessorNotification, _retval: *mut bool) -> nsresult,

}


impl nsITextInputProcessorCallback {
    /* boolean onNotify (in nsITextInputProcessor aTextInputProcessor, in nsITextInputProcessorNotification aNotification); */
    #[inline]
    pub unsafe fn onNotify(&self, aTextInputProcessor: Option<&nsITextInputProcessor>, aNotification: Option<&nsITextInputProcessorNotification>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).onNotify)(self as *const _, aTextInputProcessor.map_or(::std::ptr::null(), |x| x as *const _), aNotification.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}



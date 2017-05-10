//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFilePicker.idl
//


#[repr(C)]
pub struct nsIFilePickerShownCallback {
    vtable: *const nsIFilePickerShownCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFilePickerShownCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0d79adad, 0xb244, 0x49a5,
            [0x99, 0x97, 0x2a, 0x8c, 0xad, 0x93, 0xfc, 0x44])
    }
}

unsafe impl RefCounted for nsIFilePickerShownCallback {
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
pub trait nsIFilePickerShownCallbackCoerce {
    fn coerce_from(v: &nsIFilePickerShownCallback) -> &Self;
}

impl nsIFilePickerShownCallbackCoerce for nsIFilePickerShownCallback {
    #[inline]
    fn coerce_from(v: &nsIFilePickerShownCallback) -> &Self {
        v
    }
}

impl nsIFilePickerShownCallback {
    #[inline]
    pub fn coerce<T: nsIFilePickerShownCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFilePickerShownCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFilePickerShownCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFilePickerShownCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFilePickerShownCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void done (in short aResult); */
    pub done: unsafe extern "C" fn (this: *const nsIFilePickerShownCallback, aResult: libc::int16_t) -> nsresult,

}


impl nsIFilePickerShownCallback {
    /* void done (in short aResult); */
    #[inline]
    pub unsafe fn done(&self, aResult: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).done)(self as *const _, aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIFilePicker_consts {
    pub const modeOpen: i64 = 0;
    pub const modeSave: i64 = 1;
    pub const modeGetFolder: i64 = 2;
    pub const modeOpenMultiple: i64 = 3;
    pub const returnOK: i64 = 0;
    pub const returnCancel: i64 = 1;
    pub const returnReplace: i64 = 2;
    pub const filterAll: i64 = 1;
    pub const filterHTML: i64 = 2;
    pub const filterText: i64 = 4;
    pub const filterImages: i64 = 8;
    pub const filterXML: i64 = 16;
    pub const filterXUL: i64 = 32;
    pub const filterApps: i64 = 64;
    pub const filterAllowURLs: i64 = 128;
    pub const filterAudio: i64 = 256;
    pub const filterVideo: i64 = 512;
}


#[repr(C)]
pub struct nsIFilePicker {
    vtable: *const nsIFilePickerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFilePicker {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9285b984, 0x02d3, 0x46b4,
            [0x95, 0x14, 0x7d, 0xa8, 0xc4, 0x71, 0xa7, 0x47])
    }
}

unsafe impl RefCounted for nsIFilePicker {
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
pub trait nsIFilePickerCoerce {
    fn coerce_from(v: &nsIFilePicker) -> &Self;
}

impl nsIFilePickerCoerce for nsIFilePicker {
    #[inline]
    fn coerce_from(v: &nsIFilePicker) -> &Self {
        v
    }
}

impl nsIFilePicker {
    #[inline]
    pub fn coerce<T: nsIFilePickerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFilePicker {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFilePickerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFilePicker) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFilePickerVTable {
    pub __base: nsISupportsVTable,

    /* void init (in mozIDOMWindowProxy parent, in AString title, in short mode); */
    pub init: unsafe extern "C" fn (this: *const nsIFilePicker, parent: *const mozIDOMWindowProxy, title: *const nsAString, mode: libc::int16_t) -> nsresult,

    /* void appendFilters (in long filterMask); */
    pub appendFilters: unsafe extern "C" fn (this: *const nsIFilePicker, filterMask: libc::int32_t) -> nsresult,

    /* void appendFilter (in AString title, in AString filter); */
    pub appendFilter: unsafe extern "C" fn (this: *const nsIFilePicker, title: *const nsAString, filter: *const nsAString) -> nsresult,

    /* attribute AString defaultString; */
    pub get_defaultString: unsafe extern "C" fn (this: *const nsIFilePicker, aDefaultString: *mut nsAString) -> nsresult,
    pub set_defaultString: unsafe extern "C" fn (this: *const nsIFilePicker, aDefaultString: *const nsAString) -> nsresult,

    /* attribute AString defaultExtension; */
    pub get_defaultExtension: unsafe extern "C" fn (this: *const nsIFilePicker, aDefaultExtension: *mut nsAString) -> nsresult,
    pub set_defaultExtension: unsafe extern "C" fn (this: *const nsIFilePicker, aDefaultExtension: *const nsAString) -> nsresult,

    /* attribute long filterIndex; */
    pub get_filterIndex: unsafe extern "C" fn (this: *const nsIFilePicker, aFilterIndex: *mut libc::int32_t) -> nsresult,
    pub set_filterIndex: unsafe extern "C" fn (this: *const nsIFilePicker, aFilterIndex: libc::int32_t) -> nsresult,

    /* attribute nsIFile displayDirectory; */
    pub get_displayDirectory: unsafe extern "C" fn (this: *const nsIFilePicker, aDisplayDirectory: *mut *const nsIFile) -> nsresult,
    pub set_displayDirectory: unsafe extern "C" fn (this: *const nsIFilePicker, aDisplayDirectory: *const nsIFile) -> nsresult,

    /* attribute AString displaySpecialDirectory; */
    pub get_displaySpecialDirectory: unsafe extern "C" fn (this: *const nsIFilePicker, aDisplaySpecialDirectory: *mut nsAString) -> nsresult,
    pub set_displaySpecialDirectory: unsafe extern "C" fn (this: *const nsIFilePicker, aDisplaySpecialDirectory: *const nsAString) -> nsresult,

    /* readonly attribute nsIFile file; */
    pub get_file: unsafe extern "C" fn (this: *const nsIFilePicker, aFile: *mut *const nsIFile) -> nsresult,

    /* readonly attribute nsIURI fileURL; */
    pub get_fileURL: unsafe extern "C" fn (this: *const nsIFilePicker, aFileURL: *mut *const nsIURI) -> nsresult,

    /* readonly attribute nsISimpleEnumerator files; */
    pub get_files: unsafe extern "C" fn (this: *const nsIFilePicker, aFiles: *mut *const nsISimpleEnumerator) -> nsresult,

    /* readonly attribute nsISupports domFileOrDirectory; */
    pub get_domFileOrDirectory: unsafe extern "C" fn (this: *const nsIFilePicker, aDomFileOrDirectory: *mut *const nsISupports) -> nsresult,

    /* readonly attribute nsISimpleEnumerator domFileOrDirectoryEnumerator; */
    pub get_domFileOrDirectoryEnumerator: unsafe extern "C" fn (this: *const nsIFilePicker, aDomFileOrDirectoryEnumerator: *mut *const nsISimpleEnumerator) -> nsresult,

    /* attribute boolean addToRecentDocs; */
    pub get_addToRecentDocs: unsafe extern "C" fn (this: *const nsIFilePicker, aAddToRecentDocs: *mut bool) -> nsresult,
    pub set_addToRecentDocs: unsafe extern "C" fn (this: *const nsIFilePicker, aAddToRecentDocs: bool) -> nsresult,

    /* [deprecated] short show (); */
    pub show: unsafe extern "C" fn (this: *const nsIFilePicker, _retval: *mut libc::int16_t) -> nsresult,

    /* void open (in nsIFilePickerShownCallback aFilePickerShownCallback); */
    pub open: unsafe extern "C" fn (this: *const nsIFilePicker, aFilePickerShownCallback: *const nsIFilePickerShownCallback) -> nsresult,

    /* readonly attribute short mode; */
    pub get_mode: unsafe extern "C" fn (this: *const nsIFilePicker, aMode: *mut libc::int16_t) -> nsresult,

    /* attribute AString okButtonLabel; */
    pub get_okButtonLabel: unsafe extern "C" fn (this: *const nsIFilePicker, aOkButtonLabel: *mut nsAString) -> nsresult,
    pub set_okButtonLabel: unsafe extern "C" fn (this: *const nsIFilePicker, aOkButtonLabel: *const nsAString) -> nsresult,

}


impl nsIFilePicker {
    /* void init (in mozIDOMWindowProxy parent, in AString title, in short mode); */
    #[inline]
    pub unsafe fn init(&self, parent: Option<&mozIDOMWindowProxy>, title: &[u16], mode: libc::int16_t) -> Result<(), nsresult> {
        let title = nsString::from(title);
        match ((*self.vtable).init)(self as *const _, parent.map_or(::std::ptr::null(), |x| x as *const _), &*title, mode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void appendFilters (in long filterMask); */
    #[inline]
    pub unsafe fn appendFilters(&self, filterMask: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).appendFilters)(self as *const _, filterMask) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void appendFilter (in AString title, in AString filter); */
    #[inline]
    pub unsafe fn appendFilter(&self, title: &[u16], filter: &[u16]) -> Result<(), nsresult> {
        let title = nsString::from(title);
        let filter = nsString::from(filter);
        match ((*self.vtable).appendFilter)(self as *const _, &*title, &*filter) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString defaultString; */
    #[inline]
    pub unsafe fn get_defaultString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_defaultString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_defaultString(&self, aDefaultString: &[u16]) -> Result<(), nsresult> {
        let aDefaultString = nsString::from(aDefaultString);
        match ((*self.vtable).set_defaultString)(self as *const _, &*aDefaultString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString defaultExtension; */
    #[inline]
    pub unsafe fn get_defaultExtension(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_defaultExtension)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_defaultExtension(&self, aDefaultExtension: &[u16]) -> Result<(), nsresult> {
        let aDefaultExtension = nsString::from(aDefaultExtension);
        match ((*self.vtable).set_defaultExtension)(self as *const _, &*aDefaultExtension) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long filterIndex; */
    #[inline]
    pub unsafe fn get_filterIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_filterIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_filterIndex(&self, aFilterIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_filterIndex)(self as *const _, aFilterIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIFile displayDirectory; */
    #[inline]
    pub unsafe fn get_displayDirectory(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_displayDirectory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_displayDirectory(&self, aDisplayDirectory: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).set_displayDirectory)(self as *const _, aDisplayDirectory.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString displaySpecialDirectory; */
    #[inline]
    pub unsafe fn get_displaySpecialDirectory(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_displaySpecialDirectory)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_displaySpecialDirectory(&self, aDisplaySpecialDirectory: &[u16]) -> Result<(), nsresult> {
        let aDisplaySpecialDirectory = nsString::from(aDisplaySpecialDirectory);
        match ((*self.vtable).set_displaySpecialDirectory)(self as *const _, &*aDisplaySpecialDirectory) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIFile file; */
    #[inline]
    pub unsafe fn get_file(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_file)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI fileURL; */
    #[inline]
    pub unsafe fn get_fileURL(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_fileURL)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsISimpleEnumerator files; */
    #[inline]
    pub unsafe fn get_files(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_files)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsISupports domFileOrDirectory; */
    #[inline]
    pub unsafe fn get_domFileOrDirectory(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_domFileOrDirectory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsISimpleEnumerator domFileOrDirectoryEnumerator; */
    #[inline]
    pub unsafe fn get_domFileOrDirectoryEnumerator(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_domFileOrDirectoryEnumerator)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute boolean addToRecentDocs; */
    #[inline]
    pub unsafe fn get_addToRecentDocs(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_addToRecentDocs)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_addToRecentDocs(&self, aAddToRecentDocs: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_addToRecentDocs)(self as *const _, aAddToRecentDocs) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [deprecated] short show (); */
    #[inline]
    pub unsafe fn show(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).show)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void open (in nsIFilePickerShownCallback aFilePickerShownCallback); */
    #[inline]
    pub unsafe fn open(&self, aFilePickerShownCallback: Option<&nsIFilePickerShownCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).open)(self as *const _, aFilePickerShownCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute short mode; */
    #[inline]
    pub unsafe fn get_mode(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_mode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute AString okButtonLabel; */
    #[inline]
    pub unsafe fn get_okButtonLabel(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_okButtonLabel)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_okButtonLabel(&self, aOkButtonLabel: &[u16]) -> Result<(), nsresult> {
        let aOkButtonLabel = nsString::from(aOkButtonLabel);
        match ((*self.vtable).set_okButtonLabel)(self as *const _, &*aOkButtonLabel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



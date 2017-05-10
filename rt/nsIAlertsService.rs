//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAlertsService.idl
//


#[repr(C)]
pub struct nsIAlertNotificationImageListener {
    vtable: *const nsIAlertNotificationImageListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAlertNotificationImageListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa71a637d, 0xde1d, 0x47c6,
            [0xa8, 0xd2, 0xc6, 0x0b, 0x25, 0x96, 0xf4, 0x71])
    }
}

unsafe impl RefCounted for nsIAlertNotificationImageListener {
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
pub trait nsIAlertNotificationImageListenerCoerce {
    fn coerce_from(v: &nsIAlertNotificationImageListener) -> &Self;
}

impl nsIAlertNotificationImageListenerCoerce for nsIAlertNotificationImageListener {
    #[inline]
    fn coerce_from(v: &nsIAlertNotificationImageListener) -> &Self {
        v
    }
}

impl nsIAlertNotificationImageListener {
    #[inline]
    pub fn coerce<T: nsIAlertNotificationImageListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAlertNotificationImageListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAlertNotificationImageListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAlertNotificationImageListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAlertNotificationImageListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onImageReady (in nsISupports aUserData, in imgIRequest aRequest); */
    pub onImageReady: unsafe extern "C" fn (this: *const nsIAlertNotificationImageListener, aUserData: *const nsISupports, aRequest: *const imgIRequest) -> nsresult,

    /* void onImageMissing (in nsISupports aUserData); */
    pub onImageMissing: unsafe extern "C" fn (this: *const nsIAlertNotificationImageListener, aUserData: *const nsISupports) -> nsresult,

}


impl nsIAlertNotificationImageListener {
    /* void onImageReady (in nsISupports aUserData, in imgIRequest aRequest); */
    #[inline]
    pub unsafe fn onImageReady(&self, aUserData: Option<&nsISupports>, aRequest: Option<&imgIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).onImageReady)(self as *const _, aUserData.map_or(::std::ptr::null(), |x| x as *const _), aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onImageMissing (in nsISupports aUserData); */
    #[inline]
    pub unsafe fn onImageMissing(&self, aUserData: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).onImageMissing)(self as *const _, aUserData.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIAlertNotification {
    vtable: *const nsIAlertNotificationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAlertNotification {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcf2e4cb6, 0x4b8f, 0x4eca,
            [0xae, 0xa9, 0xd5, 0x1a, 0x8f, 0x9f, 0x7a, 0x50])
    }
}

unsafe impl RefCounted for nsIAlertNotification {
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
pub trait nsIAlertNotificationCoerce {
    fn coerce_from(v: &nsIAlertNotification) -> &Self;
}

impl nsIAlertNotificationCoerce for nsIAlertNotification {
    #[inline]
    fn coerce_from(v: &nsIAlertNotification) -> &Self {
        v
    }
}

impl nsIAlertNotification {
    #[inline]
    pub fn coerce<T: nsIAlertNotificationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAlertNotification {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAlertNotificationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAlertNotification) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAlertNotificationVTable {
    pub __base: nsISupportsVTable,

    /* void init ([optional] in AString aName, [optional] in AString aImageURL, [optional] in AString aTitle, [optional] in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction); */
    pub init: unsafe extern "C" fn (this: *const nsIAlertNotification, aName: *const nsAString, aImageURL: *const nsAString, aTitle: *const nsAString, aText: *const nsAString, aTextClickable: bool, aCookie: *const nsAString, aDir: *const nsAString, aLang: *const nsAString, aData: *const nsAString, aPrincipal: *const nsIPrincipal, aInPrivateBrowsing: bool, aRequireInteraction: bool) -> nsresult,

    /* readonly attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIAlertNotification, aName: *mut nsAString) -> nsresult,

    /* readonly attribute AString imageURL; */
    pub get_imageURL: unsafe extern "C" fn (this: *const nsIAlertNotification, aImageURL: *mut nsAString) -> nsresult,

    /* readonly attribute AString title; */
    pub get_title: unsafe extern "C" fn (this: *const nsIAlertNotification, aTitle: *mut nsAString) -> nsresult,

    /* readonly attribute AString text; */
    pub get_text: unsafe extern "C" fn (this: *const nsIAlertNotification, aText: *mut nsAString) -> nsresult,

    /* readonly attribute boolean textClickable; */
    pub get_textClickable: unsafe extern "C" fn (this: *const nsIAlertNotification, aTextClickable: *mut bool) -> nsresult,

    /* readonly attribute AString cookie; */
    pub get_cookie: unsafe extern "C" fn (this: *const nsIAlertNotification, aCookie: *mut nsAString) -> nsresult,

    /* readonly attribute AString dir; */
    pub get_dir: unsafe extern "C" fn (this: *const nsIAlertNotification, aDir: *mut nsAString) -> nsresult,

    /* readonly attribute AString lang; */
    pub get_lang: unsafe extern "C" fn (this: *const nsIAlertNotification, aLang: *mut nsAString) -> nsresult,

    /* readonly attribute AString data; */
    pub get_data: unsafe extern "C" fn (this: *const nsIAlertNotification, aData: *mut nsAString) -> nsresult,

    /* readonly attribute nsIPrincipal principal; */
    pub get_principal: unsafe extern "C" fn (this: *const nsIAlertNotification, aPrincipal: *mut *const nsIPrincipal) -> nsresult,

    /* readonly attribute nsIURI URI; */
    pub get_URI: unsafe extern "C" fn (this: *const nsIAlertNotification, aURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute boolean inPrivateBrowsing; */
    pub get_inPrivateBrowsing: unsafe extern "C" fn (this: *const nsIAlertNotification, aInPrivateBrowsing: *mut bool) -> nsresult,

    /* readonly attribute boolean requireInteraction; */
    pub get_requireInteraction: unsafe extern "C" fn (this: *const nsIAlertNotification, aRequireInteraction: *mut bool) -> nsresult,

    /* readonly attribute boolean actionable; */
    pub get_actionable: unsafe extern "C" fn (this: *const nsIAlertNotification, aActionable: *mut bool) -> nsresult,

    /* readonly attribute AString source; */
    pub get_source: unsafe extern "C" fn (this: *const nsIAlertNotification, aSource: *mut nsAString) -> nsresult,

    /* nsICancelable loadImage (in unsigned long aTimeout, in nsIAlertNotificationImageListener aListener, [optional] in nsISupports aUserData); */
    pub loadImage: unsafe extern "C" fn (this: *const nsIAlertNotification, aTimeout: libc::uint32_t, aListener: *const nsIAlertNotificationImageListener, aUserData: *const nsISupports, _retval: *mut *const nsICancelable) -> nsresult,

}


impl nsIAlertNotification {
    /* void init ([optional] in AString aName, [optional] in AString aImageURL, [optional] in AString aTitle, [optional] in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction); */
    #[inline]
    pub unsafe fn init(&self, aName: &[u16], aImageURL: &[u16], aTitle: &[u16], aText: &[u16], aTextClickable: bool, aCookie: &[u16], aDir: &[u16], aLang: &[u16], aData: &[u16], aPrincipal: Option<&nsIPrincipal>, aInPrivateBrowsing: bool, aRequireInteraction: bool) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        let aImageURL = nsString::from(aImageURL);
        let aTitle = nsString::from(aTitle);
        let aText = nsString::from(aText);
        let aCookie = nsString::from(aCookie);
        let aDir = nsString::from(aDir);
        let aLang = nsString::from(aLang);
        let aData = nsString::from(aData);
        match ((*self.vtable).init)(self as *const _, &*aName, &*aImageURL, &*aTitle, &*aText, aTextClickable, &*aCookie, &*aDir, &*aLang, &*aData, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aInPrivateBrowsing, aRequireInteraction) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString imageURL; */
    #[inline]
    pub unsafe fn get_imageURL(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_imageURL)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString title; */
    #[inline]
    pub unsafe fn get_title(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_title)(self as *const _, &mut *_retval) {
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

    /* readonly attribute boolean textClickable; */
    #[inline]
    pub unsafe fn get_textClickable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_textClickable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString cookie; */
    #[inline]
    pub unsafe fn get_cookie(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_cookie)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString dir; */
    #[inline]
    pub unsafe fn get_dir(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_dir)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString lang; */
    #[inline]
    pub unsafe fn get_lang(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_lang)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_data)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIPrincipal principal; */
    #[inline]
    pub unsafe fn get_principal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_principal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI URI; */
    #[inline]
    pub unsafe fn get_URI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_URI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean inPrivateBrowsing; */
    #[inline]
    pub unsafe fn get_inPrivateBrowsing(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_inPrivateBrowsing)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean requireInteraction; */
    #[inline]
    pub unsafe fn get_requireInteraction(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_requireInteraction)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean actionable; */
    #[inline]
    pub unsafe fn get_actionable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_actionable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString source; */
    #[inline]
    pub unsafe fn get_source(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_source)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsICancelable loadImage (in unsigned long aTimeout, in nsIAlertNotificationImageListener aListener, [optional] in nsISupports aUserData); */
    #[inline]
    pub unsafe fn loadImage(&self, aTimeout: libc::uint32_t, aListener: Option<&nsIAlertNotificationImageListener>, aUserData: Option<&nsISupports>) -> Result<Option<RefPtr<nsICancelable>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).loadImage)(self as *const _, aTimeout, aListener.map_or(::std::ptr::null(), |x| x as *const _), aUserData.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIAlertsService {
    vtable: *const nsIAlertsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAlertsService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf7a36392, 0xd98b, 0x4141,
            [0xa7, 0xd7, 0x4e, 0x46, 0x64, 0x26, 0x84, 0xe3])
    }
}

unsafe impl RefCounted for nsIAlertsService {
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
pub trait nsIAlertsServiceCoerce {
    fn coerce_from(v: &nsIAlertsService) -> &Self;
}

impl nsIAlertsServiceCoerce for nsIAlertsService {
    #[inline]
    fn coerce_from(v: &nsIAlertsService) -> &Self {
        v
    }
}

impl nsIAlertsService {
    #[inline]
    pub fn coerce<T: nsIAlertsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAlertsService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAlertsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAlertsService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAlertsServiceVTable {
    pub __base: nsISupportsVTable,

    /* void showPersistentNotification (in AString aPersistentData, in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener); */
    pub showPersistentNotification: unsafe extern "C" fn (this: *const nsIAlertsService, aPersistentData: *const nsAString, aAlert: *const nsIAlertNotification, aAlertListener: *const nsIObserver) -> nsresult,

    /* void showAlert (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener); */
    pub showAlert: unsafe extern "C" fn (this: *const nsIAlertsService, aAlert: *const nsIAlertNotification, aAlertListener: *const nsIObserver) -> nsresult,

    /* void showAlertNotification (in AString aImageURL, in AString aTitle, in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in nsIObserver aAlertListener, [optional] in AString aName, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction); */
    pub showAlertNotification: unsafe extern "C" fn (this: *const nsIAlertsService, aImageURL: *const nsAString, aTitle: *const nsAString, aText: *const nsAString, aTextClickable: bool, aCookie: *const nsAString, aAlertListener: *const nsIObserver, aName: *const nsAString, aDir: *const nsAString, aLang: *const nsAString, aData: *const nsAString, aPrincipal: *const nsIPrincipal, aInPrivateBrowsing: bool, aRequireInteraction: bool) -> nsresult,

    /* void closeAlert ([optional] in AString aName, [optional] in nsIPrincipal aPrincipal); */
    pub closeAlert: unsafe extern "C" fn (this: *const nsIAlertsService, aName: *const nsAString, aPrincipal: *const nsIPrincipal) -> nsresult,

}


impl nsIAlertsService {
    /* void showPersistentNotification (in AString aPersistentData, in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener); */
    #[inline]
    pub unsafe fn showPersistentNotification(&self, aPersistentData: &[u16], aAlert: Option<&nsIAlertNotification>, aAlertListener: Option<&nsIObserver>) -> Result<(), nsresult> {
        let aPersistentData = nsString::from(aPersistentData);
        match ((*self.vtable).showPersistentNotification)(self as *const _, &*aPersistentData, aAlert.map_or(::std::ptr::null(), |x| x as *const _), aAlertListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showAlert (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener); */
    #[inline]
    pub unsafe fn showAlert(&self, aAlert: Option<&nsIAlertNotification>, aAlertListener: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).showAlert)(self as *const _, aAlert.map_or(::std::ptr::null(), |x| x as *const _), aAlertListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showAlertNotification (in AString aImageURL, in AString aTitle, in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in nsIObserver aAlertListener, [optional] in AString aName, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction); */
    #[inline]
    pub unsafe fn showAlertNotification(&self, aImageURL: &[u16], aTitle: &[u16], aText: &[u16], aTextClickable: bool, aCookie: &[u16], aAlertListener: Option<&nsIObserver>, aName: &[u16], aDir: &[u16], aLang: &[u16], aData: &[u16], aPrincipal: Option<&nsIPrincipal>, aInPrivateBrowsing: bool, aRequireInteraction: bool) -> Result<(), nsresult> {
        let aImageURL = nsString::from(aImageURL);
        let aTitle = nsString::from(aTitle);
        let aText = nsString::from(aText);
        let aCookie = nsString::from(aCookie);
        let aName = nsString::from(aName);
        let aDir = nsString::from(aDir);
        let aLang = nsString::from(aLang);
        let aData = nsString::from(aData);
        match ((*self.vtable).showAlertNotification)(self as *const _, &*aImageURL, &*aTitle, &*aText, aTextClickable, &*aCookie, aAlertListener.map_or(::std::ptr::null(), |x| x as *const _), &*aName, &*aDir, &*aLang, &*aData, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aInPrivateBrowsing, aRequireInteraction) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void closeAlert ([optional] in AString aName, [optional] in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn closeAlert(&self, aName: &[u16], aPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).closeAlert)(self as *const _, &*aName, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIAlertsDoNotDisturb {
    vtable: *const nsIAlertsDoNotDisturbVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAlertsDoNotDisturb {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc5d63e3a, 0x259d, 0x45a8,
            [0xb9, 0x64, 0x83, 0x77, 0x96, 0x7c, 0xb4, 0xd2])
    }
}

unsafe impl RefCounted for nsIAlertsDoNotDisturb {
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
pub trait nsIAlertsDoNotDisturbCoerce {
    fn coerce_from(v: &nsIAlertsDoNotDisturb) -> &Self;
}

impl nsIAlertsDoNotDisturbCoerce for nsIAlertsDoNotDisturb {
    #[inline]
    fn coerce_from(v: &nsIAlertsDoNotDisturb) -> &Self {
        v
    }
}

impl nsIAlertsDoNotDisturb {
    #[inline]
    pub fn coerce<T: nsIAlertsDoNotDisturbCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAlertsDoNotDisturb {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAlertsDoNotDisturbCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAlertsDoNotDisturb) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAlertsDoNotDisturbVTable {
    pub __base: nsISupportsVTable,

    /* attribute bool manualDoNotDisturb; */
    pub get_manualDoNotDisturb: unsafe extern "C" fn (this: *const nsIAlertsDoNotDisturb, aManualDoNotDisturb: *mut bool) -> nsresult,
    pub set_manualDoNotDisturb: unsafe extern "C" fn (this: *const nsIAlertsDoNotDisturb, aManualDoNotDisturb: bool) -> nsresult,

}


impl nsIAlertsDoNotDisturb {
    /* attribute bool manualDoNotDisturb; */
    #[inline]
    pub unsafe fn get_manualDoNotDisturb(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_manualDoNotDisturb)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_manualDoNotDisturb(&self, aManualDoNotDisturb: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_manualDoNotDisturb)(self as *const _, aManualDoNotDisturb) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIAlertsIconData {
    vtable: *const nsIAlertsIconDataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAlertsIconData {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfc6d7f0a, 0x0cf6, 0x4268,
            [0x8c, 0x71, 0xab, 0x64, 0x08, 0x42, 0xb9, 0xb1])
    }
}

unsafe impl RefCounted for nsIAlertsIconData {
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
pub trait nsIAlertsIconDataCoerce {
    fn coerce_from(v: &nsIAlertsIconData) -> &Self;
}

impl nsIAlertsIconDataCoerce for nsIAlertsIconData {
    #[inline]
    fn coerce_from(v: &nsIAlertsIconData) -> &Self {
        v
    }
}

impl nsIAlertsIconData {
    #[inline]
    pub fn coerce<T: nsIAlertsIconDataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAlertsIconData {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAlertsIconDataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAlertsIconData) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAlertsIconDataVTable {
    pub __base: nsISupportsVTable,

    /* void showAlertWithIconData (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener, [optional] in uint32_t aIconSize, [array, size_is (aIconSize), const] in uint8_t aIconData); */
    /// Unable to call function as its signature contains a non-rust type
    pub showAlertWithIconData: *const ::libc::c_void,

}


impl nsIAlertsIconData {
    /* void showAlertWithIconData (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener, [optional] in uint32_t aIconSize, [array, size_is (aIconSize), const] in uint8_t aIconData); */


}


#[repr(C)]
pub struct nsIAlertsIconURI {
    vtable: *const nsIAlertsIconURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAlertsIconURI {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf3c82915, 0xbf60, 0x41ea,
            [0x91, 0xce, 0x6c, 0x46, 0xb2, 0x2e, 0x38, 0x1a])
    }
}

unsafe impl RefCounted for nsIAlertsIconURI {
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
pub trait nsIAlertsIconURICoerce {
    fn coerce_from(v: &nsIAlertsIconURI) -> &Self;
}

impl nsIAlertsIconURICoerce for nsIAlertsIconURI {
    #[inline]
    fn coerce_from(v: &nsIAlertsIconURI) -> &Self {
        v
    }
}

impl nsIAlertsIconURI {
    #[inline]
    pub fn coerce<T: nsIAlertsIconURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAlertsIconURI {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAlertsIconURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAlertsIconURI) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAlertsIconURIVTable {
    pub __base: nsISupportsVTable,

    /* void showAlertWithIconURI (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener, [optional] in nsIURI aIconURI); */
    pub showAlertWithIconURI: unsafe extern "C" fn (this: *const nsIAlertsIconURI, aAlert: *const nsIAlertNotification, aAlertListener: *const nsIObserver, aIconURI: *const nsIURI) -> nsresult,

}


impl nsIAlertsIconURI {
    /* void showAlertWithIconURI (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener, [optional] in nsIURI aIconURI); */
    #[inline]
    pub unsafe fn showAlertWithIconURI(&self, aAlert: Option<&nsIAlertNotification>, aAlertListener: Option<&nsIObserver>, aIconURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).showAlertWithIconURI)(self as *const _, aAlert.map_or(::std::ptr::null(), |x| x as *const _), aAlertListener.map_or(::std::ptr::null(), |x| x as *const _), aIconURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIGeckoMediaPluginService.idl
//


#[repr(C)]
pub struct mozIGeckoMediaPluginService {
    vtable: *const mozIGeckoMediaPluginServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIGeckoMediaPluginService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x44d362ae, 0x937a, 0x4803,
            [0xbe, 0xe6, 0xf2, 0x51, 0x2a, 0x01, 0x49, 0xd1])
    }
}

unsafe impl RefCounted for mozIGeckoMediaPluginService {
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
pub trait mozIGeckoMediaPluginServiceCoerce {
    fn coerce_from(v: &mozIGeckoMediaPluginService) -> &Self;
}

impl mozIGeckoMediaPluginServiceCoerce for mozIGeckoMediaPluginService {
    #[inline]
    fn coerce_from(v: &mozIGeckoMediaPluginService) -> &Self {
        v
    }
}

impl mozIGeckoMediaPluginService {
    #[inline]
    pub fn coerce<T: mozIGeckoMediaPluginServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIGeckoMediaPluginService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIGeckoMediaPluginServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIGeckoMediaPluginService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIGeckoMediaPluginServiceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIThread thread; */
    pub get_thread: unsafe extern "C" fn (this: *const mozIGeckoMediaPluginService, aThread: *mut *const nsIThread) -> nsresult,

    /* void RunPluginCrashCallbacks (in unsigned long pluginId, in ACString pluginName); */
    pub RunPluginCrashCallbacks: unsafe extern "C" fn (this: *const mozIGeckoMediaPluginService, pluginId: libc::uint32_t, pluginName: *const nsACString) -> nsresult,

    /* [noscript] boolean hasPluginForAPI (in ACString api, in TagArray tags); */
    /// Unable to call function as its signature contains a non-rust type
    pub hasPluginForAPI: *const ::libc::c_void,

    /* [noscript] void getGMPVideoDecoder (in GMPCrashHelperPtr helper, in TagArray tags, [optional] in ACString nodeId, in GetGMPVideoDecoderCallback callback); */
    /// Unable to call function as its signature contains a non-rust type
    pub getGMPVideoDecoder: *const ::libc::c_void,

    /* [noscript] void getDecryptingGMPVideoDecoder (in GMPCrashHelperPtr helper, in TagArray tags, in ACString nodeId, in GetGMPVideoDecoderCallback callback, in uint32_t decryptorId); */
    /// Unable to call function as its signature contains a non-rust type
    pub getDecryptingGMPVideoDecoder: *const ::libc::c_void,

    /* [noscript] void getGMPVideoEncoder (in GMPCrashHelperPtr helper, in TagArray tags, [optional] in ACString nodeId, in GetGMPVideoEncoderCallback callback); */
    /// Unable to call function as its signature contains a non-rust type
    pub getGMPVideoEncoder: *const ::libc::c_void,

    /* [noscript] void getGMPDecryptor (in GMPCrashHelperPtr helper, in TagArray tags, in ACString nodeId, in GetGMPDecryptorCallback callback); */
    /// Unable to call function as its signature contains a non-rust type
    pub getGMPDecryptor: *const ::libc::c_void,

    /* [noscript] void getNodeId (in AString origin, in AString topLevelOrigin, in AString gmpName, in GetNodeIdCallback callback); */
    /// Unable to call function as its signature contains a non-rust type
    pub getNodeId: *const ::libc::c_void,

}


impl mozIGeckoMediaPluginService {
    /* readonly attribute nsIThread thread; */
    #[inline]
    pub unsafe fn get_thread(&self, ) -> Result<Option<RefPtr<nsIThread>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_thread)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void RunPluginCrashCallbacks (in unsigned long pluginId, in ACString pluginName); */
    #[inline]
    pub unsafe fn RunPluginCrashCallbacks(&self, pluginId: libc::uint32_t, pluginName: &[u8]) -> Result<(), nsresult> {
        let pluginName = nsCString::from(pluginName);
        match ((*self.vtable).RunPluginCrashCallbacks)(self as *const _, pluginId, &*pluginName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] boolean hasPluginForAPI (in ACString api, in TagArray tags); */


    /* [noscript] void getGMPVideoDecoder (in GMPCrashHelperPtr helper, in TagArray tags, [optional] in ACString nodeId, in GetGMPVideoDecoderCallback callback); */


    /* [noscript] void getDecryptingGMPVideoDecoder (in GMPCrashHelperPtr helper, in TagArray tags, in ACString nodeId, in GetGMPVideoDecoderCallback callback, in uint32_t decryptorId); */


    /* [noscript] void getGMPVideoEncoder (in GMPCrashHelperPtr helper, in TagArray tags, [optional] in ACString nodeId, in GetGMPVideoEncoderCallback callback); */


    /* [noscript] void getGMPDecryptor (in GMPCrashHelperPtr helper, in TagArray tags, in ACString nodeId, in GetGMPDecryptorCallback callback); */


    /* [noscript] void getNodeId (in AString origin, in AString topLevelOrigin, in AString gmpName, in GetNodeIdCallback callback); */


}



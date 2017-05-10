//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURLParser.idl
//


#[repr(C)]
pub struct nsIURLParser {
    vtable: *const nsIURLParserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIURLParser {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x78c5d19f, 0xf5d2, 0x4732,
            [0x8d, 0x3d, 0xd5, 0xa7, 0xd7, 0x13, 0x3b, 0xc0])
    }
}

unsafe impl RefCounted for nsIURLParser {
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
pub trait nsIURLParserCoerce {
    fn coerce_from(v: &nsIURLParser) -> &Self;
}

impl nsIURLParserCoerce for nsIURLParser {
    #[inline]
    fn coerce_from(v: &nsIURLParser) -> &Self {
        v
    }
}

impl nsIURLParser {
    #[inline]
    pub fn coerce<T: nsIURLParserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIURLParser {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIURLParserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURLParser) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIURLParserVTable {
    pub __base: nsISupportsVTable,

    /* void parseURL (in string spec, in long specLen, out unsigned long schemePos, out long schemeLen, out unsigned long authorityPos, out long authorityLen, out unsigned long pathPos, out long pathLen); */
    pub parseURL: unsafe extern "C" fn (this: *const nsIURLParser, spec: *const libc::c_char, specLen: libc::int32_t, schemePos: *mut libc::uint32_t, schemeLen: *mut libc::int32_t, authorityPos: *mut libc::uint32_t, authorityLen: *mut libc::int32_t, pathPos: *mut libc::uint32_t, pathLen: *mut libc::int32_t) -> nsresult,

    /* void parseAuthority (in string authority, in long authorityLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen, out unsigned long hostnamePos, out long hostnameLen, out long port); */
    pub parseAuthority: unsafe extern "C" fn (this: *const nsIURLParser, authority: *const libc::c_char, authorityLen: libc::int32_t, usernamePos: *mut libc::uint32_t, usernameLen: *mut libc::int32_t, passwordPos: *mut libc::uint32_t, passwordLen: *mut libc::int32_t, hostnamePos: *mut libc::uint32_t, hostnameLen: *mut libc::int32_t, port: *mut libc::int32_t) -> nsresult,

    /* void parseUserInfo (in string userinfo, in long userinfoLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen); */
    pub parseUserInfo: unsafe extern "C" fn (this: *const nsIURLParser, userinfo: *const libc::c_char, userinfoLen: libc::int32_t, usernamePos: *mut libc::uint32_t, usernameLen: *mut libc::int32_t, passwordPos: *mut libc::uint32_t, passwordLen: *mut libc::int32_t) -> nsresult,

    /* void parseServerInfo (in string serverinfo, in long serverinfoLen, out unsigned long hostnamePos, out long hostnameLen, out long port); */
    pub parseServerInfo: unsafe extern "C" fn (this: *const nsIURLParser, serverinfo: *const libc::c_char, serverinfoLen: libc::int32_t, hostnamePos: *mut libc::uint32_t, hostnameLen: *mut libc::int32_t, port: *mut libc::int32_t) -> nsresult,

    /* void parsePath (in string path, in long pathLen, out unsigned long filepathPos, out long filepathLen, out unsigned long queryPos, out long queryLen, out unsigned long refPos, out long refLen); */
    pub parsePath: unsafe extern "C" fn (this: *const nsIURLParser, path: *const libc::c_char, pathLen: libc::int32_t, filepathPos: *mut libc::uint32_t, filepathLen: *mut libc::int32_t, queryPos: *mut libc::uint32_t, queryLen: *mut libc::int32_t, refPos: *mut libc::uint32_t, refLen: *mut libc::int32_t) -> nsresult,

    /* void parseFilePath (in string filepath, in long filepathLen, out unsigned long directoryPos, out long directoryLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen); */
    pub parseFilePath: unsafe extern "C" fn (this: *const nsIURLParser, filepath: *const libc::c_char, filepathLen: libc::int32_t, directoryPos: *mut libc::uint32_t, directoryLen: *mut libc::int32_t, basenamePos: *mut libc::uint32_t, basenameLen: *mut libc::int32_t, extensionPos: *mut libc::uint32_t, extensionLen: *mut libc::int32_t) -> nsresult,

    /* void parseFileName (in string filename, in long filenameLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen); */
    pub parseFileName: unsafe extern "C" fn (this: *const nsIURLParser, filename: *const libc::c_char, filenameLen: libc::int32_t, basenamePos: *mut libc::uint32_t, basenameLen: *mut libc::int32_t, extensionPos: *mut libc::uint32_t, extensionLen: *mut libc::int32_t) -> nsresult,

}


impl nsIURLParser {
    /* void parseURL (in string spec, in long specLen, out unsigned long schemePos, out long schemeLen, out unsigned long authorityPos, out long authorityLen, out unsigned long pathPos, out long pathLen); */
    #[inline]
    pub unsafe fn parseURL(&self, spec: *const libc::c_char, specLen: libc::int32_t) -> Result<(libc::uint32_t, libc::int32_t, libc::uint32_t, libc::int32_t, libc::uint32_t, libc::int32_t), nsresult> {
        let mut schemePos: libc::uint32_t = ::std::mem::zeroed();
        let mut schemeLen: libc::int32_t = ::std::mem::zeroed();
        let mut authorityPos: libc::uint32_t = ::std::mem::zeroed();
        let mut authorityLen: libc::int32_t = ::std::mem::zeroed();
        let mut pathPos: libc::uint32_t = ::std::mem::zeroed();
        let mut pathLen: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).parseURL)(self as *const _, spec, specLen, &mut schemePos as *mut _, &mut schemeLen as *mut _, &mut authorityPos as *mut _, &mut authorityLen as *mut _, &mut pathPos as *mut _, &mut pathLen as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((schemePos, schemeLen, authorityPos, authorityLen, pathPos, pathLen))
    }

    /* void parseAuthority (in string authority, in long authorityLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen, out unsigned long hostnamePos, out long hostnameLen, out long port); */
    #[inline]
    pub unsafe fn parseAuthority(&self, authority: *const libc::c_char, authorityLen: libc::int32_t) -> Result<(libc::uint32_t, libc::int32_t, libc::uint32_t, libc::int32_t, libc::uint32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut usernamePos: libc::uint32_t = ::std::mem::zeroed();
        let mut usernameLen: libc::int32_t = ::std::mem::zeroed();
        let mut passwordPos: libc::uint32_t = ::std::mem::zeroed();
        let mut passwordLen: libc::int32_t = ::std::mem::zeroed();
        let mut hostnamePos: libc::uint32_t = ::std::mem::zeroed();
        let mut hostnameLen: libc::int32_t = ::std::mem::zeroed();
        let mut port: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).parseAuthority)(self as *const _, authority, authorityLen, &mut usernamePos as *mut _, &mut usernameLen as *mut _, &mut passwordPos as *mut _, &mut passwordLen as *mut _, &mut hostnamePos as *mut _, &mut hostnameLen as *mut _, &mut port as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((usernamePos, usernameLen, passwordPos, passwordLen, hostnamePos, hostnameLen, port))
    }

    /* void parseUserInfo (in string userinfo, in long userinfoLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen); */
    #[inline]
    pub unsafe fn parseUserInfo(&self, userinfo: *const libc::c_char, userinfoLen: libc::int32_t) -> Result<(libc::uint32_t, libc::int32_t, libc::uint32_t, libc::int32_t), nsresult> {
        let mut usernamePos: libc::uint32_t = ::std::mem::zeroed();
        let mut usernameLen: libc::int32_t = ::std::mem::zeroed();
        let mut passwordPos: libc::uint32_t = ::std::mem::zeroed();
        let mut passwordLen: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).parseUserInfo)(self as *const _, userinfo, userinfoLen, &mut usernamePos as *mut _, &mut usernameLen as *mut _, &mut passwordPos as *mut _, &mut passwordLen as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((usernamePos, usernameLen, passwordPos, passwordLen))
    }

    /* void parseServerInfo (in string serverinfo, in long serverinfoLen, out unsigned long hostnamePos, out long hostnameLen, out long port); */
    #[inline]
    pub unsafe fn parseServerInfo(&self, serverinfo: *const libc::c_char, serverinfoLen: libc::int32_t) -> Result<(libc::uint32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut hostnamePos: libc::uint32_t = ::std::mem::zeroed();
        let mut hostnameLen: libc::int32_t = ::std::mem::zeroed();
        let mut port: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).parseServerInfo)(self as *const _, serverinfo, serverinfoLen, &mut hostnamePos as *mut _, &mut hostnameLen as *mut _, &mut port as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((hostnamePos, hostnameLen, port))
    }

    /* void parsePath (in string path, in long pathLen, out unsigned long filepathPos, out long filepathLen, out unsigned long queryPos, out long queryLen, out unsigned long refPos, out long refLen); */
    #[inline]
    pub unsafe fn parsePath(&self, path: *const libc::c_char, pathLen: libc::int32_t) -> Result<(libc::uint32_t, libc::int32_t, libc::uint32_t, libc::int32_t, libc::uint32_t, libc::int32_t), nsresult> {
        let mut filepathPos: libc::uint32_t = ::std::mem::zeroed();
        let mut filepathLen: libc::int32_t = ::std::mem::zeroed();
        let mut queryPos: libc::uint32_t = ::std::mem::zeroed();
        let mut queryLen: libc::int32_t = ::std::mem::zeroed();
        let mut refPos: libc::uint32_t = ::std::mem::zeroed();
        let mut refLen: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).parsePath)(self as *const _, path, pathLen, &mut filepathPos as *mut _, &mut filepathLen as *mut _, &mut queryPos as *mut _, &mut queryLen as *mut _, &mut refPos as *mut _, &mut refLen as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((filepathPos, filepathLen, queryPos, queryLen, refPos, refLen))
    }

    /* void parseFilePath (in string filepath, in long filepathLen, out unsigned long directoryPos, out long directoryLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen); */
    #[inline]
    pub unsafe fn parseFilePath(&self, filepath: *const libc::c_char, filepathLen: libc::int32_t) -> Result<(libc::uint32_t, libc::int32_t, libc::uint32_t, libc::int32_t, libc::uint32_t, libc::int32_t), nsresult> {
        let mut directoryPos: libc::uint32_t = ::std::mem::zeroed();
        let mut directoryLen: libc::int32_t = ::std::mem::zeroed();
        let mut basenamePos: libc::uint32_t = ::std::mem::zeroed();
        let mut basenameLen: libc::int32_t = ::std::mem::zeroed();
        let mut extensionPos: libc::uint32_t = ::std::mem::zeroed();
        let mut extensionLen: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).parseFilePath)(self as *const _, filepath, filepathLen, &mut directoryPos as *mut _, &mut directoryLen as *mut _, &mut basenamePos as *mut _, &mut basenameLen as *mut _, &mut extensionPos as *mut _, &mut extensionLen as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((directoryPos, directoryLen, basenamePos, basenameLen, extensionPos, extensionLen))
    }

    /* void parseFileName (in string filename, in long filenameLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen); */
    #[inline]
    pub unsafe fn parseFileName(&self, filename: *const libc::c_char, filenameLen: libc::int32_t) -> Result<(libc::uint32_t, libc::int32_t, libc::uint32_t, libc::int32_t), nsresult> {
        let mut basenamePos: libc::uint32_t = ::std::mem::zeroed();
        let mut basenameLen: libc::int32_t = ::std::mem::zeroed();
        let mut extensionPos: libc::uint32_t = ::std::mem::zeroed();
        let mut extensionLen: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).parseFileName)(self as *const _, filename, filenameLen, &mut basenamePos as *mut _, &mut basenameLen as *mut _, &mut extensionPos as *mut _, &mut extensionLen as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((basenamePos, basenameLen, extensionPos, extensionLen))
    }

}



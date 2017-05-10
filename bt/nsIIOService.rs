//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIOService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIOService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIProtocolHandler getProtocolHandler (in string aScheme); */
                    Method {
                        name: "getProtocolHandler",
                        abi: "C",
                        params: &[Param { name: "aScheme", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIProtocolHandler" }],
                        ret: "nsresult",
                    },

                    /* unsigned long getProtocolFlags (in string aScheme); */
                    Method {
                        name: "getProtocolFlags",
                        abi: "C",
                        params: &[Param { name: "aScheme", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIURI newURI (in AUTF8String aSpec, [optional] in string aOriginCharset, [optional] in nsIURI aBaseURI); */
                    Method {
                        name: "newURI",
                        abi: "C",
                        params: &[Param { name: "aSpec", ty: "*const nsACString" }, Param { name: "aOriginCharset", ty: "*const libc::c_char" }, Param { name: "aBaseURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* nsIURI newFileURI (in nsIFile aFile); */
                    Method {
                        name: "newFileURI",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* nsIChannel newChannelFromURI2 (in nsIURI aURI, in nsIDOMNode aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in unsigned long aContentPolicyType); */
                    Method {
                        name: "newChannelFromURI2",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aLoadingNode", ty: "*const nsIDOMNode" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aSecurityFlags", ty: "libc::uint32_t" }, Param { name: "aContentPolicyType", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* nsIChannel newChannelFromURIWithLoadInfo (in nsIURI aURI, in nsILoadInfo aLoadInfo); */
                    Method {
                        name: "newChannelFromURIWithLoadInfo",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aLoadInfo", ty: "*const nsILoadInfo" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* nsIChannel newChannel2 (in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI, in nsIDOMNode aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in unsigned long aContentPolicyType); */
                    Method {
                        name: "newChannel2",
                        abi: "C",
                        params: &[Param { name: "aSpec", ty: "*const nsACString" }, Param { name: "aOriginCharset", ty: "*const libc::c_char" }, Param { name: "aBaseURI", ty: "*const nsIURI" }, Param { name: "aLoadingNode", ty: "*const nsIDOMNode" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aSecurityFlags", ty: "libc::uint32_t" }, Param { name: "aContentPolicyType", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* nsIChannel newChannelFromURI (in nsIURI aURI); */
                    Method {
                        name: "newChannelFromURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* nsIChannel newChannel (in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI); */
                    Method {
                        name: "newChannel",
                        abi: "C",
                        params: &[Param { name: "aSpec", ty: "*const nsACString" }, Param { name: "aOriginCharset", ty: "*const libc::c_char" }, Param { name: "aBaseURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean offline; */
                    Method {
                        name: "get_offline",
                        abi: "C",
                        params: &[Param { name: "aOffline", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_offline",
                        abi: "C",
                        params: &[Param { name: "aOffline", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean connectivity; */
                    Method {
                        name: "get_connectivity",
                        abi: "C",
                        params: &[Param { name: "aConnectivity", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean allowPort (in long aPort, in string aScheme); */
                    Method {
                        name: "allowPort",
                        abi: "C",
                        params: &[Param { name: "aPort", ty: "libc::int32_t" }, Param { name: "aScheme", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* ACString extractScheme (in AUTF8String urlString); */
                    Method {
                        name: "extractScheme",
                        abi: "C",
                        params: &[Param { name: "urlString", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIIOServiceInternal",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void SetConnectivity (in boolean connectivity); */
                    Method {
                        name: "SetConnectivity",
                        abi: "C",
                        params: &[Param { name: "connectivity", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void NotifyWakeup (); */
                    Method {
                        name: "NotifyWakeup",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


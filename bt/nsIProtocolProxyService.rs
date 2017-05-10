//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProtocolProxyService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProtocolProxyService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsICancelable asyncResolve (in nsISupports aChannelOrURI, in unsigned long aFlags, in nsIProtocolProxyCallback aCallback); */
                    Method {
                        name: "asyncResolve",
                        abi: "C",
                        params: &[Param { name: "aChannelOrURI", ty: "*const nsISupports" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "aCallback", ty: "*const nsIProtocolProxyCallback" }, Param { name: "_retval", ty: "*mut *const nsICancelable" }],
                        ret: "nsresult",
                    },

                    /* nsIProxyInfo newProxyInfo (in ACString aType, in AUTF8String aHost, in long aPort, in unsigned long aFlags, in unsigned long aFailoverTimeout, in nsIProxyInfo aFailoverProxy); */
                    Method {
                        name: "newProxyInfo",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*const nsACString" }, Param { name: "aHost", ty: "*const nsACString" }, Param { name: "aPort", ty: "libc::int32_t" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "aFailoverTimeout", ty: "libc::uint32_t" }, Param { name: "aFailoverProxy", ty: "*const nsIProxyInfo" }, Param { name: "_retval", ty: "*mut *const nsIProxyInfo" }],
                        ret: "nsresult",
                    },

                    /* nsIProxyInfo newProxyInfoWithAuth (in ACString aType, in AUTF8String aHost, in long aPort, in ACString aUsername, in ACString aPassword, in unsigned long aFlags, in unsigned long aFailoverTimeout, in nsIProxyInfo aFailoverProxy); */
                    Method {
                        name: "newProxyInfoWithAuth",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*const nsACString" }, Param { name: "aHost", ty: "*const nsACString" }, Param { name: "aPort", ty: "libc::int32_t" }, Param { name: "aUsername", ty: "*const nsACString" }, Param { name: "aPassword", ty: "*const nsACString" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "aFailoverTimeout", ty: "libc::uint32_t" }, Param { name: "aFailoverProxy", ty: "*const nsIProxyInfo" }, Param { name: "_retval", ty: "*mut *const nsIProxyInfo" }],
                        ret: "nsresult",
                    },

                    /* nsIProxyInfo getFailoverForProxy (in nsIProxyInfo aProxyInfo, in nsIURI aURI, in nsresult aReason); */
                    Method {
                        name: "getFailoverForProxy",
                        abi: "C",
                        params: &[Param { name: "aProxyInfo", ty: "*const nsIProxyInfo" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aReason", ty: "nsresult" }, Param { name: "_retval", ty: "*mut *const nsIProxyInfo" }],
                        ret: "nsresult",
                    },

                    /* void registerFilter (in nsIProtocolProxyFilter aFilter, in unsigned long aPosition); */
                    Method {
                        name: "registerFilter",
                        abi: "C",
                        params: &[Param { name: "aFilter", ty: "*const nsIProtocolProxyFilter" }, Param { name: "aPosition", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void registerChannelFilter (in nsIProtocolProxyChannelFilter aFilter, in unsigned long aPosition); */
                    Method {
                        name: "registerChannelFilter",
                        abi: "C",
                        params: &[Param { name: "aFilter", ty: "*const nsIProtocolProxyChannelFilter" }, Param { name: "aPosition", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void unregisterFilter (in nsIProtocolProxyFilter aFilter); */
                    Method {
                        name: "unregisterFilter",
                        abi: "C",
                        params: &[Param { name: "aFilter", ty: "*const nsIProtocolProxyFilter" }],
                        ret: "nsresult",
                    },

                    /* void unregisterChannelFilter (in nsIProtocolProxyChannelFilter aFilter); */
                    Method {
                        name: "unregisterChannelFilter",
                        abi: "C",
                        params: &[Param { name: "aFilter", ty: "*const nsIProtocolProxyChannelFilter" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long proxyConfigType; */
                    Method {
                        name: "get_proxyConfigType",
                        abi: "C",
                        params: &[Param { name: "aProxyConfigType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


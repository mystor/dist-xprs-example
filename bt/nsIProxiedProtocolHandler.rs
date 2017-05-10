//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProxiedProtocolHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProxiedProtocolHandler",
            base: Some("nsIProtocolHandler"),
            methods: Some(&[
                    /* nsIChannel newProxiedChannel2 (in nsIURI uri, in nsIProxyInfo proxyInfo, in unsigned long proxyResolveFlags, in nsIURI proxyURI, in nsILoadInfo aLoadInfo); */
                    Method {
                        name: "newProxiedChannel2",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "proxyInfo", ty: "*const nsIProxyInfo" }, Param { name: "proxyResolveFlags", ty: "libc::uint32_t" }, Param { name: "proxyURI", ty: "*const nsIURI" }, Param { name: "aLoadInfo", ty: "*const nsILoadInfo" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* nsIChannel newProxiedChannel (in nsIURI uri, in nsIProxyInfo proxyInfo, in unsigned long proxyResolveFlags, in nsIURI proxyURI); */
                    Method {
                        name: "newProxiedChannel",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "proxyInfo", ty: "*const nsIProxyInfo" }, Param { name: "proxyResolveFlags", ty: "libc::uint32_t" }, Param { name: "proxyURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


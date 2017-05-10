//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIOService2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIOService2",
            base: Some("nsIIOService"),
            methods: Some(&[
                    /* attribute boolean manageOfflineStatus; */
                    Method {
                        name: "get_manageOfflineStatus",
                        abi: "C",
                        params: &[Param { name: "aManageOfflineStatus", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_manageOfflineStatus",
                        abi: "C",
                        params: &[Param { name: "aManageOfflineStatus", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* nsIChannel newChannelFromURIWithProxyFlags2 (in nsIURI aURI, in nsIURI aProxyURI, in unsigned long aProxyFlags, in nsIDOMNode aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in unsigned long aContentPolicyType); */
                    Method {
                        name: "newChannelFromURIWithProxyFlags2",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aProxyURI", ty: "*const nsIURI" }, Param { name: "aProxyFlags", ty: "libc::uint32_t" }, Param { name: "aLoadingNode", ty: "*const nsIDOMNode" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aSecurityFlags", ty: "libc::uint32_t" }, Param { name: "aContentPolicyType", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* nsIChannel newChannelFromURIWithProxyFlags (in nsIURI aURI, in nsIURI aProxyURI, in unsigned long aProxyFlags); */
                    Method {
                        name: "newChannelFromURIWithProxyFlags",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aProxyURI", ty: "*const nsIURI" }, Param { name: "aProxyFlags", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIChannel" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


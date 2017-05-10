//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProtocolProxyFilter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProtocolProxyFilter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIProxyInfo applyFilter (in nsIProtocolProxyService aProxyService, in nsIURI aURI, in nsIProxyInfo aProxy); */
                    Method {
                        name: "applyFilter",
                        abi: "C",
                        params: &[Param { name: "aProxyService", ty: "*const nsIProtocolProxyService" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aProxy", ty: "*const nsIProxyInfo" }, Param { name: "_retval", ty: "*mut *const nsIProxyInfo" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIProtocolProxyChannelFilter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIProxyInfo applyFilter (in nsIProtocolProxyService aProxyService, in nsIChannel aChannel, in nsIProxyInfo aProxy); */
                    Method {
                        name: "applyFilter",
                        abi: "C",
                        params: &[Param { name: "aProxyService", ty: "*const nsIProtocolProxyService" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aProxy", ty: "*const nsIProxyInfo" }, Param { name: "_retval", ty: "*mut *const nsIProxyInfo" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


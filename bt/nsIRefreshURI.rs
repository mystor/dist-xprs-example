//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRefreshURI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRefreshURI",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void refreshURI (in nsIURI aURI, in long aMillis, in boolean aRepeat, in boolean aMetaRefresh); */
                    Method {
                        name: "refreshURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aMillis", ty: "libc::int32_t" }, Param { name: "aRepeat", ty: "bool" }, Param { name: "aMetaRefresh", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void forceRefreshURI (in nsIURI aURI, in long aMillis, in boolean aMetaRefresh); */
                    Method {
                        name: "forceRefreshURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aMillis", ty: "libc::int32_t" }, Param { name: "aMetaRefresh", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void setupRefreshURI (in nsIChannel aChannel); */
                    Method {
                        name: "setupRefreshURI",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }],
                        ret: "nsresult",
                    },

                    /* void setupRefreshURIFromHeader (in nsIURI aBaseURI, in nsIPrincipal principal, in ACString aHeader); */
                    Method {
                        name: "setupRefreshURIFromHeader",
                        abi: "C",
                        params: &[Param { name: "aBaseURI", ty: "*const nsIURI" }, Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "aHeader", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void cancelRefreshURITimers (); */
                    Method {
                        name: "cancelRefreshURITimers",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean refreshPending; */
                    Method {
                        name: "get_refreshPending",
                        abi: "C",
                        params: &[Param { name: "aRefreshPending", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


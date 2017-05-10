//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISpeculativeConnect.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISpeculativeConnect",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void speculativeConnect (in nsIURI aURI, in nsIInterfaceRequestor aCallbacks); */
                    Method {
                        name: "speculativeConnect",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aCallbacks", ty: "*const nsIInterfaceRequestor" }],
                        ret: "nsresult",
                    },

                    /* void speculativeConnect2 (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks); */
                    Method {
                        name: "speculativeConnect2",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aCallbacks", ty: "*const nsIInterfaceRequestor" }],
                        ret: "nsresult",
                    },

                    /* void speculativeAnonymousConnect (in nsIURI aURI, in nsIInterfaceRequestor aCallbacks); */
                    Method {
                        name: "speculativeAnonymousConnect",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aCallbacks", ty: "*const nsIInterfaceRequestor" }],
                        ret: "nsresult",
                    },

                    /* void speculativeAnonymousConnect2 (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks); */
                    Method {
                        name: "speculativeAnonymousConnect2",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aCallbacks", ty: "*const nsIInterfaceRequestor" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISpeculativeConnectionOverrider",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [infallible] readonly attribute unsigned long parallelSpeculativeConnectLimit; */
                    Method {
                        name: "get_parallelSpeculativeConnectLimit",
                        abi: "C",
                        params: &[Param { name: "aParallelSpeculativeConnectLimit", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [infallible] readonly attribute boolean ignoreIdle; */
                    Method {
                        name: "get_ignoreIdle",
                        abi: "C",
                        params: &[Param { name: "aIgnoreIdle", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [infallible] readonly attribute boolean isFromPredictor; */
                    Method {
                        name: "get_isFromPredictor",
                        abi: "C",
                        params: &[Param { name: "aIsFromPredictor", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [infallible] readonly attribute boolean allow1918; */
                    Method {
                        name: "get_allow1918",
                        abi: "C",
                        params: &[Param { name: "aAllow1918", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


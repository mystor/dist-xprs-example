//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXPConnect.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXPConnectJSObjectHolder",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIXPConnectWrappedNative",
            base: Some("nsIXPConnectJSObjectHolder"),
            methods: None,
        },


        Interface {
            name: "nsIXPConnectWrappedJS",
            base: Some("nsIXPConnectJSObjectHolder"),
            methods: Some(&[
                    /* readonly attribute nsIInterfaceInfo InterfaceInfo; */
                    Method {
                        name: "get_InterfaceInfo",
                        abi: "C",
                        params: &[Param { name: "aInterfaceInfo", ty: "*mut *const nsIInterfaceInfo" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIIDPtr InterfaceIID; */
                    Method {
                        name: "get_InterfaceIID",
                        abi: "C",
                        params: &[Param { name: "aInterfaceIID", ty: "*mut *const nsIID" }],
                        ret: "nsresult",
                    },

                    /* void debugDump (in short depth); */
                    Method {
                        name: "debugDump",
                        abi: "C",
                        params: &[Param { name: "depth", ty: "libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void aggregatedQueryInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
                    Method {
                        name: "aggregatedQueryInterface",
                        abi: "C",
                        params: &[Param { name: "uuid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIXPConnectWrappedJSUnmarkGray",
            base: Some("nsIXPConnectWrappedJS"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIXPCWrappedJSObjectGetter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsISupports neverCalled; */
                    Method {
                        name: "get_neverCalled",
                        abi: "C",
                        params: &[Param { name: "aNeverCalled", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIXPCFunctionThisTranslator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISupports TranslateThis (in nsISupports aInitialThis); */
                    Method {
                        name: "TranslateThis",
                        abi: "C",
                        params: &[Param { name: "aInitialThis", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIXPConnect",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}


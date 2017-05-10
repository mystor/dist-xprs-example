//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIQuotaRequests.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIQuotaRequestBase",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "get_principal",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*mut *const nsIPrincipal" }],
                        ret: "nsresult",
                    },

                    /* [must_use] readonly attribute nsresult resultCode; */
                    Method {
                        name: "get_resultCode",
                        abi: "C",
                        params: &[Param { name: "aResultCode", ty: "*mut nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIQuotaUsageRequest",
            base: Some("nsIQuotaRequestBase"),
            methods: Some(&[
                    /* [must_use] readonly attribute nsIVariant result; */
                    Method {
                        name: "get_result",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIQuotaUsageCallback callback; */
                    Method {
                        name: "get_callback",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*mut *const nsIQuotaUsageCallback" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_callback",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*const nsIQuotaUsageCallback" }],
                        ret: "nsresult",
                    },

                    /* [must_use] void cancel (); */
                    Method {
                        name: "cancel",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIQuotaRequest",
            base: Some("nsIQuotaRequestBase"),
            methods: Some(&[
                    /* [must_use] readonly attribute nsIVariant result; */
                    Method {
                        name: "get_result",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIQuotaCallback callback; */
                    Method {
                        name: "get_callback",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*mut *const nsIQuotaCallback" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_callback",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*const nsIQuotaCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIQuotaResults.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIQuotaUsageResult",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute ACString origin; */
                    Method {
                        name: "get_origin",
                        abi: "C",
                        params: &[Param { name: "aOrigin", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean persisted; */
                    Method {
                        name: "get_persisted",
                        abi: "C",
                        params: &[Param { name: "aPersisted", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long long usage; */
                    Method {
                        name: "get_usage",
                        abi: "C",
                        params: &[Param { name: "aUsage", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIQuotaOriginUsageResult",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned long long usage; */
                    Method {
                        name: "get_usage",
                        abi: "C",
                        params: &[Param { name: "aUsage", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long long fileUsage; */
                    Method {
                        name: "get_fileUsage",
                        abi: "C",
                        params: &[Param { name: "aFileUsage", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long long limit; */
                    Method {
                        name: "get_limit",
                        abi: "C",
                        params: &[Param { name: "aLimit", ty: "*mut libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


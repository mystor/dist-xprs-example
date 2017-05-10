//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIQuotaManagerService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIQuotaManagerService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] nsIQuotaRequest init (); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIQuotaRequest" }],
                        ret: "nsresult",
                    },

                    /* [must_use] nsIQuotaRequest initStoragesForPrincipal (in nsIPrincipal aPrincipal, in ACString aPersistenceType); */
                    Method {
                        name: "initStoragesForPrincipal",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aPersistenceType", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIQuotaRequest" }],
                        ret: "nsresult",
                    },

                    /* [must_use] nsIQuotaUsageRequest getUsage (in nsIQuotaUsageCallback aCallback, [optional] in boolean aGetAll); */
                    Method {
                        name: "getUsage",
                        abi: "C",
                        params: &[Param { name: "aCallback", ty: "*const nsIQuotaUsageCallback" }, Param { name: "aGetAll", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIQuotaUsageRequest" }],
                        ret: "nsresult",
                    },

                    /* [must_use] nsIQuotaUsageRequest getUsageForPrincipal (in nsIPrincipal aPrincipal, in nsIQuotaUsageCallback aCallback, [optional] in boolean aGetGroupUsage); */
                    Method {
                        name: "getUsageForPrincipal",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aCallback", ty: "*const nsIQuotaUsageCallback" }, Param { name: "aGetGroupUsage", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIQuotaUsageRequest" }],
                        ret: "nsresult",
                    },

                    /* [must_use] nsIQuotaRequest clear (); */
                    Method {
                        name: "clear",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIQuotaRequest" }],
                        ret: "nsresult",
                    },

                    /* [must_use] nsIQuotaRequest clearStoragesForPrincipal (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType, [optional] in boolean aClearAll); */
                    Method {
                        name: "clearStoragesForPrincipal",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aPersistenceType", ty: "*const nsACString" }, Param { name: "aClearAll", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIQuotaRequest" }],
                        ret: "nsresult",
                    },

                    /* [must_use] nsIQuotaRequest reset (); */
                    Method {
                        name: "reset",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIQuotaRequest" }],
                        ret: "nsresult",
                    },

                    /* [must_use] nsIQuotaRequest persisted (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "persisted",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut *const nsIQuotaRequest" }],
                        ret: "nsresult",
                    },

                    /* [must_use] nsIQuotaRequest persist (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "persist",
                        abi: "C",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut *const nsIQuotaRequest" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


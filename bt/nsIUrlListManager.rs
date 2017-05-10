//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUrlListManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUrlListManagerCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleEvent (in ACString value); */
                    Method {
                        name: "handleEvent",
                        abi: "C",
                        params: &[Param { name: "value", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUrlListManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* ACString getGethashUrl (in ACString tableName); */
                    Method {
                        name: "getGethashUrl",
                        abi: "C",
                        params: &[Param { name: "tableName", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* ACString getUpdateUrl (in ACString tableName); */
                    Method {
                        name: "getUpdateUrl",
                        abi: "C",
                        params: &[Param { name: "tableName", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean registerTable (in ACString tableName, in ACString providerName, in ACString updateUrl, in ACString gethashUrl); */
                    Method {
                        name: "registerTable",
                        abi: "C",
                        params: &[Param { name: "tableName", ty: "*const nsACString" }, Param { name: "providerName", ty: "*const nsACString" }, Param { name: "updateUrl", ty: "*const nsACString" }, Param { name: "gethashUrl", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void enableUpdate (in ACString tableName); */
                    Method {
                        name: "enableUpdate",
                        abi: "C",
                        params: &[Param { name: "tableName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void disableUpdate (in ACString tableName); */
                    Method {
                        name: "disableUpdate",
                        abi: "C",
                        params: &[Param { name: "tableName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void maybeToggleUpdateChecking (); */
                    Method {
                        name: "maybeToggleUpdateChecking",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void safeLookup (in nsIPrincipal key, in nsIUrlListManagerCallback cb); */
                    Method {
                        name: "safeLookup",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsIPrincipal" }, Param { name: "cb", ty: "*const nsIUrlListManagerCallback" }],
                        ret: "nsresult",
                    },

                    /* boolean checkForUpdates (in ACString updateUrl); */
                    Method {
                        name: "checkForUpdates",
                        abi: "C",
                        params: &[Param { name: "updateUrl", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


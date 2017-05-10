//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUrlClassifierDBService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUrlClassifierCallback",
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
            name: "nsIUrlClassifierUpdateObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void updateUrlRequested (in ACString url, in ACString table); */
                    Method {
                        name: "updateUrlRequested",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsACString" }, Param { name: "table", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void streamFinished (in nsresult status, in unsigned long delay); */
                    Method {
                        name: "streamFinished",
                        abi: "C",
                        params: &[Param { name: "status", ty: "nsresult" }, Param { name: "delay", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void updateError (in nsresult error); */
                    Method {
                        name: "updateError",
                        abi: "C",
                        params: &[Param { name: "error", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void updateSuccess (in unsigned long requestedTimeout); */
                    Method {
                        name: "updateSuccess",
                        abi: "C",
                        params: &[Param { name: "requestedTimeout", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUrlClassifierDBService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void lookup (in nsIPrincipal principal, in ACString tables, in nsIUrlClassifierCallback c); */
                    Method {
                        name: "lookup",
                        abi: "C",
                        params: &[Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "tables", ty: "*const nsACString" }, Param { name: "c", ty: "*const nsIUrlClassifierCallback" }],
                        ret: "nsresult",
                    },

                    /* void getTables (in nsIUrlClassifierCallback c); */
                    Method {
                        name: "getTables",
                        abi: "C",
                        params: &[Param { name: "c", ty: "*const nsIUrlClassifierCallback" }],
                        ret: "nsresult",
                    },

                    /* void setHashCompleter (in ACString tableName, in nsIUrlClassifierHashCompleter completer); */
                    Method {
                        name: "setHashCompleter",
                        abi: "C",
                        params: &[Param { name: "tableName", ty: "*const nsACString" }, Param { name: "completer", ty: "*const nsIUrlClassifierHashCompleter" }],
                        ret: "nsresult",
                    },

                    /* void setLastUpdateTime (in ACString tableName, in unsigned long long lastUpdateTime); */
                    Method {
                        name: "setLastUpdateTime",
                        abi: "C",
                        params: &[Param { name: "tableName", ty: "*const nsACString" }, Param { name: "lastUpdateTime", ty: "libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    /* void clearLastResults (); */
                    Method {
                        name: "clearLastResults",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void beginUpdate (in nsIUrlClassifierUpdateObserver updater, in ACString tables); */
                    Method {
                        name: "beginUpdate",
                        abi: "C",
                        params: &[Param { name: "updater", ty: "*const nsIUrlClassifierUpdateObserver" }, Param { name: "tables", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void beginStream (in ACString table); */
                    Method {
                        name: "beginStream",
                        abi: "C",
                        params: &[Param { name: "table", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void updateStream (in ACString updateChunk); */
                    Method {
                        name: "updateStream",
                        abi: "C",
                        params: &[Param { name: "updateChunk", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void finishStream (); */
                    Method {
                        name: "finishStream",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void finishUpdate (); */
                    Method {
                        name: "finishUpdate",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void cancelUpdate (); */
                    Method {
                        name: "cancelUpdate",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void resetDatabase (); */
                    Method {
                        name: "resetDatabase",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void reloadDatabase (); */
                    Method {
                        name: "reloadDatabase",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUrlClassifierLookupCallback",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIUrlClassifierClassifyCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleResult (in ACString aList, in ACString aPrefix); */
                    Method {
                        name: "handleResult",
                        abi: "C",
                        params: &[Param { name: "aList", ty: "*const nsACString" }, Param { name: "aPrefix", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


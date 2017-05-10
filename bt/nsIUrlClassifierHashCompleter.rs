//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUrlClassifierHashCompleter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFullHashMatch",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute ACString tableName; */
                    Method {
                        name: "get_tableName",
                        abi: "C",
                        params: &[Param { name: "aTableName", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString fullHash; */
                    Method {
                        name: "get_fullHash",
                        abi: "C",
                        params: &[Param { name: "aFullHash", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t cacheDuration; */
                    Method {
                        name: "get_cacheDuration",
                        abi: "C",
                        params: &[Param { name: "aCacheDuration", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUrlClassifierHashCompleterCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void completionV2 (in ACString hash, in ACString table, in uint32_t chunkId); */
                    Method {
                        name: "completionV2",
                        abi: "C",
                        params: &[Param { name: "hash", ty: "*const nsACString" }, Param { name: "table", ty: "*const nsACString" }, Param { name: "chunkId", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void completionV4 (in ACString partialHash, in ACString table, in uint32_t negativeCacheDuration, in nsIArray fullHashes); */
                    Method {
                        name: "completionV4",
                        abi: "C",
                        params: &[Param { name: "partialHash", ty: "*const nsACString" }, Param { name: "table", ty: "*const nsACString" }, Param { name: "negativeCacheDuration", ty: "uint32_t" }, Param { name: "fullHashes", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* void completionFinished (in nsresult status); */
                    Method {
                        name: "completionFinished",
                        abi: "C",
                        params: &[Param { name: "status", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIUrlClassifierHashCompleter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void complete (in ACString partialHash, in ACString gethashUrl, in ACString tableName, in nsIUrlClassifierHashCompleterCallback callback); */
                    Method {
                        name: "complete",
                        abi: "C",
                        params: &[Param { name: "partialHash", ty: "*const nsACString" }, Param { name: "gethashUrl", ty: "*const nsACString" }, Param { name: "tableName", ty: "*const nsACString" }, Param { name: "callback", ty: "*const nsIUrlClassifierHashCompleterCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


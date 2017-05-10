//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICachingChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICachingChannel",
            base: Some("nsICacheInfoChannel"),
            methods: Some(&[
                    /* attribute nsISupports cacheToken; */
                    Method {
                        name: "get_cacheToken",
                        abi: "C",
                        params: &[Param { name: "aCacheToken", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_cacheToken",
                        abi: "C",
                        params: &[Param { name: "aCacheToken", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISupports offlineCacheToken; */
                    Method {
                        name: "get_offlineCacheToken",
                        abi: "C",
                        params: &[Param { name: "aOfflineCacheToken", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_offlineCacheToken",
                        abi: "C",
                        params: &[Param { name: "aOfflineCacheToken", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean cacheOnlyMetadata; */
                    Method {
                        name: "get_cacheOnlyMetadata",
                        abi: "C",
                        params: &[Param { name: "aCacheOnlyMetadata", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_cacheOnlyMetadata",
                        abi: "C",
                        params: &[Param { name: "aCacheOnlyMetadata", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean pin; */
                    Method {
                        name: "get_pin",
                        abi: "C",
                        params: &[Param { name: "aPin", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_pin",
                        abi: "C",
                        params: &[Param { name: "aPin", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void forceCacheEntryValidFor (in unsigned long aSecondsToTheFuture); */
                    Method {
                        name: "forceCacheEntryValidFor",
                        abi: "C",
                        params: &[Param { name: "aSecondsToTheFuture", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


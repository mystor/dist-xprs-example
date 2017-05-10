//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheInfoChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheInfoChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute uint32_t cacheTokenExpirationTime; */
                    Method {
                        name: "get_cacheTokenExpirationTime",
                        abi: "C",
                        params: &[Param { name: "aCacheTokenExpirationTime", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute ACString cacheTokenCachedCharset; */
                    Method {
                        name: "get_cacheTokenCachedCharset",
                        abi: "C",
                        params: &[Param { name: "aCacheTokenCachedCharset", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_cacheTokenCachedCharset",
                        abi: "C",
                        params: &[Param { name: "aCacheTokenCachedCharset", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean isFromCache (); */
                    Method {
                        name: "isFromCache",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISupports cacheKey; */
                    Method {
                        name: "get_cacheKey",
                        abi: "C",
                        params: &[Param { name: "aCacheKey", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_cacheKey",
                        abi: "C",
                        params: &[Param { name: "aCacheKey", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean allowStaleCacheContent; */
                    Method {
                        name: "get_allowStaleCacheContent",
                        abi: "C",
                        params: &[Param { name: "aAllowStaleCacheContent", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_allowStaleCacheContent",
                        abi: "C",
                        params: &[Param { name: "aAllowStaleCacheContent", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void preferAlternativeDataType (in ACString type); */
                    Method {
                        name: "preferAlternativeDataType",
                        abi: "C",
                        params: &[Param { name: "type_", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString alternativeDataType; */
                    Method {
                        name: "get_alternativeDataType",
                        abi: "C",
                        params: &[Param { name: "aAlternativeDataType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* nsIOutputStream openAlternativeOutputStream (in ACString type); */
                    Method {
                        name: "openAlternativeOutputStream",
                        abi: "C",
                        params: &[Param { name: "type_", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIOutputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


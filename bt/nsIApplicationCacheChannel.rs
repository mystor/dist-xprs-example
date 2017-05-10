//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIApplicationCacheChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIApplicationCacheChannel",
            base: Some("nsIApplicationCacheContainer"),
            methods: Some(&[
                    /* readonly attribute boolean loadedFromApplicationCache; */
                    Method {
                        name: "get_loadedFromApplicationCache",
                        abi: "C",
                        params: &[Param { name: "aLoadedFromApplicationCache", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean inheritApplicationCache; */
                    Method {
                        name: "get_inheritApplicationCache",
                        abi: "C",
                        params: &[Param { name: "aInheritApplicationCache", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_inheritApplicationCache",
                        abi: "C",
                        params: &[Param { name: "aInheritApplicationCache", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean chooseApplicationCache; */
                    Method {
                        name: "get_chooseApplicationCache",
                        abi: "C",
                        params: &[Param { name: "aChooseApplicationCache", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_chooseApplicationCache",
                        abi: "C",
                        params: &[Param { name: "aChooseApplicationCache", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void markOfflineCacheEntryAsForeign (); */
                    Method {
                        name: "markOfflineCacheEntryAsForeign",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* attribute nsIApplicationCache applicationCacheForWrite; */
                    Method {
                        name: "get_applicationCacheForWrite",
                        abi: "C",
                        params: &[Param { name: "aApplicationCacheForWrite", ty: "*mut *const nsIApplicationCache" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_applicationCacheForWrite",
                        abi: "C",
                        params: &[Param { name: "aApplicationCacheForWrite", ty: "*const nsIApplicationCache" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


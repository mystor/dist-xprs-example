//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheSession.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheSession",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute boolean doomEntriesIfExpired; */
                    Method {
                        name: "get_doomEntriesIfExpired",
                        abi: "C",
                        params: &[Param { name: "aDoomEntriesIfExpired", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_doomEntriesIfExpired",
                        abi: "C",
                        params: &[Param { name: "aDoomEntriesIfExpired", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIFile profileDirectory; */
                    Method {
                        name: "get_profileDirectory",
                        abi: "C",
                        params: &[Param { name: "aProfileDirectory", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_profileDirectory",
                        abi: "C",
                        params: &[Param { name: "aProfileDirectory", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* nsICacheEntryDescriptor openCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in boolean blockingMode); */
                    Method {
                        name: "openCacheEntry",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "accessRequested", ty: "nsCacheAccessMode" }, Param { name: "blockingMode", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsICacheEntryDescriptor" }],
                        ret: "nsresult",
                    },

                    /* void asyncOpenCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in nsICacheListener listener, [optional] in boolean noWait); */
                    Method {
                        name: "asyncOpenCacheEntry",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "accessRequested", ty: "nsCacheAccessMode" }, Param { name: "listener", ty: "*const nsICacheListener" }, Param { name: "noWait", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void evictEntries (); */
                    Method {
                        name: "evictEntries",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean isStorageEnabled (); */
                    Method {
                        name: "isStorageEnabled",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void doomEntry (in ACString key, in nsICacheListener listener); */
                    Method {
                        name: "doomEntry",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const nsACString" }, Param { name: "listener", ty: "*const nsICacheListener" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean isPrivate; */
                    Method {
                        name: "get_isPrivate",
                        abi: "C",
                        params: &[Param { name: "aIsPrivate", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_isPrivate",
                        abi: "C",
                        params: &[Param { name: "aIsPrivate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


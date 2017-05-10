//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheStorageService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheStorageService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsICacheStorage memoryCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
                    Method {
                        name: "memoryCacheStorage",
                        abi: "C",
                        params: &[Param { name: "aLoadContextInfo", ty: "*const nsILoadContextInfo" }, Param { name: "_retval", ty: "*mut *const nsICacheStorage" }],
                        ret: "nsresult",
                    },

                    /* nsICacheStorage diskCacheStorage (in nsILoadContextInfo aLoadContextInfo, in bool aLookupAppCache); */
                    Method {
                        name: "diskCacheStorage",
                        abi: "C",
                        params: &[Param { name: "aLoadContextInfo", ty: "*const nsILoadContextInfo" }, Param { name: "aLookupAppCache", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsICacheStorage" }],
                        ret: "nsresult",
                    },

                    /* nsICacheStorage pinningCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
                    Method {
                        name: "pinningCacheStorage",
                        abi: "C",
                        params: &[Param { name: "aLoadContextInfo", ty: "*const nsILoadContextInfo" }, Param { name: "_retval", ty: "*mut *const nsICacheStorage" }],
                        ret: "nsresult",
                    },

                    /* nsICacheStorage appCacheStorage (in nsILoadContextInfo aLoadContextInfo, in nsIApplicationCache aApplicationCache); */
                    Method {
                        name: "appCacheStorage",
                        abi: "C",
                        params: &[Param { name: "aLoadContextInfo", ty: "*const nsILoadContextInfo" }, Param { name: "aApplicationCache", ty: "*const nsIApplicationCache" }, Param { name: "_retval", ty: "*mut *const nsICacheStorage" }],
                        ret: "nsresult",
                    },

                    /* nsICacheStorage synthesizedCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
                    Method {
                        name: "synthesizedCacheStorage",
                        abi: "C",
                        params: &[Param { name: "aLoadContextInfo", ty: "*const nsILoadContextInfo" }, Param { name: "_retval", ty: "*mut *const nsICacheStorage" }],
                        ret: "nsresult",
                    },

                    /* void clear (); */
                    Method {
                        name: "clear",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void purgeFromMemory (in uint32_t aWhat); */
                    Method {
                        name: "purgeFromMemory",
                        abi: "C",
                        params: &[Param { name: "aWhat", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIEventTarget ioTarget; */
                    Method {
                        name: "get_ioTarget",
                        abi: "C",
                        params: &[Param { name: "aIoTarget", ty: "*mut *const nsIEventTarget" }],
                        ret: "nsresult",
                    },

                    /* void asyncGetDiskConsumption (in nsICacheStorageConsumptionObserver aObserver); */
                    Method {
                        name: "asyncGetDiskConsumption",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsICacheStorageConsumptionObserver" }],
                        ret: "nsresult",
                    },

                    /* void asyncVisitAllStorages (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries); */
                    Method {
                        name: "asyncVisitAllStorages",
                        abi: "C",
                        params: &[Param { name: "aVisitor", ty: "*const nsICacheStorageVisitor" }, Param { name: "aVisitEntries", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICacheStorageConsumptionObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onNetworkCacheDiskConsumption (in int64_t aDiskSize); */
                    Method {
                        name: "onNetworkCacheDiskConsumption",
                        abi: "C",
                        params: &[Param { name: "aDiskSize", ty: "int64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


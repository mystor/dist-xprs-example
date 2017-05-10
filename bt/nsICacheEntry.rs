//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheEntry.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheEntry",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute ACString key; */
                    Method {
                        name: "get_key",
                        abi: "C",
                        params: &[Param { name: "aKey", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean persistent; */
                    Method {
                        name: "get_persistent",
                        abi: "C",
                        params: &[Param { name: "aPersistent", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long fetchCount; */
                    Method {
                        name: "get_fetchCount",
                        abi: "C",
                        params: &[Param { name: "aFetchCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t lastFetched; */
                    Method {
                        name: "get_lastFetched",
                        abi: "C",
                        params: &[Param { name: "aLastFetched", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t lastModified; */
                    Method {
                        name: "get_lastModified",
                        abi: "C",
                        params: &[Param { name: "aLastModified", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t expirationTime; */
                    Method {
                        name: "get_expirationTime",
                        abi: "C",
                        params: &[Param { name: "aExpirationTime", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void setExpirationTime (in uint32_t expirationTime); */
                    Method {
                        name: "setExpirationTime",
                        abi: "C",
                        params: &[Param { name: "expirationTime", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint64_t onStartTime; */
                    Method {
                        name: "get_onStartTime",
                        abi: "C",
                        params: &[Param { name: "aOnStartTime", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint64_t onStopTime; */
                    Method {
                        name: "get_onStopTime",
                        abi: "C",
                        params: &[Param { name: "aOnStopTime", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },

                    /* void setNetworkTimes (in uint64_t onStartTime, in uint64_t onStopTime); */
                    Method {
                        name: "setNetworkTimes",
                        abi: "C",
                        params: &[Param { name: "onStartTime", ty: "uint64_t" }, Param { name: "onStopTime", ty: "uint64_t" }],
                        ret: "nsresult",
                    },

                    /* void forceValidFor (in unsigned long aSecondsToTheFuture); */
                    Method {
                        name: "forceValidFor",
                        abi: "C",
                        params: &[Param { name: "aSecondsToTheFuture", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isForcedValid; */
                    Method {
                        name: "get_isForcedValid",
                        abi: "C",
                        params: &[Param { name: "aIsForcedValid", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream openInputStream (in long long offset); */
                    Method {
                        name: "openInputStream",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::int64_t" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* nsIOutputStream openOutputStream (in long long offset); */
                    Method {
                        name: "openOutputStream",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::int64_t" }, Param { name: "_retval", ty: "*mut *const nsIOutputStream" }],
                        ret: "nsresult",
                    },

                    /* attribute int64_t predictedDataSize; */
                    Method {
                        name: "get_predictedDataSize",
                        abi: "C",
                        params: &[Param { name: "aPredictedDataSize", ty: "*mut int64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_predictedDataSize",
                        abi: "C",
                        params: &[Param { name: "aPredictedDataSize", ty: "int64_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISupports securityInfo; */
                    Method {
                        name: "get_securityInfo",
                        abi: "C",
                        params: &[Param { name: "aSecurityInfo", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_securityInfo",
                        abi: "C",
                        params: &[Param { name: "aSecurityInfo", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long storageDataSize; */
                    Method {
                        name: "get_storageDataSize",
                        abi: "C",
                        params: &[Param { name: "aStorageDataSize", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void asyncDoom (in nsICacheEntryDoomCallback listener); */
                    Method {
                        name: "asyncDoom",
                        abi: "C",
                        params: &[Param { name: "listener", ty: "*const nsICacheEntryDoomCallback" }],
                        ret: "nsresult",
                    },

                    /* string getMetaDataElement (in string key); */
                    Method {
                        name: "getMetaDataElement",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void setMetaDataElement (in string key, in string value); */
                    Method {
                        name: "setMetaDataElement",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void visitMetaData (in nsICacheEntryMetaDataVisitor visitor); */
                    Method {
                        name: "visitMetaData",
                        abi: "C",
                        params: &[Param { name: "visitor", ty: "*const nsICacheEntryMetaDataVisitor" }],
                        ret: "nsresult",
                    },

                    /* void metaDataReady (); */
                    Method {
                        name: "metaDataReady",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void setValid (); */
                    Method {
                        name: "setValid",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t diskStorageSizeInKB; */
                    Method {
                        name: "get_diskStorageSizeInKB",
                        abi: "C",
                        params: &[Param { name: "aDiskStorageSizeInKB", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsICacheEntry recreate ([optional] in boolean aMemoryOnly); */
                    Method {
                        name: "recreate",
                        abi: "C",
                        params: &[Param { name: "aMemoryOnly", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsICacheEntry" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long dataSize; */
                    Method {
                        name: "get_dataSize",
                        abi: "C",
                        params: &[Param { name: "aDataSize", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long altDataSize; */
                    Method {
                        name: "get_altDataSize",
                        abi: "C",
                        params: &[Param { name: "aAltDataSize", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* nsIOutputStream openAlternativeOutputStream (in ACString type); */
                    Method {
                        name: "openAlternativeOutputStream",
                        abi: "C",
                        params: &[Param { name: "type_", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIOutputStream" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream openAlternativeInputStream (in ACString type); */
                    Method {
                        name: "openAlternativeInputStream",
                        abi: "C",
                        params: &[Param { name: "type_", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsILoadContextInfo loadContextInfo; */
                    Method {
                        name: "get_loadContextInfo",
                        abi: "C",
                        params: &[Param { name: "aLoadContextInfo", ty: "*mut *const nsILoadContextInfo" }],
                        ret: "nsresult",
                    },

                    /* void close (); */
                    Method {
                        name: "close",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void markValid (); */
                    Method {
                        name: "markValid",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void maybeMarkValid (); */
                    Method {
                        name: "maybeMarkValid",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean hasWriteAccess (in boolean aWriteAllowed); */
                    Method {
                        name: "hasWriteAccess",
                        abi: "C",
                        params: &[Param { name: "aWriteAllowed", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICacheEntryMetaDataVisitor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onMetaDataElement (in string key, in string value); */
                    Method {
                        name: "onMetaDataElement",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheEntryDescriptor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheEntryDescriptor",
            base: Some("nsICacheEntryInfo"),
            methods: Some(&[
                    /* void setExpirationTime (in uint32_t expirationTime); */
                    Method {
                        name: "setExpirationTime",
                        abi: "C",
                        params: &[Param { name: "expirationTime", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void setDataSize (in unsigned long size); */
                    Method {
                        name: "setDataSize",
                        abi: "C",
                        params: &[Param { name: "size", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIInputStream openInputStream (in unsigned long offset); */
                    Method {
                        name: "openInputStream",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "nsresult",
                    },

                    /* nsIOutputStream openOutputStream (in unsigned long offset); */
                    Method {
                        name: "openOutputStream",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIOutputStream" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISupports cacheElement; */
                    Method {
                        name: "get_cacheElement",
                        abi: "C",
                        params: &[Param { name: "aCacheElement", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_cacheElement",
                        abi: "C",
                        params: &[Param { name: "aCacheElement", ty: "*const nsISupports" }],
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

                    /* readonly attribute nsCacheAccessMode accessGranted; */
                    Method {
                        name: "get_accessGranted",
                        abi: "C",
                        params: &[Param { name: "aAccessGranted", ty: "*mut nsCacheAccessMode" }],
                        ret: "nsresult",
                    },

                    /* attribute nsCacheStoragePolicy storagePolicy; */
                    Method {
                        name: "get_storagePolicy",
                        abi: "C",
                        params: &[Param { name: "aStoragePolicy", ty: "*mut nsCacheStoragePolicy" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_storagePolicy",
                        abi: "C",
                        params: &[Param { name: "aStoragePolicy", ty: "nsCacheStoragePolicy" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIFile file; */
                    Method {
                        name: "get_file",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*mut *const nsIFile" }],
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

                    /* void doom (); */
                    Method {
                        name: "doom",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void doomAndFailPendingRequests (in nsresult status); */
                    Method {
                        name: "doomAndFailPendingRequests",
                        abi: "C",
                        params: &[Param { name: "status", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void asyncDoom (in nsICacheListener listener); */
                    Method {
                        name: "asyncDoom",
                        abi: "C",
                        params: &[Param { name: "listener", ty: "*const nsICacheListener" }],
                        ret: "nsresult",
                    },

                    /* void markValid (); */
                    Method {
                        name: "markValid",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void close (); */
                    Method {
                        name: "close",
                        abi: "C",
                        params: &[],
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

                    /* void visitMetaData (in nsICacheMetaDataVisitor visitor); */
                    Method {
                        name: "visitMetaData",
                        abi: "C",
                        params: &[Param { name: "visitor", ty: "*const nsICacheMetaDataVisitor" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICacheMetaDataVisitor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean visitMetaDataElement (in string key, in string value); */
                    Method {
                        name: "visitMetaDataElement",
                        abi: "C",
                        params: &[Param { name: "key", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


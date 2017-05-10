//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheVisitor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheVisitor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean visitDevice (in string deviceID, in nsICacheDeviceInfo deviceInfo); */
                    Method {
                        name: "visitDevice",
                        abi: "C",
                        params: &[Param { name: "deviceID", ty: "*const libc::c_char" }, Param { name: "deviceInfo", ty: "*const nsICacheDeviceInfo" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean visitEntry (in string deviceID, in nsICacheEntryInfo entryInfo); */
                    Method {
                        name: "visitEntry",
                        abi: "C",
                        params: &[Param { name: "deviceID", ty: "*const libc::c_char" }, Param { name: "entryInfo", ty: "*const nsICacheEntryInfo" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICacheDeviceInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute string description; */
                    Method {
                        name: "get_description",
                        abi: "C",
                        params: &[Param { name: "aDescription", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string usageReport; */
                    Method {
                        name: "get_usageReport",
                        abi: "C",
                        params: &[Param { name: "aUsageReport", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long entryCount; */
                    Method {
                        name: "get_entryCount",
                        abi: "C",
                        params: &[Param { name: "aEntryCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long totalSize; */
                    Method {
                        name: "get_totalSize",
                        abi: "C",
                        params: &[Param { name: "aTotalSize", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long maximumSize; */
                    Method {
                        name: "get_maximumSize",
                        abi: "C",
                        params: &[Param { name: "aMaximumSize", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsICacheEntryInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute string clientID; */
                    Method {
                        name: "get_clientID",
                        abi: "C",
                        params: &[Param { name: "aClientID", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute string deviceID; */
                    Method {
                        name: "get_deviceID",
                        abi: "C",
                        params: &[Param { name: "aDeviceID", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString key; */
                    Method {
                        name: "get_key",
                        abi: "C",
                        params: &[Param { name: "aKey", ty: "*mut nsACString" }],
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

                    /* readonly attribute unsigned long dataSize; */
                    Method {
                        name: "get_dataSize",
                        abi: "C",
                        params: &[Param { name: "aDataSize", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean isStreamBased (); */
                    Method {
                        name: "isStreamBased",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


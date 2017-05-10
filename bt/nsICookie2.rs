//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICookie2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICookie2",
            base: Some("nsICookie"),
            methods: Some(&[
                    /* readonly attribute AUTF8String rawHost; */
                    Method {
                        name: "get_rawHost",
                        abi: "C",
                        params: &[Param { name: "aRawHost", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isSession; */
                    Method {
                        name: "get_isSession",
                        abi: "C",
                        params: &[Param { name: "aIsSession", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute int64_t expiry; */
                    Method {
                        name: "get_expiry",
                        abi: "C",
                        params: &[Param { name: "aExpiry", ty: "*mut int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isHttpOnly; */
                    Method {
                        name: "get_isHttpOnly",
                        abi: "C",
                        params: &[Param { name: "aIsHttpOnly", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute int64_t creationTime; */
                    Method {
                        name: "get_creationTime",
                        abi: "C",
                        params: &[Param { name: "aCreationTime", ty: "*mut int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute int64_t lastAccessed; */
                    Method {
                        name: "get_lastAccessed",
                        abi: "C",
                        params: &[Param { name: "aLastAccessed", ty: "*mut int64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


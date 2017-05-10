//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINSSVersion.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINSSVersion",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AString NSPR_MinVersion; */
                    Method {
                        name: "get_NSPR_MinVersion",
                        abi: "C",
                        params: &[Param { name: "aNSPR_MinVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString NSS_MinVersion; */
                    Method {
                        name: "get_NSS_MinVersion",
                        abi: "C",
                        params: &[Param { name: "aNSS_MinVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString NSSUTIL_MinVersion; */
                    Method {
                        name: "get_NSSUTIL_MinVersion",
                        abi: "C",
                        params: &[Param { name: "aNSSUTIL_MinVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString NSSSSL_MinVersion; */
                    Method {
                        name: "get_NSSSSL_MinVersion",
                        abi: "C",
                        params: &[Param { name: "aNSSSSL_MinVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString NSSSMIME_MinVersion; */
                    Method {
                        name: "get_NSSSMIME_MinVersion",
                        abi: "C",
                        params: &[Param { name: "aNSSSMIME_MinVersion", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString NSPR_Version; */
                    Method {
                        name: "get_NSPR_Version",
                        abi: "C",
                        params: &[Param { name: "aNSPR_Version", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString NSS_Version; */
                    Method {
                        name: "get_NSS_Version",
                        abi: "C",
                        params: &[Param { name: "aNSS_Version", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString NSSUTIL_Version; */
                    Method {
                        name: "get_NSSUTIL_Version",
                        abi: "C",
                        params: &[Param { name: "aNSSUTIL_Version", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString NSSSSL_Version; */
                    Method {
                        name: "get_NSSSSL_Version",
                        abi: "C",
                        params: &[Param { name: "aNSSSSL_Version", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString NSSSMIME_Version; */
                    Method {
                        name: "get_NSSSMIME_Version",
                        abi: "C",
                        params: &[Param { name: "aNSSSMIME_Version", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


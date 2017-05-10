//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIX509CertValidity.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIX509CertValidity",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute PRTime notBefore; */
                    Method {
                        name: "get_notBefore",
                        abi: "C",
                        params: &[Param { name: "aNotBefore", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString notBeforeLocalTime; */
                    Method {
                        name: "get_notBeforeLocalTime",
                        abi: "C",
                        params: &[Param { name: "aNotBeforeLocalTime", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString notBeforeLocalDay; */
                    Method {
                        name: "get_notBeforeLocalDay",
                        abi: "C",
                        params: &[Param { name: "aNotBeforeLocalDay", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString notBeforeGMT; */
                    Method {
                        name: "get_notBeforeGMT",
                        abi: "C",
                        params: &[Param { name: "aNotBeforeGMT", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime notAfter; */
                    Method {
                        name: "get_notAfter",
                        abi: "C",
                        params: &[Param { name: "aNotAfter", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString notAfterLocalTime; */
                    Method {
                        name: "get_notAfterLocalTime",
                        abi: "C",
                        params: &[Param { name: "aNotAfterLocalTime", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString notAfterLocalDay; */
                    Method {
                        name: "get_notAfterLocalDay",
                        abi: "C",
                        params: &[Param { name: "aNotAfterLocalDay", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString notAfterGMT; */
                    Method {
                        name: "get_notAfterGMT",
                        abi: "C",
                        params: &[Param { name: "aNotAfterGMT", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


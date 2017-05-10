//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/inICSSValueSearch.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "inICSSValueSearch",
            base: Some("inISearchProcess"),
            methods: Some(&[
                    /* attribute nsIDOMDocument document; */
                    Method {
                        name: "get_document",
                        abi: "C",
                        params: &[Param { name: "aDocument", ty: "*mut *const nsIDOMDocument" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_document",
                        abi: "C",
                        params: &[Param { name: "aDocument", ty: "*const nsIDOMDocument" }],
                        ret: "nsresult",
                    },

                    /* attribute wstring baseURL; */
                    Method {
                        name: "get_baseURL",
                        abi: "C",
                        params: &[Param { name: "aBaseURL", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_baseURL",
                        abi: "C",
                        params: &[Param { name: "aBaseURL", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean returnRelativeURLs; */
                    Method {
                        name: "get_returnRelativeURLs",
                        abi: "C",
                        params: &[Param { name: "aReturnRelativeURLs", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_returnRelativeURLs",
                        abi: "C",
                        params: &[Param { name: "aReturnRelativeURLs", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean normalizeChromeURLs; */
                    Method {
                        name: "get_normalizeChromeURLs",
                        abi: "C",
                        params: &[Param { name: "aNormalizeChromeURLs", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_normalizeChromeURLs",
                        abi: "C",
                        params: &[Param { name: "aNormalizeChromeURLs", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void addPropertyCriteria (in wstring aPropName); */
                    Method {
                        name: "addPropertyCriteria",
                        abi: "C",
                        params: &[Param { name: "aPropName", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute wstring textCriteria; */
                    Method {
                        name: "get_textCriteria",
                        abi: "C",
                        params: &[Param { name: "aTextCriteria", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_textCriteria",
                        abi: "C",
                        params: &[Param { name: "aTextCriteria", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


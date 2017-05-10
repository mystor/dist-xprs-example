//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoCompleteResult.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAutoCompleteResult",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AString searchString; */
                    Method {
                        name: "get_searchString",
                        abi: "C",
                        params: &[Param { name: "aSearchString", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned short searchResult; */
                    Method {
                        name: "get_searchResult",
                        abi: "C",
                        params: &[Param { name: "aSearchResult", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long defaultIndex; */
                    Method {
                        name: "get_defaultIndex",
                        abi: "C",
                        params: &[Param { name: "aDefaultIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString errorDescription; */
                    Method {
                        name: "get_errorDescription",
                        abi: "C",
                        params: &[Param { name: "aErrorDescription", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long matchCount; */
                    Method {
                        name: "get_matchCount",
                        abi: "C",
                        params: &[Param { name: "aMatchCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* AString getValueAt (in long index); */
                    Method {
                        name: "getValueAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getLabelAt (in long index); */
                    Method {
                        name: "getLabelAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getCommentAt (in long index); */
                    Method {
                        name: "getCommentAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getStyleAt (in long index); */
                    Method {
                        name: "getStyleAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getImageAt (in long index); */
                    Method {
                        name: "getImageAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getFinalCompleteValueAt (in long index); */
                    Method {
                        name: "getFinalCompleteValueAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void removeValueAt (in long rowIndex, in boolean removeFromDb); */
                    Method {
                        name: "removeValueAt",
                        abi: "C",
                        params: &[Param { name: "rowIndex", ty: "libc::int32_t" }, Param { name: "removeFromDb", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


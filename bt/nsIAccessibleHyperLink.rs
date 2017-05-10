//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleHyperLink.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleHyperLink",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute long startIndex; */
                    Method {
                        name: "get_startIndex",
                        abi: "C",
                        params: &[Param { name: "aStartIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long endIndex; */
                    Method {
                        name: "get_endIndex",
                        abi: "C",
                        params: &[Param { name: "aEndIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean valid; */
                    Method {
                        name: "get_valid",
                        abi: "C",
                        params: &[Param { name: "aValid", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long anchorCount; */
                    Method {
                        name: "get_anchorCount",
                        abi: "C",
                        params: &[Param { name: "aAnchorCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIURI getURI (in long index); */
                    Method {
                        name: "getURI",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessible getAnchor (in long index); */
                    Method {
                        name: "getAnchor",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


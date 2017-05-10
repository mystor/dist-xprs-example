//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleHyperText.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleHyperText",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute long linkCount; */
                    Method {
                        name: "get_linkCount",
                        abi: "C",
                        params: &[Param { name: "aLinkCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessibleHyperLink getLinkAt (in long index); */
                    Method {
                        name: "getLinkAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIAccessibleHyperLink" }],
                        ret: "nsresult",
                    },

                    /* long getLinkIndex (in nsIAccessibleHyperLink link); */
                    Method {
                        name: "getLinkIndex",
                        abi: "C",
                        params: &[Param { name: "link", ty: "*const nsIAccessibleHyperLink" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* long getLinkIndexAtOffset (in long offset); */
                    Method {
                        name: "getLinkIndexAtOffset",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


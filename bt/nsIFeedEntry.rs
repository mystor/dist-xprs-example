//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedEntry.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFeedEntry",
            base: Some("nsIFeedContainer"),
            methods: Some(&[
                    /* attribute nsIFeedTextConstruct summary; */
                    Method {
                        name: "get_summary",
                        abi: "C",
                        params: &[Param { name: "aSummary", ty: "*mut *const nsIFeedTextConstruct" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_summary",
                        abi: "C",
                        params: &[Param { name: "aSummary", ty: "*const nsIFeedTextConstruct" }],
                        ret: "nsresult",
                    },

                    /* attribute AString published; */
                    Method {
                        name: "get_published",
                        abi: "C",
                        params: &[Param { name: "aPublished", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_published",
                        abi: "C",
                        params: &[Param { name: "aPublished", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIFeedTextConstruct content; */
                    Method {
                        name: "get_content",
                        abi: "C",
                        params: &[Param { name: "aContent", ty: "*mut *const nsIFeedTextConstruct" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_content",
                        abi: "C",
                        params: &[Param { name: "aContent", ty: "*const nsIFeedTextConstruct" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIArray enclosures; */
                    Method {
                        name: "get_enclosures",
                        abi: "C",
                        params: &[Param { name: "aEnclosures", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_enclosures",
                        abi: "C",
                        params: &[Param { name: "aEnclosures", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIArray mediaContent; */
                    Method {
                        name: "get_mediaContent",
                        abi: "C",
                        params: &[Param { name: "aMediaContent", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_mediaContent",
                        abi: "C",
                        params: &[Param { name: "aMediaContent", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeed.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFeed",
            base: Some("nsIFeedContainer"),
            methods: Some(&[
                    /* attribute nsIFeedTextConstruct subtitle; */
                    Method {
                        name: "get_subtitle",
                        abi: "C",
                        params: &[Param { name: "aSubtitle", ty: "*mut *const nsIFeedTextConstruct" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_subtitle",
                        abi: "C",
                        params: &[Param { name: "aSubtitle", ty: "*const nsIFeedTextConstruct" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute long enclosureCount; */
                    Method {
                        name: "get_enclosureCount",
                        abi: "C",
                        params: &[Param { name: "aEnclosureCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_enclosureCount",
                        abi: "C",
                        params: &[Param { name: "aEnclosureCount", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIArray items; */
                    Method {
                        name: "get_items",
                        abi: "C",
                        params: &[Param { name: "aItems", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_items",
                        abi: "C",
                        params: &[Param { name: "aItems", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIWritablePropertyBag2 cloud; */
                    Method {
                        name: "get_cloud",
                        abi: "C",
                        params: &[Param { name: "aCloud", ty: "*mut *const nsIWritablePropertyBag2" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_cloud",
                        abi: "C",
                        params: &[Param { name: "aCloud", ty: "*const nsIWritablePropertyBag2" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIFeedGenerator generator; */
                    Method {
                        name: "get_generator",
                        abi: "C",
                        params: &[Param { name: "aGenerator", ty: "*mut *const nsIFeedGenerator" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_generator",
                        abi: "C",
                        params: &[Param { name: "aGenerator", ty: "*const nsIFeedGenerator" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIWritablePropertyBag2 image; */
                    Method {
                        name: "get_image",
                        abi: "C",
                        params: &[Param { name: "aImage", ty: "*mut *const nsIWritablePropertyBag2" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_image",
                        abi: "C",
                        params: &[Param { name: "aImage", ty: "*const nsIWritablePropertyBag2" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIWritablePropertyBag2 textInput; */
                    Method {
                        name: "get_textInput",
                        abi: "C",
                        params: &[Param { name: "aTextInput", ty: "*mut *const nsIWritablePropertyBag2" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_textInput",
                        abi: "C",
                        params: &[Param { name: "aTextInput", ty: "*const nsIWritablePropertyBag2" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIArray skipDays; */
                    Method {
                        name: "get_skipDays",
                        abi: "C",
                        params: &[Param { name: "aSkipDays", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_skipDays",
                        abi: "C",
                        params: &[Param { name: "aSkipDays", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIArray skipHours; */
                    Method {
                        name: "get_skipHours",
                        abi: "C",
                        params: &[Param { name: "aSkipHours", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_skipHours",
                        abi: "C",
                        params: &[Param { name: "aSkipHours", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


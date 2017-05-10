//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedContainer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFeedContainer",
            base: Some("nsIFeedElementBase"),
            methods: Some(&[
                    /* attribute AString id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIWritablePropertyBag2 fields; */
                    Method {
                        name: "get_fields",
                        abi: "C",
                        params: &[Param { name: "aFields", ty: "*mut *const nsIWritablePropertyBag2" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_fields",
                        abi: "C",
                        params: &[Param { name: "aFields", ty: "*const nsIWritablePropertyBag2" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIFeedTextConstruct title; */
                    Method {
                        name: "get_title",
                        abi: "C",
                        params: &[Param { name: "aTitle", ty: "*mut *const nsIFeedTextConstruct" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_title",
                        abi: "C",
                        params: &[Param { name: "aTitle", ty: "*const nsIFeedTextConstruct" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIURI link; */
                    Method {
                        name: "get_link",
                        abi: "C",
                        params: &[Param { name: "aLink", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_link",
                        abi: "C",
                        params: &[Param { name: "aLink", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIArray links; */
                    Method {
                        name: "get_links",
                        abi: "C",
                        params: &[Param { name: "aLinks", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_links",
                        abi: "C",
                        params: &[Param { name: "aLinks", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIArray categories; */
                    Method {
                        name: "get_categories",
                        abi: "C",
                        params: &[Param { name: "aCategories", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_categories",
                        abi: "C",
                        params: &[Param { name: "aCategories", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIFeedTextConstruct rights; */
                    Method {
                        name: "get_rights",
                        abi: "C",
                        params: &[Param { name: "aRights", ty: "*mut *const nsIFeedTextConstruct" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_rights",
                        abi: "C",
                        params: &[Param { name: "aRights", ty: "*const nsIFeedTextConstruct" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIArray authors; */
                    Method {
                        name: "get_authors",
                        abi: "C",
                        params: &[Param { name: "aAuthors", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_authors",
                        abi: "C",
                        params: &[Param { name: "aAuthors", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIArray contributors; */
                    Method {
                        name: "get_contributors",
                        abi: "C",
                        params: &[Param { name: "aContributors", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_contributors",
                        abi: "C",
                        params: &[Param { name: "aContributors", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* attribute AString updated; */
                    Method {
                        name: "get_updated",
                        abi: "C",
                        params: &[Param { name: "aUpdated", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_updated",
                        abi: "C",
                        params: &[Param { name: "aUpdated", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void normalize (); */
                    Method {
                        name: "normalize",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


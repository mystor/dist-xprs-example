//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIAsyncLivemarks.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIAsyncLivemarks",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "mozILivemarkInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute long long id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString guid; */
                    Method {
                        name: "get_guid",
                        abi: "C",
                        params: &[Param { name: "aGuid", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString title; */
                    Method {
                        name: "get_title",
                        abi: "C",
                        params: &[Param { name: "aTitle", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long parentId; */
                    Method {
                        name: "get_parentId",
                        abi: "C",
                        params: &[Param { name: "aParentId", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long parentGuid; */
                    Method {
                        name: "get_parentGuid",
                        abi: "C",
                        params: &[Param { name: "aParentGuid", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long index; */
                    Method {
                        name: "get_index",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime dateAdded; */
                    Method {
                        name: "get_dateAdded",
                        abi: "C",
                        params: &[Param { name: "aDateAdded", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime lastModified; */
                    Method {
                        name: "get_lastModified",
                        abi: "C",
                        params: &[Param { name: "aLastModified", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI feedURI; */
                    Method {
                        name: "get_feedURI",
                        abi: "C",
                        params: &[Param { name: "aFeedURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI siteURI; */
                    Method {
                        name: "get_siteURI",
                        abi: "C",
                        params: &[Param { name: "aSiteURI", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "mozILivemark",
            base: Some("mozILivemarkInfo"),
            methods: None,
        },


        ]; D}


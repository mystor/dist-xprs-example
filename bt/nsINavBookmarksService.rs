//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINavBookmarksService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINavBookmarkObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute boolean skipTags; */
                    Method {
                        name: "get_skipTags",
                        abi: "C",
                        params: &[Param { name: "aSkipTags", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean skipDescendantsOnItemRemoval; */
                    Method {
                        name: "get_skipDescendantsOnItemRemoval",
                        abi: "C",
                        params: &[Param { name: "aSkipDescendantsOnItemRemoval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void onBeginUpdateBatch (); */
                    Method {
                        name: "onBeginUpdateBatch",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onEndUpdateBatch (); */
                    Method {
                        name: "onEndUpdateBatch",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onItemAdded (in long long aItemId, in long long aParentId, in long aIndex, in unsigned short aItemType, in nsIURI aURI, in AUTF8String aTitle, in PRTime aDateAdded, in ACString aGuid, in ACString aParentGuid, in unsigned short aSource); */
                    Method {
                        name: "onItemAdded",
                        abi: "C",
                        params: &[Param { name: "aItemId", ty: "libc::int64_t" }, Param { name: "aParentId", ty: "libc::int64_t" }, Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "aItemType", ty: "libc::uint16_t" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aTitle", ty: "*const nsACString" }, Param { name: "aDateAdded", ty: "PRTime" }, Param { name: "aGuid", ty: "*const nsACString" }, Param { name: "aParentGuid", ty: "*const nsACString" }, Param { name: "aSource", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* void onItemRemoved (in long long aItemId, in long long aParentId, in long aIndex, in unsigned short aItemType, in nsIURI aURI, in ACString aGuid, in ACString aParentGuid, in unsigned short aSource); */
                    Method {
                        name: "onItemRemoved",
                        abi: "C",
                        params: &[Param { name: "aItemId", ty: "libc::int64_t" }, Param { name: "aParentId", ty: "libc::int64_t" }, Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "aItemType", ty: "libc::uint16_t" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aGuid", ty: "*const nsACString" }, Param { name: "aParentGuid", ty: "*const nsACString" }, Param { name: "aSource", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* void onItemChanged (in long long aItemId, in ACString aProperty, in boolean aIsAnnotationProperty, in AUTF8String aNewValue, in PRTime aLastModified, in unsigned short aItemType, in long long aParentId, in ACString aGuid, in ACString aParentGuid, in AUTF8String aOldValue, in unsigned short aSource); */
                    Method {
                        name: "onItemChanged",
                        abi: "C",
                        params: &[Param { name: "aItemId", ty: "libc::int64_t" }, Param { name: "aProperty", ty: "*const nsACString" }, Param { name: "aIsAnnotationProperty", ty: "bool" }, Param { name: "aNewValue", ty: "*const nsACString" }, Param { name: "aLastModified", ty: "PRTime" }, Param { name: "aItemType", ty: "libc::uint16_t" }, Param { name: "aParentId", ty: "libc::int64_t" }, Param { name: "aGuid", ty: "*const nsACString" }, Param { name: "aParentGuid", ty: "*const nsACString" }, Param { name: "aOldValue", ty: "*const nsACString" }, Param { name: "aSource", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* void onItemVisited (in long long aItemId, in long long aVisitId, in PRTime aTime, in unsigned long aTransitionType, in nsIURI aURI, in long long aParentId, in ACString aGuid, in ACString aParentGuid); */
                    Method {
                        name: "onItemVisited",
                        abi: "C",
                        params: &[Param { name: "aItemId", ty: "libc::int64_t" }, Param { name: "aVisitId", ty: "libc::int64_t" }, Param { name: "aTime", ty: "PRTime" }, Param { name: "aTransitionType", ty: "libc::uint32_t" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aParentId", ty: "libc::int64_t" }, Param { name: "aGuid", ty: "*const nsACString" }, Param { name: "aParentGuid", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void onItemMoved (in long long aItemId, in long long aOldParentId, in long aOldIndex, in long long aNewParentId, in long aNewIndex, in unsigned short aItemType, in ACString aGuid, in ACString aOldParentGuid, in ACString aNewParentGuid, in unsigned short aSource); */
                    Method {
                        name: "onItemMoved",
                        abi: "C",
                        params: &[Param { name: "aItemId", ty: "libc::int64_t" }, Param { name: "aOldParentId", ty: "libc::int64_t" }, Param { name: "aOldIndex", ty: "libc::int32_t" }, Param { name: "aNewParentId", ty: "libc::int64_t" }, Param { name: "aNewIndex", ty: "libc::int32_t" }, Param { name: "aItemType", ty: "libc::uint16_t" }, Param { name: "aGuid", ty: "*const nsACString" }, Param { name: "aOldParentGuid", ty: "*const nsACString" }, Param { name: "aNewParentGuid", ty: "*const nsACString" }, Param { name: "aSource", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINavBookmarksService",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}


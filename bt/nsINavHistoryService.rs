//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINavHistoryService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINavHistoryResultNode",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsINavHistoryContainerResultNode parent; */
                    Method {
                        name: "get_parent",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*mut *const nsINavHistoryContainerResultNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsINavHistoryResult parentResult; */
                    Method {
                        name: "get_parentResult",
                        abi: "C",
                        params: &[Param { name: "aParentResult", ty: "*mut *const nsINavHistoryResult" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String uri; */
                    Method {
                        name: "get_uri",
                        abi: "C",
                        params: &[Param { name: "aUri", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String title; */
                    Method {
                        name: "get_title",
                        abi: "C",
                        params: &[Param { name: "aTitle", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long accessCount; */
                    Method {
                        name: "get_accessCount",
                        abi: "C",
                        params: &[Param { name: "aAccessCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime time; */
                    Method {
                        name: "get_time",
                        abi: "C",
                        params: &[Param { name: "aTime", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String icon; */
                    Method {
                        name: "get_icon",
                        abi: "C",
                        params: &[Param { name: "aIcon", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long indentLevel; */
                    Method {
                        name: "get_indentLevel",
                        abi: "C",
                        params: &[Param { name: "aIndentLevel", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long bookmarkIndex; */
                    Method {
                        name: "get_bookmarkIndex",
                        abi: "C",
                        params: &[Param { name: "aBookmarkIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long itemId; */
                    Method {
                        name: "get_itemId",
                        abi: "C",
                        params: &[Param { name: "aItemId", ty: "*mut libc::int64_t" }],
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

                    /* readonly attribute AString tags; */
                    Method {
                        name: "get_tags",
                        abi: "C",
                        params: &[Param { name: "aTags", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString pageGuid; */
                    Method {
                        name: "get_pageGuid",
                        abi: "C",
                        params: &[Param { name: "aPageGuid", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString bookmarkGuid; */
                    Method {
                        name: "get_bookmarkGuid",
                        abi: "C",
                        params: &[Param { name: "aBookmarkGuid", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long visitId; */
                    Method {
                        name: "get_visitId",
                        abi: "C",
                        params: &[Param { name: "aVisitId", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long long fromVisitId; */
                    Method {
                        name: "get_fromVisitId",
                        abi: "C",
                        params: &[Param { name: "aFromVisitId", ty: "*mut libc::int64_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long visitType; */
                    Method {
                        name: "get_visitType",
                        abi: "C",
                        params: &[Param { name: "aVisitType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINavHistoryContainerResultNode",
            base: Some("nsINavHistoryResultNode"),
            methods: Some(&[
                    /* attribute boolean containerOpen; */
                    Method {
                        name: "get_containerOpen",
                        abi: "C",
                        params: &[Param { name: "aContainerOpen", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_containerOpen",
                        abi: "C",
                        params: &[Param { name: "aContainerOpen", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned short state; */
                    Method {
                        name: "get_state",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean hasChildren; */
                    Method {
                        name: "get_hasChildren",
                        abi: "C",
                        params: &[Param { name: "aHasChildren", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long childCount; */
                    Method {
                        name: "get_childCount",
                        abi: "C",
                        params: &[Param { name: "aChildCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsINavHistoryResultNode getChild (in unsigned long aIndex); */
                    Method {
                        name: "getChild",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsINavHistoryResultNode" }],
                        ret: "nsresult",
                    },

                    /* unsigned long getChildIndex (in nsINavHistoryResultNode aNode); */
                    Method {
                        name: "getChildIndex",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsINavHistoryResultNode findNodeByDetails (in AUTF8String aURIString, in PRTime aTime, in long long aItemId, in boolean aRecursive); */
                    Method {
                        name: "findNodeByDetails",
                        abi: "C",
                        params: &[Param { name: "aURIString", ty: "*const nsACString" }, Param { name: "aTime", ty: "PRTime" }, Param { name: "aItemId", ty: "libc::int64_t" }, Param { name: "aRecursive", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsINavHistoryResultNode" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINavHistoryQueryResultNode",
            base: Some("nsINavHistoryContainerResultNode"),
            methods: None,
        },


        Interface {
            name: "nsINavHistoryResultObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void nodeInserted (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aNode, in unsigned long aNewIndex); */
                    Method {
                        name: "nodeInserted",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*const nsINavHistoryContainerResultNode" }, Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aNewIndex", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void nodeRemoved (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aItem, in unsigned long aOldIndex); */
                    Method {
                        name: "nodeRemoved",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*const nsINavHistoryContainerResultNode" }, Param { name: "aItem", ty: "*const nsINavHistoryResultNode" }, Param { name: "aOldIndex", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void nodeMoved (in nsINavHistoryResultNode aNode, in nsINavHistoryContainerResultNode aOldParent, in unsigned long aOldIndex, in nsINavHistoryContainerResultNode aNewParent, in unsigned long aNewIndex); */
                    Method {
                        name: "nodeMoved",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aOldParent", ty: "*const nsINavHistoryContainerResultNode" }, Param { name: "aOldIndex", ty: "libc::uint32_t" }, Param { name: "aNewParent", ty: "*const nsINavHistoryContainerResultNode" }, Param { name: "aNewIndex", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void nodeTitleChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewTitle); */
                    Method {
                        name: "nodeTitleChanged",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aNewTitle", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void nodeURIChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewURI); */
                    Method {
                        name: "nodeURIChanged",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aNewURI", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void nodeIconChanged (in nsINavHistoryResultNode aNode); */
                    Method {
                        name: "nodeIconChanged",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }],
                        ret: "nsresult",
                    },

                    /* void nodeHistoryDetailsChanged (in nsINavHistoryResultNode aNode, in PRTime aNewVisitDate, in unsigned long aNewAccessCount); */
                    Method {
                        name: "nodeHistoryDetailsChanged",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aNewVisitDate", ty: "PRTime" }, Param { name: "aNewAccessCount", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void nodeTagsChanged (in nsINavHistoryResultNode aNode); */
                    Method {
                        name: "nodeTagsChanged",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }],
                        ret: "nsresult",
                    },

                    /* void nodeKeywordChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewKeyword); */
                    Method {
                        name: "nodeKeywordChanged",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aNewKeyword", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void nodeAnnotationChanged (in nsINavHistoryResultNode aNode, in AUTF8String aAnnoName); */
                    Method {
                        name: "nodeAnnotationChanged",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aAnnoName", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void nodeDateAddedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue); */
                    Method {
                        name: "nodeDateAddedChanged",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aNewValue", ty: "PRTime" }],
                        ret: "nsresult",
                    },

                    /* void nodeLastModifiedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue); */
                    Method {
                        name: "nodeLastModifiedChanged",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aNewValue", ty: "PRTime" }],
                        ret: "nsresult",
                    },

                    /* void containerStateChanged (in nsINavHistoryContainerResultNode aContainerNode, in unsigned long aOldState, in unsigned long aNewState); */
                    Method {
                        name: "containerStateChanged",
                        abi: "C",
                        params: &[Param { name: "aContainerNode", ty: "*const nsINavHistoryContainerResultNode" }, Param { name: "aOldState", ty: "libc::uint32_t" }, Param { name: "aNewState", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void invalidateContainer (in nsINavHistoryContainerResultNode aContainerNode); */
                    Method {
                        name: "invalidateContainer",
                        abi: "C",
                        params: &[Param { name: "aContainerNode", ty: "*const nsINavHistoryContainerResultNode" }],
                        ret: "nsresult",
                    },

                    /* void sortingChanged (in unsigned short sortingMode); */
                    Method {
                        name: "sortingChanged",
                        abi: "C",
                        params: &[Param { name: "sortingMode", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* void batching (in boolean aToggleMode); */
                    Method {
                        name: "batching",
                        abi: "C",
                        params: &[Param { name: "aToggleMode", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute nsINavHistoryResult result; */
                    Method {
                        name: "get_result",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "*mut *const nsINavHistoryResult" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_result",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "*const nsINavHistoryResult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINavHistoryResultTreeViewer",
            base: Some("nsINavHistoryResultObserver"),
            methods: Some(&[
                    /* nsINavHistoryResultNode nodeForTreeIndex (in unsigned long aIndex); */
                    Method {
                        name: "nodeForTreeIndex",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsINavHistoryResultNode" }],
                        ret: "nsresult",
                    },

                    /* unsigned long treeIndexForNode (in nsINavHistoryResultNode aNode); */
                    Method {
                        name: "treeIndexForNode",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINavHistoryResult",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute unsigned short sortingMode; */
                    Method {
                        name: "get_sortingMode",
                        abi: "C",
                        params: &[Param { name: "aSortingMode", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_sortingMode",
                        abi: "C",
                        params: &[Param { name: "aSortingMode", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String sortingAnnotation; */
                    Method {
                        name: "get_sortingAnnotation",
                        abi: "C",
                        params: &[Param { name: "aSortingAnnotation", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_sortingAnnotation",
                        abi: "C",
                        params: &[Param { name: "aSortingAnnotation", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean suppressNotifications; */
                    Method {
                        name: "get_suppressNotifications",
                        abi: "C",
                        params: &[Param { name: "aSuppressNotifications", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_suppressNotifications",
                        abi: "C",
                        params: &[Param { name: "aSuppressNotifications", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void addObserver (in nsINavHistoryResultObserver aObserver, [optional] in boolean aOwnsWeak); */
                    Method {
                        name: "addObserver",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsINavHistoryResultObserver" }, Param { name: "aOwnsWeak", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void removeObserver (in nsINavHistoryResultObserver aObserver); */
                    Method {
                        name: "removeObserver",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsINavHistoryResultObserver" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsINavHistoryContainerResultNode root; */
                    Method {
                        name: "get_root",
                        abi: "C",
                        params: &[Param { name: "aRoot", ty: "*mut *const nsINavHistoryContainerResultNode" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINavHistoryObserver",
            base: Some("nsISupports"),
            methods: Some(&[
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

                    /* void onVisit (in nsIURI aURI, in long long aVisitId, in PRTime aTime, in long long aSessionId, in long long aReferrerVisitId, in unsigned long aTransitionType, in ACString aGuid, in boolean aHidden, in unsigned long aVisitCount, in unsigned long aTyped, in AString aLastKnownTitle); */
                    Method {
                        name: "onVisit",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aVisitId", ty: "libc::int64_t" }, Param { name: "aTime", ty: "PRTime" }, Param { name: "aSessionId", ty: "libc::int64_t" }, Param { name: "aReferrerVisitId", ty: "libc::int64_t" }, Param { name: "aTransitionType", ty: "libc::uint32_t" }, Param { name: "aGuid", ty: "*const nsACString" }, Param { name: "aHidden", ty: "bool" }, Param { name: "aVisitCount", ty: "libc::uint32_t" }, Param { name: "aTyped", ty: "libc::uint32_t" }, Param { name: "aLastKnownTitle", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void onTitleChanged (in nsIURI aURI, in AString aPageTitle, in ACString aGUID); */
                    Method {
                        name: "onTitleChanged",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aPageTitle", ty: "*const nsAString" }, Param { name: "aGUID", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void onFrecencyChanged (in nsIURI aURI, in long aNewFrecency, in ACString aGUID, in boolean aHidden, in PRTime aVisitDate); */
                    Method {
                        name: "onFrecencyChanged",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aNewFrecency", ty: "libc::int32_t" }, Param { name: "aGUID", ty: "*const nsACString" }, Param { name: "aHidden", ty: "bool" }, Param { name: "aVisitDate", ty: "PRTime" }],
                        ret: "nsresult",
                    },

                    /* void onManyFrecenciesChanged (); */
                    Method {
                        name: "onManyFrecenciesChanged",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onDeleteURI (in nsIURI aURI, in ACString aGUID, in unsigned short aReason); */
                    Method {
                        name: "onDeleteURI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aGUID", ty: "*const nsACString" }, Param { name: "aReason", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* void onClearHistory (); */
                    Method {
                        name: "onClearHistory",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onPageChanged (in nsIURI aURI, in unsigned long aChangedAttribute, in AString aNewValue, in ACString aGUID); */
                    Method {
                        name: "onPageChanged",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aChangedAttribute", ty: "libc::uint32_t" }, Param { name: "aNewValue", ty: "*const nsAString" }, Param { name: "aGUID", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void onDeleteVisits (in nsIURI aURI, in PRTime aVisitTime, in ACString aGUID, in unsigned short aReason, in unsigned long aTransitionType); */
                    Method {
                        name: "onDeleteVisits",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aVisitTime", ty: "PRTime" }, Param { name: "aGUID", ty: "*const nsACString" }, Param { name: "aReason", ty: "libc::uint16_t" }, Param { name: "aTransitionType", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINavHistoryQuery",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsINavHistoryQueryOptions",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute unsigned short sortingMode; */
                    Method {
                        name: "get_sortingMode",
                        abi: "C",
                        params: &[Param { name: "aSortingMode", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_sortingMode",
                        abi: "C",
                        params: &[Param { name: "aSortingMode", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute AUTF8String sortingAnnotation; */
                    Method {
                        name: "get_sortingAnnotation",
                        abi: "C",
                        params: &[Param { name: "aSortingAnnotation", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_sortingAnnotation",
                        abi: "C",
                        params: &[Param { name: "aSortingAnnotation", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned short resultType; */
                    Method {
                        name: "get_resultType",
                        abi: "C",
                        params: &[Param { name: "aResultType", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_resultType",
                        abi: "C",
                        params: &[Param { name: "aResultType", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean excludeItems; */
                    Method {
                        name: "get_excludeItems",
                        abi: "C",
                        params: &[Param { name: "aExcludeItems", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_excludeItems",
                        abi: "C",
                        params: &[Param { name: "aExcludeItems", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean excludeQueries; */
                    Method {
                        name: "get_excludeQueries",
                        abi: "C",
                        params: &[Param { name: "aExcludeQueries", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_excludeQueries",
                        abi: "C",
                        params: &[Param { name: "aExcludeQueries", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean excludeReadOnlyFolders; */
                    Method {
                        name: "get_excludeReadOnlyFolders",
                        abi: "C",
                        params: &[Param { name: "aExcludeReadOnlyFolders", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_excludeReadOnlyFolders",
                        abi: "C",
                        params: &[Param { name: "aExcludeReadOnlyFolders", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean expandQueries; */
                    Method {
                        name: "get_expandQueries",
                        abi: "C",
                        params: &[Param { name: "aExpandQueries", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_expandQueries",
                        abi: "C",
                        params: &[Param { name: "aExpandQueries", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean includeHidden; */
                    Method {
                        name: "get_includeHidden",
                        abi: "C",
                        params: &[Param { name: "aIncludeHidden", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_includeHidden",
                        abi: "C",
                        params: &[Param { name: "aIncludeHidden", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long maxResults; */
                    Method {
                        name: "get_maxResults",
                        abi: "C",
                        params: &[Param { name: "aMaxResults", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_maxResults",
                        abi: "C",
                        params: &[Param { name: "aMaxResults", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned short queryType; */
                    Method {
                        name: "get_queryType",
                        abi: "C",
                        params: &[Param { name: "aQueryType", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_queryType",
                        abi: "C",
                        params: &[Param { name: "aQueryType", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean asyncEnabled; */
                    Method {
                        name: "get_asyncEnabled",
                        abi: "C",
                        params: &[Param { name: "aAsyncEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_asyncEnabled",
                        abi: "C",
                        params: &[Param { name: "aAsyncEnabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* nsINavHistoryQueryOptions clone (); */
                    Method {
                        name: "clone",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsINavHistoryQueryOptions" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsINavHistoryService",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsINavHistoryBatchCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void runBatched (in nsISupports aUserData); */
                    Method {
                        name: "runBatched",
                        abi: "C",
                        params: &[Param { name: "aUserData", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


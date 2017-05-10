//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocShellTreeItem.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDocShellTreeItem",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute AString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* boolean nameEquals (in AString name); */
                    Method {
                        name: "nameEquals",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* attribute long itemType; */
                    Method {
                        name: "get_itemType",
                        abi: "C",
                        params: &[Param { name: "aItemType", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_itemType",
                        abi: "C",
                        params: &[Param { name: "aItemType", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript,nostdcall,notxpcom] long ItemType (); */
                    Method {
                        name: "ItemType",
                        abi: "C",
                        params: &[],
                        ret: "libc::int32_t",
                    },

                    /* readonly attribute nsIDocShellTreeItem parent; */
                    Method {
                        name: "get_parent",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*mut *const nsIDocShellTreeItem" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDocShellTreeItem sameTypeParent; */
                    Method {
                        name: "get_sameTypeParent",
                        abi: "C",
                        params: &[Param { name: "aSameTypeParent", ty: "*mut *const nsIDocShellTreeItem" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDocShellTreeItem rootTreeItem; */
                    Method {
                        name: "get_rootTreeItem",
                        abi: "C",
                        params: &[Param { name: "aRootTreeItem", ty: "*mut *const nsIDocShellTreeItem" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDocShellTreeItem sameTypeRootTreeItem; */
                    Method {
                        name: "get_sameTypeRootTreeItem",
                        abi: "C",
                        params: &[Param { name: "aSameTypeRootTreeItem", ty: "*mut *const nsIDocShellTreeItem" }],
                        ret: "nsresult",
                    },

                    /* nsIDocShellTreeItem findItemWithName (in AString name, in nsIDocShellTreeItem aRequestor, in nsIDocShellTreeItem aOriginalRequestor, in bool aSkipTabGroup); */
                    Method {
                        name: "findItemWithName",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "aRequestor", ty: "*const nsIDocShellTreeItem" }, Param { name: "aOriginalRequestor", ty: "*const nsIDocShellTreeItem" }, Param { name: "aSkipTabGroup", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIDocShellTreeItem" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDocShellTreeOwner treeOwner; */
                    Method {
                        name: "get_treeOwner",
                        abi: "C",
                        params: &[Param { name: "aTreeOwner", ty: "*mut *const nsIDocShellTreeOwner" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void setTreeOwner (in nsIDocShellTreeOwner treeOwner); */
                    Method {
                        name: "setTreeOwner",
                        abi: "C",
                        params: &[Param { name: "treeOwner", ty: "*const nsIDocShellTreeOwner" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long childCount; */
                    Method {
                        name: "get_childCount",
                        abi: "C",
                        params: &[Param { name: "aChildCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void addChild (in nsIDocShellTreeItem child); */
                    Method {
                        name: "addChild",
                        abi: "C",
                        params: &[Param { name: "child", ty: "*const nsIDocShellTreeItem" }],
                        ret: "nsresult",
                    },

                    /* void removeChild (in nsIDocShellTreeItem child); */
                    Method {
                        name: "removeChild",
                        abi: "C",
                        params: &[Param { name: "child", ty: "*const nsIDocShellTreeItem" }],
                        ret: "nsresult",
                    },

                    /* nsIDocShellTreeItem getChildAt (in long index); */
                    Method {
                        name: "getChildAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIDocShellTreeItem" }],
                        ret: "nsresult",
                    },

                    /* nsIDocShellTreeItem findChildWithName (in AString aName, in boolean aRecurse, in boolean aSameType, in nsIDocShellTreeItem aRequestor, in nsIDocShellTreeItem aOriginalRequestor); */
                    Method {
                        name: "findChildWithName",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }, Param { name: "aRecurse", ty: "bool" }, Param { name: "aSameType", ty: "bool" }, Param { name: "aRequestor", ty: "*const nsIDocShellTreeItem" }, Param { name: "aOriginalRequestor", ty: "*const nsIDocShellTreeItem" }, Param { name: "_retval", ty: "*mut *const nsIDocShellTreeItem" }],
                        ret: "nsresult",
                    },

                    /* [noscript,nostdcall,notxpcom] nsIDocument getDocument (); */
                    Method {
                        name: "getDocument",
                        abi: "C",
                        params: &[],
                        ret: "*const nsIDocument",
                    },

                    /* [noscript,nostdcall,notxpcom] nsPIDOMWindowOuter getWindow (); */
                    Method {
                        name: "getWindow",
                        abi: "C",
                        params: &[],
                        ret: "*const nsPIDOMWindowOuter",
                    },

                    ]),
        },


        ]; D}


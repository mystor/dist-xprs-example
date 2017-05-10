//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessible.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessible",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIAccessible parent; */
                    Method {
                        name: "get_parent",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessible nextSibling; */
                    Method {
                        name: "get_nextSibling",
                        abi: "C",
                        params: &[Param { name: "aNextSibling", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessible previousSibling; */
                    Method {
                        name: "get_previousSibling",
                        abi: "C",
                        params: &[Param { name: "aPreviousSibling", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessible firstChild; */
                    Method {
                        name: "get_firstChild",
                        abi: "C",
                        params: &[Param { name: "aFirstChild", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessible lastChild; */
                    Method {
                        name: "get_lastChild",
                        abi: "C",
                        params: &[Param { name: "aLastChild", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray children; */
                    Method {
                        name: "get_children",
                        abi: "C",
                        params: &[Param { name: "aChildren", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long childCount; */
                    Method {
                        name: "get_childCount",
                        abi: "C",
                        params: &[Param { name: "aChildCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long indexInParent; */
                    Method {
                        name: "get_indexInParent",
                        abi: "C",
                        params: &[Param { name: "aIndexInParent", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMNode DOMNode; */
                    Method {
                        name: "get_DOMNode",
                        abi: "C",
                        params: &[Param { name: "aDOMNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString id; */
                    Method {
                        name: "get_id",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessibleDocument document; */
                    Method {
                        name: "get_document",
                        abi: "C",
                        params: &[Param { name: "aDocument", ty: "*mut *const nsIAccessibleDocument" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessibleDocument rootDocument; */
                    Method {
                        name: "get_rootDocument",
                        abi: "C",
                        params: &[Param { name: "aRootDocument", ty: "*mut *const nsIAccessibleDocument" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString language; */
                    Method {
                        name: "get_language",
                        abi: "C",
                        params: &[Param { name: "aLanguage", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString value; */
                    Method {
                        name: "get_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString description; */
                    Method {
                        name: "get_description",
                        abi: "C",
                        params: &[Param { name: "aDescription", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString accessKey; */
                    Method {
                        name: "get_accessKey",
                        abi: "C",
                        params: &[Param { name: "aAccessKey", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString keyboardShortcut; */
                    Method {
                        name: "get_keyboardShortcut",
                        abi: "C",
                        params: &[Param { name: "aKeyboardShortcut", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long role; */
                    Method {
                        name: "get_role",
                        abi: "C",
                        params: &[Param { name: "aRole", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void getState (out unsigned long aState, out unsigned long aExtraState); */
                    Method {
                        name: "getState",
                        abi: "C",
                        params: &[Param { name: "aState", ty: "*mut libc::uint32_t" }, Param { name: "aExtraState", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString help; */
                    Method {
                        name: "get_help",
                        abi: "C",
                        params: &[Param { name: "aHelp", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessible focusedChild; */
                    Method {
                        name: "get_focusedChild",
                        abi: "C",
                        params: &[Param { name: "aFocusedChild", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIPersistentProperties attributes; */
                    Method {
                        name: "get_attributes",
                        abi: "C",
                        params: &[Param { name: "aAttributes", ty: "*mut *const nsIPersistentProperties" }],
                        ret: "nsresult",
                    },

                    /* void groupPosition (out long aGroupLevel, out long aSimilarItemsInGroup, out long aPositionInGroup); */
                    Method {
                        name: "groupPosition",
                        abi: "C",
                        params: &[Param { name: "aGroupLevel", ty: "*mut libc::int32_t" }, Param { name: "aSimilarItemsInGroup", ty: "*mut libc::int32_t" }, Param { name: "aPositionInGroup", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessible getChildAtPoint (in long x, in long y); */
                    Method {
                        name: "getChildAtPoint",
                        abi: "C",
                        params: &[Param { name: "x", ty: "libc::int32_t" }, Param { name: "y", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessible getDeepestChildAtPoint (in long x, in long y); */
                    Method {
                        name: "getDeepestChildAtPoint",
                        abi: "C",
                        params: &[Param { name: "x", ty: "libc::int32_t" }, Param { name: "y", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessible getChildAt (in long aChildIndex); */
                    Method {
                        name: "getChildAt",
                        abi: "C",
                        params: &[Param { name: "aChildIndex", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessibleRelation getRelationByType (in unsigned long aRelationType); */
                    Method {
                        name: "getRelationByType",
                        abi: "C",
                        params: &[Param { name: "aRelationType", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIAccessibleRelation" }],
                        ret: "nsresult",
                    },

                    /* nsIArray getRelations (); */
                    Method {
                        name: "getRelations",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* void getBounds (out long x, out long y, out long width, out long height); */
                    Method {
                        name: "getBounds",
                        abi: "C",
                        params: &[Param { name: "x", ty: "*mut libc::int32_t" }, Param { name: "y", ty: "*mut libc::int32_t" }, Param { name: "width", ty: "*mut libc::int32_t" }, Param { name: "height", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setSelected (in boolean isSelected); */
                    Method {
                        name: "setSelected",
                        abi: "C",
                        params: &[Param { name: "isSelected", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void takeSelection (); */
                    Method {
                        name: "takeSelection",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void takeFocus (); */
                    Method {
                        name: "takeFocus",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint8_t actionCount; */
                    Method {
                        name: "get_actionCount",
                        abi: "C",
                        params: &[Param { name: "aActionCount", ty: "*mut uint8_t" }],
                        ret: "nsresult",
                    },

                    /* AString getActionName (in uint8_t index); */
                    Method {
                        name: "getActionName",
                        abi: "C",
                        params: &[Param { name: "index", ty: "uint8_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getActionDescription (in uint8_t aIndex); */
                    Method {
                        name: "getActionDescription",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "uint8_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void doAction (in uint8_t index); */
                    Method {
                        name: "doAction",
                        abi: "C",
                        params: &[Param { name: "index", ty: "uint8_t" }],
                        ret: "nsresult",
                    },

                    /* void scrollTo (in unsigned long aScrollType); */
                    Method {
                        name: "scrollTo",
                        abi: "C",
                        params: &[Param { name: "aScrollType", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void scrollToPoint (in unsigned long coordinateType, in long x, in long y); */
                    Method {
                        name: "scrollToPoint",
                        abi: "C",
                        params: &[Param { name: "coordinateType", ty: "libc::uint32_t" }, Param { name: "x", ty: "libc::int32_t" }, Param { name: "y", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


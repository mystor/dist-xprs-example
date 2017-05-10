//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMRange.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMRange",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMNode startContainer; */
                    Method {
                        name: "get_startContainer",
                        abi: "C",
                        params: &[Param { name: "aStartContainer", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long startOffset; */
                    Method {
                        name: "get_startOffset",
                        abi: "C",
                        params: &[Param { name: "aStartOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMNode endContainer; */
                    Method {
                        name: "get_endContainer",
                        abi: "C",
                        params: &[Param { name: "aEndContainer", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long endOffset; */
                    Method {
                        name: "get_endOffset",
                        abi: "C",
                        params: &[Param { name: "aEndOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean collapsed; */
                    Method {
                        name: "get_collapsed",
                        abi: "C",
                        params: &[Param { name: "aCollapsed", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMNode commonAncestorContainer; */
                    Method {
                        name: "get_commonAncestorContainer",
                        abi: "C",
                        params: &[Param { name: "aCommonAncestorContainer", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void setStart (in nsIDOMNode refNode, in long offset); */
                    Method {
                        name: "setStart",
                        abi: "C",
                        params: &[Param { name: "refNode", ty: "*const nsIDOMNode" }, Param { name: "offset", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setEnd (in nsIDOMNode refNode, in long offset); */
                    Method {
                        name: "setEnd",
                        abi: "C",
                        params: &[Param { name: "refNode", ty: "*const nsIDOMNode" }, Param { name: "offset", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setStartBefore (in nsIDOMNode refNode); */
                    Method {
                        name: "setStartBefore",
                        abi: "C",
                        params: &[Param { name: "refNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void setStartAfter (in nsIDOMNode refNode); */
                    Method {
                        name: "setStartAfter",
                        abi: "C",
                        params: &[Param { name: "refNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void setEndBefore (in nsIDOMNode refNode); */
                    Method {
                        name: "setEndBefore",
                        abi: "C",
                        params: &[Param { name: "refNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void setEndAfter (in nsIDOMNode refNode); */
                    Method {
                        name: "setEndAfter",
                        abi: "C",
                        params: &[Param { name: "refNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void collapse (in boolean toStart); */
                    Method {
                        name: "collapse",
                        abi: "C",
                        params: &[Param { name: "toStart", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void selectNode (in nsIDOMNode refNode); */
                    Method {
                        name: "selectNode",
                        abi: "C",
                        params: &[Param { name: "refNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void selectNodeContents (in nsIDOMNode refNode); */
                    Method {
                        name: "selectNodeContents",
                        abi: "C",
                        params: &[Param { name: "refNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* short compareBoundaryPoints (in unsigned short how, in nsIDOMRange sourceRange); */
                    Method {
                        name: "compareBoundaryPoints",
                        abi: "C",
                        params: &[Param { name: "how", ty: "libc::uint16_t" }, Param { name: "sourceRange", ty: "*const nsIDOMRange" }, Param { name: "_retval", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void deleteContents (); */
                    Method {
                        name: "deleteContents",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* nsIDOMDocumentFragment extractContents (); */
                    Method {
                        name: "extractContents",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMDocumentFragment" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMDocumentFragment cloneContents (); */
                    Method {
                        name: "cloneContents",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMDocumentFragment" }],
                        ret: "nsresult",
                    },

                    /* void insertNode (in nsIDOMNode newNode); */
                    Method {
                        name: "insertNode",
                        abi: "C",
                        params: &[Param { name: "newNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void surroundContents (in nsIDOMNode newParent); */
                    Method {
                        name: "surroundContents",
                        abi: "C",
                        params: &[Param { name: "newParent", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMRange cloneRange (); */
                    Method {
                        name: "cloneRange",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMRange" }],
                        ret: "nsresult",
                    },

                    /* DOMString toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void detach (); */
                    Method {
                        name: "detach",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* nsIDOMDocumentFragment createContextualFragment (in DOMString fragment); */
                    Method {
                        name: "createContextualFragment",
                        abi: "C",
                        params: &[Param { name: "fragment", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMDocumentFragment" }],
                        ret: "nsresult",
                    },

                    /* boolean isPointInRange (in nsIDOMNode parent, in long offset); */
                    Method {
                        name: "isPointInRange",
                        abi: "C",
                        params: &[Param { name: "parent", ty: "*const nsIDOMNode" }, Param { name: "offset", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* short comparePoint (in nsIDOMNode parent, in long offset); */
                    Method {
                        name: "comparePoint",
                        abi: "C",
                        params: &[Param { name: "parent", ty: "*const nsIDOMNode" }, Param { name: "offset", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* boolean intersectsNode (in nsIDOMNode node); */
                    Method {
                        name: "intersectsNode",
                        abi: "C",
                        params: &[Param { name: "node", ty: "*const nsIDOMNode" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMClientRectList getClientRects (); */
                    Method {
                        name: "getClientRects",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMClientRectList" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMClientRect getBoundingClientRect (); */
                    Method {
                        name: "getBoundingClientRect",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMClientRect" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


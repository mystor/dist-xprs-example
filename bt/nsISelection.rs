//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISelection.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISelection",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMNode anchorNode; */
                    Method {
                        name: "get_anchorNode",
                        abi: "C",
                        params: &[Param { name: "aAnchorNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long anchorOffset; */
                    Method {
                        name: "get_anchorOffset",
                        abi: "C",
                        params: &[Param { name: "aAnchorOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMNode focusNode; */
                    Method {
                        name: "get_focusNode",
                        abi: "C",
                        params: &[Param { name: "aFocusNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long focusOffset; */
                    Method {
                        name: "get_focusOffset",
                        abi: "C",
                        params: &[Param { name: "aFocusOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isCollapsed; */
                    Method {
                        name: "get_isCollapsed",
                        abi: "C",
                        params: &[Param { name: "aIsCollapsed", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript,nostdcall,notxpcom] boolean collapsed (); */
                    Method {
                        name: "collapsed",
                        abi: "C",
                        params: &[],
                        ret: "bool",
                    },

                    /* readonly attribute long rangeCount; */
                    Method {
                        name: "get_rangeCount",
                        abi: "C",
                        params: &[Param { name: "aRangeCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMRange getRangeAt (in long index); */
                    Method {
                        name: "getRangeAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMRange" }],
                        ret: "nsresult",
                    },

                    /* void collapse (in nsIDOMNode parentNode, in long offset); */
                    Method {
                        name: "collapse",
                        abi: "C",
                        params: &[Param { name: "parentNode", ty: "*const nsIDOMNode" }, Param { name: "offset", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void collapseNative (in nsINode parentNode, in long offset); */
                    Method {
                        name: "collapseNative",
                        abi: "C",
                        params: &[Param { name: "parentNode", ty: "*const nsINode" }, Param { name: "offset", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void extend (in nsIDOMNode parentNode, in long offset); */
                    Method {
                        name: "extend",
                        abi: "C",
                        params: &[Param { name: "parentNode", ty: "*const nsIDOMNode" }, Param { name: "offset", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void extendNative (in nsINode parentNode, in long offset); */
                    Method {
                        name: "extendNative",
                        abi: "C",
                        params: &[Param { name: "parentNode", ty: "*const nsINode" }, Param { name: "offset", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void collapseToStart (); */
                    Method {
                        name: "collapseToStart",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void collapseToEnd (); */
                    Method {
                        name: "collapseToEnd",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean containsNode (in nsIDOMNode node, in boolean partlyContained); */
                    Method {
                        name: "containsNode",
                        abi: "C",
                        params: &[Param { name: "node", ty: "*const nsIDOMNode" }, Param { name: "partlyContained", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void selectAllChildren (in nsIDOMNode parentNode); */
                    Method {
                        name: "selectAllChildren",
                        abi: "C",
                        params: &[Param { name: "parentNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void addRange (in nsIDOMRange range); */
                    Method {
                        name: "addRange",
                        abi: "C",
                        params: &[Param { name: "range", ty: "*const nsIDOMRange" }],
                        ret: "nsresult",
                    },

                    /* void removeRange (in nsIDOMRange range); */
                    Method {
                        name: "removeRange",
                        abi: "C",
                        params: &[Param { name: "range", ty: "*const nsIDOMRange" }],
                        ret: "nsresult",
                    },

                    /* void removeAllRanges (); */
                    Method {
                        name: "removeAllRanges",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void deleteFromDocument (); */
                    Method {
                        name: "deleteFromDocument",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* DOMString toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* void modify (in DOMString alter, in DOMString direction, in DOMString granularity); */
                    Method {
                        name: "modify",
                        abi: "C",
                        params: &[Param { name: "alter", ty: "*const nsAString" }, Param { name: "direction", ty: "*const nsAString" }, Param { name: "granularity", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


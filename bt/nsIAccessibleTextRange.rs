//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleTextRange.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleTextRange",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIAccessibleText startContainer; */
                    Method {
                        name: "get_startContainer",
                        abi: "C",
                        params: &[Param { name: "aStartContainer", ty: "*mut *const nsIAccessibleText" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long startOffset; */
                    Method {
                        name: "get_startOffset",
                        abi: "C",
                        params: &[Param { name: "aStartOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessibleText endContainer; */
                    Method {
                        name: "get_endContainer",
                        abi: "C",
                        params: &[Param { name: "aEndContainer", ty: "*mut *const nsIAccessibleText" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long endOffset; */
                    Method {
                        name: "get_endOffset",
                        abi: "C",
                        params: &[Param { name: "aEndOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessible container; */
                    Method {
                        name: "get_container",
                        abi: "C",
                        params: &[Param { name: "aContainer", ty: "*mut *const nsIAccessible" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray embeddedChildren; */
                    Method {
                        name: "get_embeddedChildren",
                        abi: "C",
                        params: &[Param { name: "aEmbeddedChildren", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* boolean compare (in nsIAccessibleTextRange aOtherRange); */
                    Method {
                        name: "compare",
                        abi: "C",
                        params: &[Param { name: "aOtherRange", ty: "*const nsIAccessibleTextRange" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* long compareEndPoints (in unsigned long aEndPoint, in nsIAccessibleTextRange aOtherRange, in unsigned long aOtherRangeEndPoint); */
                    Method {
                        name: "compareEndPoints",
                        abi: "C",
                        params: &[Param { name: "aEndPoint", ty: "libc::uint32_t" }, Param { name: "aOtherRange", ty: "*const nsIAccessibleTextRange" }, Param { name: "aOtherRangeEndPoint", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString text; */
                    Method {
                        name: "get_text",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray bounds; */
                    Method {
                        name: "get_bounds",
                        abi: "C",
                        params: &[Param { name: "aBounds", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* void move (in unsigned long aUnit, in long aCount); */
                    Method {
                        name: "move_",
                        abi: "C",
                        params: &[Param { name: "aUnit", ty: "libc::uint32_t" }, Param { name: "aCount", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void moveStart (in unsigned long aUnit, in long aCount); */
                    Method {
                        name: "moveStart",
                        abi: "C",
                        params: &[Param { name: "aUnit", ty: "libc::uint32_t" }, Param { name: "aCount", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void moveEnd (in unsigned long aUnit, in long aCount); */
                    Method {
                        name: "moveEnd",
                        abi: "C",
                        params: &[Param { name: "aUnit", ty: "libc::uint32_t" }, Param { name: "aCount", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void normalize (in unsigned long aUnit); */
                    Method {
                        name: "normalize",
                        abi: "C",
                        params: &[Param { name: "aUnit", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean crop (in nsIAccessible aContainer); */
                    Method {
                        name: "crop",
                        abi: "C",
                        params: &[Param { name: "aContainer", ty: "*const nsIAccessible" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessibleTextRange findText (in AString aText, in boolean aIsBackward, in boolean aIsIgnoreCase); */
                    Method {
                        name: "findText",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*const nsAString" }, Param { name: "aIsBackward", ty: "bool" }, Param { name: "aIsIgnoreCase", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIAccessibleTextRange" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessibleTextRange findAttr (in unsigned long aAttr, in nsIVariant aValue, in boolean aIsBackward); */
                    Method {
                        name: "findAttr",
                        abi: "C",
                        params: &[Param { name: "aAttr", ty: "libc::uint32_t" }, Param { name: "aValue", ty: "*const nsIVariant" }, Param { name: "aIsBackward", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIAccessibleTextRange" }],
                        ret: "nsresult",
                    },

                    /* void addToSelection (); */
                    Method {
                        name: "addToSelection",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void removeFromSelection (); */
                    Method {
                        name: "removeFromSelection",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void select (); */
                    Method {
                        name: "select",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void scrollIntoView (in unsigned long aHow); */
                    Method {
                        name: "scrollIntoView",
                        abi: "C",
                        params: &[Param { name: "aHow", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


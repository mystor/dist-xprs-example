//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleText.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleText",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute long caretOffset; */
                    Method {
                        name: "get_caretOffset",
                        abi: "C",
                        params: &[Param { name: "aCaretOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_caretOffset",
                        abi: "C",
                        params: &[Param { name: "aCaretOffset", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long characterCount; */
                    Method {
                        name: "get_characterCount",
                        abi: "C",
                        params: &[Param { name: "aCharacterCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long selectionCount; */
                    Method {
                        name: "get_selectionCount",
                        abi: "C",
                        params: &[Param { name: "aSelectionCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* AString getText (in long startOffset, in long endOffset); */
                    Method {
                        name: "getText",
                        abi: "C",
                        params: &[Param { name: "startOffset", ty: "libc::int32_t" }, Param { name: "endOffset", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getTextAfterOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
                    Method {
                        name: "getTextAfterOffset",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::int32_t" }, Param { name: "boundaryType", ty: "AccessibleTextBoundary" }, Param { name: "startOffset", ty: "*mut libc::int32_t" }, Param { name: "endOffset", ty: "*mut libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getTextAtOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
                    Method {
                        name: "getTextAtOffset",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::int32_t" }, Param { name: "boundaryType", ty: "AccessibleTextBoundary" }, Param { name: "startOffset", ty: "*mut libc::int32_t" }, Param { name: "endOffset", ty: "*mut libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getTextBeforeOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
                    Method {
                        name: "getTextBeforeOffset",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::int32_t" }, Param { name: "boundaryType", ty: "AccessibleTextBoundary" }, Param { name: "startOffset", ty: "*mut libc::int32_t" }, Param { name: "endOffset", ty: "*mut libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* wchar getCharacterAtOffset (in long offset); */
                    Method {
                        name: "getCharacterAtOffset",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* nsIPersistentProperties getTextAttributes (in boolean includeDefAttrs, in long offset, out long rangeStartOffset, out long rangeEndOffset); */
                    Method {
                        name: "getTextAttributes",
                        abi: "C",
                        params: &[Param { name: "includeDefAttrs", ty: "bool" }, Param { name: "offset", ty: "libc::int32_t" }, Param { name: "rangeStartOffset", ty: "*mut libc::int32_t" }, Param { name: "rangeEndOffset", ty: "*mut libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIPersistentProperties" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIPersistentProperties defaultTextAttributes; */
                    Method {
                        name: "get_defaultTextAttributes",
                        abi: "C",
                        params: &[Param { name: "aDefaultTextAttributes", ty: "*mut *const nsIPersistentProperties" }],
                        ret: "nsresult",
                    },

                    /* void getCharacterExtents (in long offset, out long x, out long y, out long width, out long height, in unsigned long coordType); */
                    Method {
                        name: "getCharacterExtents",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::int32_t" }, Param { name: "x", ty: "*mut libc::int32_t" }, Param { name: "y", ty: "*mut libc::int32_t" }, Param { name: "width", ty: "*mut libc::int32_t" }, Param { name: "height", ty: "*mut libc::int32_t" }, Param { name: "coordType", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void getRangeExtents (in long startOffset, in long endOffset, out long x, out long y, out long width, out long height, in unsigned long coordType); */
                    Method {
                        name: "getRangeExtents",
                        abi: "C",
                        params: &[Param { name: "startOffset", ty: "libc::int32_t" }, Param { name: "endOffset", ty: "libc::int32_t" }, Param { name: "x", ty: "*mut libc::int32_t" }, Param { name: "y", ty: "*mut libc::int32_t" }, Param { name: "width", ty: "*mut libc::int32_t" }, Param { name: "height", ty: "*mut libc::int32_t" }, Param { name: "coordType", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* long getOffsetAtPoint (in long x, in long y, in unsigned long coordType); */
                    Method {
                        name: "getOffsetAtPoint",
                        abi: "C",
                        params: &[Param { name: "x", ty: "libc::int32_t" }, Param { name: "y", ty: "libc::int32_t" }, Param { name: "coordType", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void getSelectionBounds (in long selectionNum, out long startOffset, out long endOffset); */
                    Method {
                        name: "getSelectionBounds",
                        abi: "C",
                        params: &[Param { name: "selectionNum", ty: "libc::int32_t" }, Param { name: "startOffset", ty: "*mut libc::int32_t" }, Param { name: "endOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setSelectionBounds (in long selectionNum, in long startOffset, in long endOffset); */
                    Method {
                        name: "setSelectionBounds",
                        abi: "C",
                        params: &[Param { name: "selectionNum", ty: "libc::int32_t" }, Param { name: "startOffset", ty: "libc::int32_t" }, Param { name: "endOffset", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void addSelection (in long startOffset, in long endOffset); */
                    Method {
                        name: "addSelection",
                        abi: "C",
                        params: &[Param { name: "startOffset", ty: "libc::int32_t" }, Param { name: "endOffset", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void removeSelection (in long selectionNum); */
                    Method {
                        name: "removeSelection",
                        abi: "C",
                        params: &[Param { name: "selectionNum", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void scrollSubstringTo (in long startIndex, in long endIndex, in unsigned long scrollType); */
                    Method {
                        name: "scrollSubstringTo",
                        abi: "C",
                        params: &[Param { name: "startIndex", ty: "libc::int32_t" }, Param { name: "endIndex", ty: "libc::int32_t" }, Param { name: "scrollType", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void scrollSubstringToPoint (in long startIndex, in long endIndex, in unsigned long coordinateType, in long x, in long y); */
                    Method {
                        name: "scrollSubstringToPoint",
                        abi: "C",
                        params: &[Param { name: "startIndex", ty: "libc::int32_t" }, Param { name: "endIndex", ty: "libc::int32_t" }, Param { name: "coordinateType", ty: "libc::uint32_t" }, Param { name: "x", ty: "libc::int32_t" }, Param { name: "y", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAccessibleTextRange enclosingRange; */
                    Method {
                        name: "get_enclosingRange",
                        abi: "C",
                        params: &[Param { name: "aEnclosingRange", ty: "*mut *const nsIAccessibleTextRange" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray selectionRanges; */
                    Method {
                        name: "get_selectionRanges",
                        abi: "C",
                        params: &[Param { name: "aSelectionRanges", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIArray visibleRanges; */
                    Method {
                        name: "get_visibleRanges",
                        abi: "C",
                        params: &[Param { name: "aVisibleRanges", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessibleTextRange getRangeByChild (in nsIAccessible child); */
                    Method {
                        name: "getRangeByChild",
                        abi: "C",
                        params: &[Param { name: "child", ty: "*const nsIAccessible" }, Param { name: "_retval", ty: "*mut *const nsIAccessibleTextRange" }],
                        ret: "nsresult",
                    },

                    /* nsIAccessibleTextRange getRangeAtPoint (in long x, in long y); */
                    Method {
                        name: "getRangeAtPoint",
                        abi: "C",
                        params: &[Param { name: "x", ty: "libc::int32_t" }, Param { name: "y", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIAccessibleTextRange" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


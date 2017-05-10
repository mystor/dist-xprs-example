//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISelectionController.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISelectionController",
            base: Some("nsISelectionDisplay"),
            methods: Some(&[
                    /* void setDisplaySelection (in short toggle); */
                    Method {
                        name: "setDisplaySelection",
                        abi: "C",
                        params: &[Param { name: "toggle", ty: "libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* short getDisplaySelection (); */
                    Method {
                        name: "getDisplaySelection",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* nsISelection getSelection (in short type); */
                    Method {
                        name: "getSelection",
                        abi: "C",
                        params: &[Param { name: "type_", ty: "libc::int16_t" }, Param { name: "_retval", ty: "*mut *const nsISelection" }],
                        ret: "nsresult",
                    },

                    /* void scrollSelectionIntoView (in short type, in short region, in short flags); */
                    Method {
                        name: "scrollSelectionIntoView",
                        abi: "C",
                        params: &[Param { name: "type_", ty: "libc::int16_t" }, Param { name: "region", ty: "libc::int16_t" }, Param { name: "flags", ty: "libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void repaintSelection (in short type); */
                    Method {
                        name: "repaintSelection",
                        abi: "C",
                        params: &[Param { name: "type_", ty: "libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void setCaretEnabled (in boolean enabled); */
                    Method {
                        name: "setCaretEnabled",
                        abi: "C",
                        params: &[Param { name: "enabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void setCaretReadOnly (in boolean readOnly); */
                    Method {
                        name: "setCaretReadOnly",
                        abi: "C",
                        params: &[Param { name: "readOnly", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* boolean getCaretEnabled (); */
                    Method {
                        name: "getCaretEnabled",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean caretVisible; */
                    Method {
                        name: "get_caretVisible",
                        abi: "C",
                        params: &[Param { name: "aCaretVisible", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void setCaretVisibilityDuringSelection (in boolean visibility); */
                    Method {
                        name: "setCaretVisibilityDuringSelection",
                        abi: "C",
                        params: &[Param { name: "visibility", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void characterMove (in boolean forward, in boolean extend); */
                    Method {
                        name: "characterMove",
                        abi: "C",
                        params: &[Param { name: "forward", ty: "bool" }, Param { name: "extend", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void physicalMove (in short direction, in short amount, in boolean extend); */
                    Method {
                        name: "physicalMove",
                        abi: "C",
                        params: &[Param { name: "direction", ty: "libc::int16_t" }, Param { name: "amount", ty: "libc::int16_t" }, Param { name: "extend", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void characterExtendForDelete (); */
                    Method {
                        name: "characterExtendForDelete",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* [noscript] void characterExtendForBackspace (); */
                    Method {
                        name: "characterExtendForBackspace",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void wordMove (in boolean forward, in boolean extend); */
                    Method {
                        name: "wordMove",
                        abi: "C",
                        params: &[Param { name: "forward", ty: "bool" }, Param { name: "extend", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void wordExtendForDelete (in boolean forward); */
                    Method {
                        name: "wordExtendForDelete",
                        abi: "C",
                        params: &[Param { name: "forward", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void lineMove (in boolean forward, in boolean extend); */
                    Method {
                        name: "lineMove",
                        abi: "C",
                        params: &[Param { name: "forward", ty: "bool" }, Param { name: "extend", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void intraLineMove (in boolean forward, in boolean extend); */
                    Method {
                        name: "intraLineMove",
                        abi: "C",
                        params: &[Param { name: "forward", ty: "bool" }, Param { name: "extend", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void pageMove (in boolean forward, in boolean extend); */
                    Method {
                        name: "pageMove",
                        abi: "C",
                        params: &[Param { name: "forward", ty: "bool" }, Param { name: "extend", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void completeScroll (in boolean forward); */
                    Method {
                        name: "completeScroll",
                        abi: "C",
                        params: &[Param { name: "forward", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void completeMove (in boolean forward, in boolean extend); */
                    Method {
                        name: "completeMove",
                        abi: "C",
                        params: &[Param { name: "forward", ty: "bool" }, Param { name: "extend", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void scrollPage (in boolean forward); */
                    Method {
                        name: "scrollPage",
                        abi: "C",
                        params: &[Param { name: "forward", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void scrollLine (in boolean forward); */
                    Method {
                        name: "scrollLine",
                        abi: "C",
                        params: &[Param { name: "forward", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void scrollCharacter (in boolean right); */
                    Method {
                        name: "scrollCharacter",
                        abi: "C",
                        params: &[Param { name: "right", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void selectAll (); */
                    Method {
                        name: "selectAll",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean checkVisibility (in nsIDOMNode node, in short startOffset, in short endOffset); */
                    Method {
                        name: "checkVisibility",
                        abi: "C",
                        params: &[Param { name: "node", ty: "*const nsIDOMNode" }, Param { name: "startOffset", ty: "libc::int16_t" }, Param { name: "endOffset", ty: "libc::int16_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* [noscript,nostdcall] boolean checkVisibilityContent (in nsIContent node, in short startOffset, in short endOffset); */
                    Method {
                        name: "checkVisibilityContent",
                        abi: "C",
                        params: &[Param { name: "node", ty: "*const nsIContent" }, Param { name: "startOffset", ty: "libc::int16_t" }, Param { name: "endOffset", ty: "libc::int16_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


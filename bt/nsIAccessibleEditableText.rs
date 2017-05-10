//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleEditableText.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleEditableText",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setTextContents (in AString text); */
                    Method {
                        name: "setTextContents",
                        abi: "C",
                        params: &[Param { name: "text", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void insertText (in AString text, in long position); */
                    Method {
                        name: "insertText",
                        abi: "C",
                        params: &[Param { name: "text", ty: "*const nsAString" }, Param { name: "position", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void copyText (in long startPos, in long endPos); */
                    Method {
                        name: "copyText",
                        abi: "C",
                        params: &[Param { name: "startPos", ty: "libc::int32_t" }, Param { name: "endPos", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void cutText (in long startPos, in long endPos); */
                    Method {
                        name: "cutText",
                        abi: "C",
                        params: &[Param { name: "startPos", ty: "libc::int32_t" }, Param { name: "endPos", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void deleteText (in long startPos, in long endPos); */
                    Method {
                        name: "deleteText",
                        abi: "C",
                        params: &[Param { name: "startPos", ty: "libc::int32_t" }, Param { name: "endPos", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void pasteText (in long position); */
                    Method {
                        name: "pasteText",
                        abi: "C",
                        params: &[Param { name: "position", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


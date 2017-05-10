//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPlaintextEditor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPlaintextEditor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute long textLength; */
                    Method {
                        name: "get_textLength",
                        abi: "C",
                        params: &[Param { name: "aTextLength", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute long maxTextLength; */
                    Method {
                        name: "get_maxTextLength",
                        abi: "C",
                        params: &[Param { name: "aMaxTextLength", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_maxTextLength",
                        abi: "C",
                        params: &[Param { name: "aMaxTextLength", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute long wrapWidth; */
                    Method {
                        name: "get_wrapWidth",
                        abi: "C",
                        params: &[Param { name: "aWrapWidth", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_wrapWidth",
                        abi: "C",
                        params: &[Param { name: "aWrapWidth", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setWrapColumn (in long aWrapColumn); */
                    Method {
                        name: "setWrapColumn",
                        abi: "C",
                        params: &[Param { name: "aWrapColumn", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute long newlineHandling; */
                    Method {
                        name: "get_newlineHandling",
                        abi: "C",
                        params: &[Param { name: "aNewlineHandling", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_newlineHandling",
                        abi: "C",
                        params: &[Param { name: "aNewlineHandling", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void insertText (in DOMString aStringToInsert); */
                    Method {
                        name: "insertText",
                        abi: "C",
                        params: &[Param { name: "aStringToInsert", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void insertLineBreak (); */
                    Method {
                        name: "insertLineBreak",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


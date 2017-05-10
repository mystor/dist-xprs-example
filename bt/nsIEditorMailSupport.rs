//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditorMailSupport.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEditorMailSupport",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void pasteAsQuotation (in long aSelectionType); */
                    Method {
                        name: "pasteAsQuotation",
                        abi: "C",
                        params: &[Param { name: "aSelectionType", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode insertAsQuotation (in AString aQuotedText); */
                    Method {
                        name: "insertAsQuotation",
                        abi: "C",
                        params: &[Param { name: "aQuotedText", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void insertTextWithQuotations (in DOMString aStringToInsert); */
                    Method {
                        name: "insertTextWithQuotations",
                        abi: "C",
                        params: &[Param { name: "aStringToInsert", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void pasteAsCitedQuotation (in AString aCitation, in long aSelectionType); */
                    Method {
                        name: "pasteAsCitedQuotation",
                        abi: "C",
                        params: &[Param { name: "aCitation", ty: "*const nsAString" }, Param { name: "aSelectionType", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode insertAsCitedQuotation (in AString aQuotedText, in AString aCitation, in boolean aInsertHTML); */
                    Method {
                        name: "insertAsCitedQuotation",
                        abi: "C",
                        params: &[Param { name: "aQuotedText", ty: "*const nsAString" }, Param { name: "aCitation", ty: "*const nsAString" }, Param { name: "aInsertHTML", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void rewrap (in boolean aRespectNewlines); */
                    Method {
                        name: "rewrap",
                        abi: "C",
                        params: &[Param { name: "aRespectNewlines", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void stripCites (); */
                    Method {
                        name: "stripCites",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* nsIArray getEmbeddedObjects (); */
                    Method {
                        name: "getEmbeddedObjects",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


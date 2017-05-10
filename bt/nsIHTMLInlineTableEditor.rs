//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHTMLInlineTableEditor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHTMLInlineTableEditor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute boolean inlineTableEditingEnabled; */
                    Method {
                        name: "get_inlineTableEditingEnabled",
                        abi: "C",
                        params: &[Param { name: "aInlineTableEditingEnabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_inlineTableEditingEnabled",
                        abi: "C",
                        params: &[Param { name: "aInlineTableEditingEnabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void showInlineTableEditingUI (in nsIDOMElement aCell); */
                    Method {
                        name: "showInlineTableEditingUI",
                        abi: "C",
                        params: &[Param { name: "aCell", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void hideInlineTableEditingUI (); */
                    Method {
                        name: "hideInlineTableEditingUI",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void doInlineTableEditingAction (in nsIDOMElement aUIAnonymousElement); */
                    Method {
                        name: "doInlineTableEditingAction",
                        abi: "C",
                        params: &[Param { name: "aUIAnonymousElement", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void refreshInlineTableEditingUI (); */
                    Method {
                        name: "refreshInlineTableEditingUI",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


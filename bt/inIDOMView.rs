//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/inIDOMView.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "inIDOMView",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIDOMNode rootNode; */
                    Method {
                        name: "get_rootNode",
                        abi: "C",
                        params: &[Param { name: "aRootNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_rootNode",
                        abi: "C",
                        params: &[Param { name: "aRootNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean showAnonymousContent; */
                    Method {
                        name: "get_showAnonymousContent",
                        abi: "C",
                        params: &[Param { name: "aShowAnonymousContent", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_showAnonymousContent",
                        abi: "C",
                        params: &[Param { name: "aShowAnonymousContent", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean showSubDocuments; */
                    Method {
                        name: "get_showSubDocuments",
                        abi: "C",
                        params: &[Param { name: "aShowSubDocuments", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_showSubDocuments",
                        abi: "C",
                        params: &[Param { name: "aShowSubDocuments", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean showWhitespaceNodes; */
                    Method {
                        name: "get_showWhitespaceNodes",
                        abi: "C",
                        params: &[Param { name: "aShowWhitespaceNodes", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_showWhitespaceNodes",
                        abi: "C",
                        params: &[Param { name: "aShowWhitespaceNodes", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean showAccessibleNodes; */
                    Method {
                        name: "get_showAccessibleNodes",
                        abi: "C",
                        params: &[Param { name: "aShowAccessibleNodes", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_showAccessibleNodes",
                        abi: "C",
                        params: &[Param { name: "aShowAccessibleNodes", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long whatToShow; */
                    Method {
                        name: "get_whatToShow",
                        abi: "C",
                        params: &[Param { name: "aWhatToShow", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_whatToShow",
                        abi: "C",
                        params: &[Param { name: "aWhatToShow", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode getNodeFromRowIndex (in long rowIndex); */
                    Method {
                        name: "getNodeFromRowIndex",
                        abi: "C",
                        params: &[Param { name: "rowIndex", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* long getRowIndexFromNode (in nsIDOMNode node); */
                    Method {
                        name: "getRowIndexFromNode",
                        abi: "C",
                        params: &[Param { name: "node", ty: "*const nsIDOMNode" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void rebuild (); */
                    Method {
                        name: "rebuild",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


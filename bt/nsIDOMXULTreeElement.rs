//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULTreeElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULTreeElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsITreeColumns columns; */
                    Method {
                        name: "get_columns",
                        abi: "C",
                        params: &[Param { name: "aColumns", ty: "*mut *const nsITreeColumns" }],
                        ret: "nsresult",
                    },

                    /* attribute nsITreeView view; */
                    Method {
                        name: "get_view",
                        abi: "C",
                        params: &[Param { name: "aView", ty: "*mut *const nsITreeView" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_view",
                        abi: "C",
                        params: &[Param { name: "aView", ty: "*const nsITreeView" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMElement body; */
                    Method {
                        name: "get_body",
                        abi: "C",
                        params: &[Param { name: "aBody", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean editable; */
                    Method {
                        name: "get_editable",
                        abi: "C",
                        params: &[Param { name: "aEditable", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_editable",
                        abi: "C",
                        params: &[Param { name: "aEditable", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMXULTextBoxElement inputField; */
                    Method {
                        name: "get_inputField",
                        abi: "C",
                        params: &[Param { name: "aInputField", ty: "*mut *const nsIDOMXULTextBoxElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


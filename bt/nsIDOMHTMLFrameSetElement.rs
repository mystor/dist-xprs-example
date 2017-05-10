//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLFrameSetElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLFrameSetElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString cols; */
                    Method {
                        name: "get_cols",
                        abi: "C",
                        params: &[Param { name: "aCols", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_cols",
                        abi: "C",
                        params: &[Param { name: "aCols", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString rows; */
                    Method {
                        name: "get_rows",
                        abi: "C",
                        params: &[Param { name: "aRows", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_rows",
                        abi: "C",
                        params: &[Param { name: "aRows", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


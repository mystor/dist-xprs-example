//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMAttr.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMAttr",
            base: Some("nsIDOMNode"),
            methods: Some(&[
                    /* readonly attribute DOMString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean specified; */
                    Method {
                        name: "get_specified",
                        abi: "C",
                        params: &[Param { name: "aSpecified", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString value; */
                    Method {
                        name: "get_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMElement ownerElement; */
                    Method {
                        name: "get_ownerElement",
                        abi: "C",
                        params: &[Param { name: "aOwnerElement", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isId; */
                    Method {
                        name: "get_isId",
                        abi: "C",
                        params: &[Param { name: "aIsId", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


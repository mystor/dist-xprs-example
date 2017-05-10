//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCounter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMCounter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute DOMString identifier; */
                    Method {
                        name: "get_identifier",
                        abi: "C",
                        params: &[Param { name: "aIdentifier", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString listStyle; */
                    Method {
                        name: "get_listStyle",
                        abi: "C",
                        params: &[Param { name: "aListStyle", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString separator; */
                    Method {
                        name: "get_separator",
                        abi: "C",
                        params: &[Param { name: "aSeparator", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


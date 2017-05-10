//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLOptGroupElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLOptGroupElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute boolean disabled; */
                    Method {
                        name: "get_disabled",
                        abi: "C",
                        params: &[Param { name: "aDisabled", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_disabled",
                        abi: "C",
                        params: &[Param { name: "aDisabled", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString label; */
                    Method {
                        name: "get_label",
                        abi: "C",
                        params: &[Param { name: "aLabel", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_label",
                        abi: "C",
                        params: &[Param { name: "aLabel", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


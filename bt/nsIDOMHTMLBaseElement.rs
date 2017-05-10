//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLBaseElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLBaseElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString href; */
                    Method {
                        name: "get_href",
                        abi: "C",
                        params: &[Param { name: "aHref", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_href",
                        abi: "C",
                        params: &[Param { name: "aHref", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString target; */
                    Method {
                        name: "get_target",
                        abi: "C",
                        params: &[Param { name: "aTarget", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_target",
                        abi: "C",
                        params: &[Param { name: "aTarget", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


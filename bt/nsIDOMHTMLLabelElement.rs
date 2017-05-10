//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLLabelElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLLabelElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMHTMLFormElement form; */
                    Method {
                        name: "get_form",
                        abi: "C",
                        params: &[Param { name: "aForm", ty: "*mut *const nsIDOMHTMLFormElement" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString htmlFor; */
                    Method {
                        name: "get_htmlFor",
                        abi: "C",
                        params: &[Param { name: "aHtmlFor", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_htmlFor",
                        abi: "C",
                        params: &[Param { name: "aHtmlFor", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMHTMLElement control; */
                    Method {
                        name: "get_control",
                        abi: "C",
                        params: &[Param { name: "aControl", ty: "*mut *const nsIDOMHTMLElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


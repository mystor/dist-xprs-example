//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLFieldSetElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLFieldSetElement",
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

                    /* readonly attribute nsIDOMHTMLFormElement form; */
                    Method {
                        name: "get_form",
                        abi: "C",
                        params: &[Param { name: "aForm", ty: "*mut *const nsIDOMHTMLFormElement" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMHTMLCollection elements; */
                    Method {
                        name: "get_elements",
                        abi: "C",
                        params: &[Param { name: "aElements", ty: "*mut *const nsIDOMHTMLCollection" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean willValidate; */
                    Method {
                        name: "get_willValidate",
                        abi: "C",
                        params: &[Param { name: "aWillValidate", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMValidityState validity; */
                    Method {
                        name: "get_validity",
                        abi: "C",
                        params: &[Param { name: "aValidity", ty: "*mut *const nsIDOMValidityState" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString validationMessage; */
                    Method {
                        name: "get_validationMessage",
                        abi: "C",
                        params: &[Param { name: "aValidationMessage", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* boolean checkValidity (); */
                    Method {
                        name: "checkValidity",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void setCustomValidity (in DOMString error); */
                    Method {
                        name: "setCustomValidity",
                        abi: "C",
                        params: &[Param { name: "error", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


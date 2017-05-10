//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLButtonElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLButtonElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute boolean autofocus; */
                    Method {
                        name: "get_autofocus",
                        abi: "C",
                        params: &[Param { name: "aAutofocus", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_autofocus",
                        abi: "C",
                        params: &[Param { name: "aAutofocus", ty: "bool" }],
                        ret: "nsresult",
                    },

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

                    /* attribute DOMString formAction; */
                    Method {
                        name: "get_formAction",
                        abi: "C",
                        params: &[Param { name: "aFormAction", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_formAction",
                        abi: "C",
                        params: &[Param { name: "aFormAction", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString formEnctype; */
                    Method {
                        name: "get_formEnctype",
                        abi: "C",
                        params: &[Param { name: "aFormEnctype", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_formEnctype",
                        abi: "C",
                        params: &[Param { name: "aFormEnctype", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString formMethod; */
                    Method {
                        name: "get_formMethod",
                        abi: "C",
                        params: &[Param { name: "aFormMethod", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_formMethod",
                        abi: "C",
                        params: &[Param { name: "aFormMethod", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean formNoValidate; */
                    Method {
                        name: "get_formNoValidate",
                        abi: "C",
                        params: &[Param { name: "aFormNoValidate", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_formNoValidate",
                        abi: "C",
                        params: &[Param { name: "aFormNoValidate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString formTarget; */
                    Method {
                        name: "get_formTarget",
                        abi: "C",
                        params: &[Param { name: "aFormTarget", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_formTarget",
                        abi: "C",
                        params: &[Param { name: "aFormTarget", ty: "*const nsAString" }],
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

                    /* attribute DOMString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*const nsAString" }],
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


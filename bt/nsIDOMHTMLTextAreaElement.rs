//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLTextAreaElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLTextAreaElement",
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

                    /* attribute unsigned long cols; */
                    Method {
                        name: "get_cols",
                        abi: "C",
                        params: &[Param { name: "aCols", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_cols",
                        abi: "C",
                        params: &[Param { name: "aCols", ty: "libc::uint32_t" }],
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

                    /* attribute long maxLength; */
                    Method {
                        name: "get_maxLength",
                        abi: "C",
                        params: &[Param { name: "aMaxLength", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_maxLength",
                        abi: "C",
                        params: &[Param { name: "aMaxLength", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute long minLength; */
                    Method {
                        name: "get_minLength",
                        abi: "C",
                        params: &[Param { name: "aMinLength", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_minLength",
                        abi: "C",
                        params: &[Param { name: "aMinLength", ty: "libc::int32_t" }],
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

                    /* attribute DOMString placeholder; */
                    Method {
                        name: "get_placeholder",
                        abi: "C",
                        params: &[Param { name: "aPlaceholder", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_placeholder",
                        abi: "C",
                        params: &[Param { name: "aPlaceholder", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean readOnly; */
                    Method {
                        name: "get_readOnly",
                        abi: "C",
                        params: &[Param { name: "aReadOnly", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_readOnly",
                        abi: "C",
                        params: &[Param { name: "aReadOnly", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean required; */
                    Method {
                        name: "get_required",
                        abi: "C",
                        params: &[Param { name: "aRequired", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_required",
                        abi: "C",
                        params: &[Param { name: "aRequired", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long rows; */
                    Method {
                        name: "get_rows",
                        abi: "C",
                        params: &[Param { name: "aRows", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_rows",
                        abi: "C",
                        params: &[Param { name: "aRows", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* [Null(Stringify)] attribute DOMString wrap; */
                    Method {
                        name: "get_wrap",
                        abi: "C",
                        params: &[Param { name: "aWrap", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_wrap",
                        abi: "C",
                        params: &[Param { name: "aWrap", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString defaultValue; */
                    Method {
                        name: "get_defaultValue",
                        abi: "C",
                        params: &[Param { name: "aDefaultValue", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_defaultValue",
                        abi: "C",
                        params: &[Param { name: "aDefaultValue", ty: "*const nsAString" }],
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

                    /* readonly attribute long textLength; */
                    Method {
                        name: "get_textLength",
                        abi: "C",
                        params: &[Param { name: "aTextLength", ty: "*mut libc::int32_t" }],
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

                    /* void select (); */
                    Method {
                        name: "select",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIControllers controllers; */
                    Method {
                        name: "get_controllers",
                        abi: "C",
                        params: &[Param { name: "aControllers", ty: "*mut *const nsIControllers" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


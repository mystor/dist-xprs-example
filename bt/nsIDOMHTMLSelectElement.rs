//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLSelectElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLSelectElement",
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

                    /* attribute boolean multiple; */
                    Method {
                        name: "get_multiple",
                        abi: "C",
                        params: &[Param { name: "aMultiple", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_multiple",
                        abi: "C",
                        params: &[Param { name: "aMultiple", ty: "bool" }],
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

                    /* attribute unsigned long size; */
                    Method {
                        name: "get_size",
                        abi: "C",
                        params: &[Param { name: "aSize", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_size",
                        abi: "C",
                        params: &[Param { name: "aSize", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMHTMLOptionsCollection options; */
                    Method {
                        name: "get_options",
                        abi: "C",
                        params: &[Param { name: "aOptions", ty: "*mut *const nsIDOMHTMLOptionsCollection" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode item (in unsigned long index); */
                    Method {
                        name: "item",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMNode namedItem (in DOMString name); */
                    Method {
                        name: "namedItem",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void add (in nsIDOMHTMLElement element, [optional] in nsIVariant before) raises (DOMException); */
                    Method {
                        name: "add",
                        abi: "C",
                        params: &[Param { name: "element", ty: "*const nsIDOMHTMLElement" }, Param { name: "before", ty: "*const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* void remove (in long index); */
                    Method {
                        name: "remove",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMHTMLCollection selectedOptions; */
                    Method {
                        name: "get_selectedOptions",
                        abi: "C",
                        params: &[Param { name: "aSelectedOptions", ty: "*mut *const nsIDOMHTMLCollection" }],
                        ret: "nsresult",
                    },

                    /* attribute long selectedIndex; */
                    Method {
                        name: "get_selectedIndex",
                        abi: "C",
                        params: &[Param { name: "aSelectedIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selectedIndex",
                        abi: "C",
                        params: &[Param { name: "aSelectedIndex", ty: "libc::int32_t" }],
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

                    ]),
        },


        ]; D}


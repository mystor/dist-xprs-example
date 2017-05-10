//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLObjectElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMHTMLObjectElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMHTMLFormElement form; */
                    Method {
                        name: "get_form",
                        abi: "C",
                        params: &[Param { name: "aForm", ty: "*mut *const nsIDOMHTMLFormElement" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString code; */
                    Method {
                        name: "get_code",
                        abi: "C",
                        params: &[Param { name: "aCode", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_code",
                        abi: "C",
                        params: &[Param { name: "aCode", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString align; */
                    Method {
                        name: "get_align",
                        abi: "C",
                        params: &[Param { name: "aAlign", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_align",
                        abi: "C",
                        params: &[Param { name: "aAlign", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString archive; */
                    Method {
                        name: "get_archive",
                        abi: "C",
                        params: &[Param { name: "aArchive", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_archive",
                        abi: "C",
                        params: &[Param { name: "aArchive", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString border; */
                    Method {
                        name: "get_border",
                        abi: "C",
                        params: &[Param { name: "aBorder", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_border",
                        abi: "C",
                        params: &[Param { name: "aBorder", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString codeBase; */
                    Method {
                        name: "get_codeBase",
                        abi: "C",
                        params: &[Param { name: "aCodeBase", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_codeBase",
                        abi: "C",
                        params: &[Param { name: "aCodeBase", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString codeType; */
                    Method {
                        name: "get_codeType",
                        abi: "C",
                        params: &[Param { name: "aCodeType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_codeType",
                        abi: "C",
                        params: &[Param { name: "aCodeType", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean declare; */
                    Method {
                        name: "get_declare",
                        abi: "C",
                        params: &[Param { name: "aDeclare", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_declare",
                        abi: "C",
                        params: &[Param { name: "aDeclare", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString height; */
                    Method {
                        name: "get_height",
                        abi: "C",
                        params: &[Param { name: "aHeight", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_height",
                        abi: "C",
                        params: &[Param { name: "aHeight", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute long hspace; */
                    Method {
                        name: "get_hspace",
                        abi: "C",
                        params: &[Param { name: "aHspace", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_hspace",
                        abi: "C",
                        params: &[Param { name: "aHspace", ty: "libc::int32_t" }],
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

                    /* attribute DOMString standby; */
                    Method {
                        name: "get_standby",
                        abi: "C",
                        params: &[Param { name: "aStandby", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_standby",
                        abi: "C",
                        params: &[Param { name: "aStandby", ty: "*const nsAString" }],
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

                    /* attribute DOMString useMap; */
                    Method {
                        name: "get_useMap",
                        abi: "C",
                        params: &[Param { name: "aUseMap", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_useMap",
                        abi: "C",
                        params: &[Param { name: "aUseMap", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute long vspace; */
                    Method {
                        name: "get_vspace",
                        abi: "C",
                        params: &[Param { name: "aVspace", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_vspace",
                        abi: "C",
                        params: &[Param { name: "aVspace", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString width; */
                    Method {
                        name: "get_width",
                        abi: "C",
                        params: &[Param { name: "aWidth", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_width",
                        abi: "C",
                        params: &[Param { name: "aWidth", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMDocument contentDocument; */
                    Method {
                        name: "get_contentDocument",
                        abi: "C",
                        params: &[Param { name: "aContentDocument", ty: "*mut *const nsIDOMDocument" }],
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


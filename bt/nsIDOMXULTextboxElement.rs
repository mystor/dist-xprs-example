//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULTextboxElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULTextBoxElement",
            base: Some("nsIDOMXULControlElement"),
            methods: Some(&[
                    /* readonly attribute nsIDOMNode inputField; */
                    Method {
                        name: "get_inputField",
                        abi: "C",
                        params: &[Param { name: "aInputField", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long textLength; */
                    Method {
                        name: "get_textLength",
                        abi: "C",
                        params: &[Param { name: "aTextLength", ty: "*mut libc::int32_t" }],
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

                    /* attribute long size; */
                    Method {
                        name: "get_size",
                        abi: "C",
                        params: &[Param { name: "aSize", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_size",
                        abi: "C",
                        params: &[Param { name: "aSize", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute long selectionStart; */
                    Method {
                        name: "get_selectionStart",
                        abi: "C",
                        params: &[Param { name: "aSelectionStart", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selectionStart",
                        abi: "C",
                        params: &[Param { name: "aSelectionStart", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute long selectionEnd; */
                    Method {
                        name: "get_selectionEnd",
                        abi: "C",
                        params: &[Param { name: "aSelectionEnd", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selectionEnd",
                        abi: "C",
                        params: &[Param { name: "aSelectionEnd", ty: "libc::int32_t" }],
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

                    /* void select (); */
                    Method {
                        name: "select",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void setSelectionRange (in long selectionStart, in long selectionEnd); */
                    Method {
                        name: "setSelectionRange",
                        abi: "C",
                        params: &[Param { name: "selectionStart", ty: "libc::int32_t" }, Param { name: "selectionEnd", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


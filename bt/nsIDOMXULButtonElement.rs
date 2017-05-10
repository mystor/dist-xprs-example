//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULButtonElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULButtonElement",
            base: Some("nsIDOMXULLabeledControlElement"),
            methods: Some(&[
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

                    /* attribute DOMString dlgType; */
                    Method {
                        name: "get_dlgType",
                        abi: "C",
                        params: &[Param { name: "aDlgType", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_dlgType",
                        abi: "C",
                        params: &[Param { name: "aDlgType", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean open; */
                    Method {
                        name: "get_open",
                        abi: "C",
                        params: &[Param { name: "aOpen", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_open",
                        abi: "C",
                        params: &[Param { name: "aOpen", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean checked; */
                    Method {
                        name: "get_checked",
                        abi: "C",
                        params: &[Param { name: "aChecked", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_checked",
                        abi: "C",
                        params: &[Param { name: "aChecked", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute long checkState; */
                    Method {
                        name: "get_checkState",
                        abi: "C",
                        params: &[Param { name: "aCheckState", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_checkState",
                        abi: "C",
                        params: &[Param { name: "aCheckState", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean autoCheck; */
                    Method {
                        name: "get_autoCheck",
                        abi: "C",
                        params: &[Param { name: "aAutoCheck", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_autoCheck",
                        abi: "C",
                        params: &[Param { name: "aAutoCheck", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute DOMString group; */
                    Method {
                        name: "get_group",
                        abi: "C",
                        params: &[Param { name: "aGroup", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_group",
                        abi: "C",
                        params: &[Param { name: "aGroup", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


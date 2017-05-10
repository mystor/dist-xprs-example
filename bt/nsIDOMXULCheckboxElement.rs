//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULCheckboxElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULCheckboxElement",
            base: Some("nsIDOMXULLabeledControlElement"),
            methods: Some(&[
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

                    ]),
        },


        ]; D}


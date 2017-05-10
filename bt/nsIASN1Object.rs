//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIASN1Object.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIASN1Object",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute unsigned long type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long tag; */
                    Method {
                        name: "get_tag",
                        abi: "C",
                        params: &[Param { name: "aTag", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_tag",
                        abi: "C",
                        params: &[Param { name: "aTag", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute AString displayName; */
                    Method {
                        name: "get_displayName",
                        abi: "C",
                        params: &[Param { name: "aDisplayName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_displayName",
                        abi: "C",
                        params: &[Param { name: "aDisplayName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString displayValue; */
                    Method {
                        name: "get_displayValue",
                        abi: "C",
                        params: &[Param { name: "aDisplayValue", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_displayValue",
                        abi: "C",
                        params: &[Param { name: "aDisplayValue", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDialogParamBlock.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDialogParamBlock",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* int32_t GetInt (in int32_t inIndex); */
                    Method {
                        name: "GetInt",
                        abi: "C",
                        params: &[Param { name: "inIndex", ty: "int32_t" }, Param { name: "_retval", ty: "*mut int32_t" }],
                        ret: "nsresult",
                    },

                    /* void SetInt (in int32_t inIndex, in int32_t inInt); */
                    Method {
                        name: "SetInt",
                        abi: "C",
                        params: &[Param { name: "inIndex", ty: "int32_t" }, Param { name: "inInt", ty: "int32_t" }],
                        ret: "nsresult",
                    },

                    /* void SetNumberStrings (in int32_t inNumStrings); */
                    Method {
                        name: "SetNumberStrings",
                        abi: "C",
                        params: &[Param { name: "inNumStrings", ty: "int32_t" }],
                        ret: "nsresult",
                    },

                    /* wstring GetString (in int32_t inIndex); */
                    Method {
                        name: "GetString",
                        abi: "C",
                        params: &[Param { name: "inIndex", ty: "int32_t" }, Param { name: "_retval", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void SetString (in int32_t inIndex, in wstring inString); */
                    Method {
                        name: "SetString",
                        abi: "C",
                        params: &[Param { name: "inIndex", ty: "int32_t" }, Param { name: "inString", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIMutableArray objects; */
                    Method {
                        name: "get_objects",
                        abi: "C",
                        params: &[Param { name: "aObjects", ty: "*mut *const nsIMutableArray" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_objects",
                        abi: "C",
                        params: &[Param { name: "aObjects", ty: "*const nsIMutableArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


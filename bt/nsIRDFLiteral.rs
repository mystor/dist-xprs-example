//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFLiteral.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFLiteral",
            base: Some("nsIRDFNode"),
            methods: Some(&[
                    /* readonly attribute wstring Value; */
                    Method {
                        name: "get_Value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void GetValueConst ([shared] out wstring aConstValue); */
                    Method {
                        name: "GetValueConst",
                        abi: "C",
                        params: &[Param { name: "aConstValue", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIRDFDate",
            base: Some("nsIRDFNode"),
            methods: Some(&[
                    /* readonly attribute PRTime Value; */
                    Method {
                        name: "get_Value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIRDFInt",
            base: Some("nsIRDFNode"),
            methods: Some(&[
                    /* readonly attribute long Value; */
                    Method {
                        name: "get_Value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIRDFBlob",
            base: Some("nsIRDFNode"),
            methods: None,
        },


        ]; D}


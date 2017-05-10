//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpctest_attributes.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXPCTestObjectReadOnly",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute string strReadOnly; */
                    Method {
                        name: "get_strReadOnly",
                        abi: "C",
                        params: &[Param { name: "aStrReadOnly", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean boolReadOnly; */
                    Method {
                        name: "get_boolReadOnly",
                        abi: "C",
                        params: &[Param { name: "aBoolReadOnly", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute short shortReadOnly; */
                    Method {
                        name: "get_shortReadOnly",
                        abi: "C",
                        params: &[Param { name: "aShortReadOnly", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long longReadOnly; */
                    Method {
                        name: "get_longReadOnly",
                        abi: "C",
                        params: &[Param { name: "aLongReadOnly", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute float floatReadOnly; */
                    Method {
                        name: "get_floatReadOnly",
                        abi: "C",
                        params: &[Param { name: "aFloatReadOnly", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute char charReadOnly; */
                    Method {
                        name: "get_charReadOnly",
                        abi: "C",
                        params: &[Param { name: "aCharReadOnly", ty: "*mut libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute PRTime timeReadOnly; */
                    Method {
                        name: "get_timeReadOnly",
                        abi: "C",
                        params: &[Param { name: "aTimeReadOnly", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIXPCTestObjectReadWrite",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute string stringProperty; */
                    Method {
                        name: "get_stringProperty",
                        abi: "C",
                        params: &[Param { name: "aStringProperty", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_stringProperty",
                        abi: "C",
                        params: &[Param { name: "aStringProperty", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean booleanProperty; */
                    Method {
                        name: "get_booleanProperty",
                        abi: "C",
                        params: &[Param { name: "aBooleanProperty", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_booleanProperty",
                        abi: "C",
                        params: &[Param { name: "aBooleanProperty", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute short shortProperty; */
                    Method {
                        name: "get_shortProperty",
                        abi: "C",
                        params: &[Param { name: "aShortProperty", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_shortProperty",
                        abi: "C",
                        params: &[Param { name: "aShortProperty", ty: "libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* attribute long longProperty; */
                    Method {
                        name: "get_longProperty",
                        abi: "C",
                        params: &[Param { name: "aLongProperty", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_longProperty",
                        abi: "C",
                        params: &[Param { name: "aLongProperty", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute float floatProperty; */
                    Method {
                        name: "get_floatProperty",
                        abi: "C",
                        params: &[Param { name: "aFloatProperty", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_floatProperty",
                        abi: "C",
                        params: &[Param { name: "aFloatProperty", ty: "libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* attribute char charProperty; */
                    Method {
                        name: "get_charProperty",
                        abi: "C",
                        params: &[Param { name: "aCharProperty", ty: "*mut libc::c_char" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_charProperty",
                        abi: "C",
                        params: &[Param { name: "aCharProperty", ty: "libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* attribute PRTime timeProperty; */
                    Method {
                        name: "get_timeProperty",
                        abi: "C",
                        params: &[Param { name: "aTimeProperty", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_timeProperty",
                        abi: "C",
                        params: &[Param { name: "aTimeProperty", ty: "PRTime" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


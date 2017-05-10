//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISupportsPrimitives.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISupportsPrimitive",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned short type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsID",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute nsIDPtr data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut *const nsID" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*const nsID" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsCString",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute ACString data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsString",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute AString data; */
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

                    /* wstring toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsPRBool",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute boolean data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsPRUint8",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute uint8_t data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut uint8_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "uint8_t" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsPRUint16",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute uint16_t data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut uint16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "uint16_t" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsPRUint32",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute uint32_t data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsPRUint64",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute uint64_t data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "uint64_t" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsPRTime",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute PRTime data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut PRTime" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "PRTime" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsChar",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute char data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut libc::c_char" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsPRInt16",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute int16_t data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut int16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "int16_t" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsPRInt32",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute int32_t data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "int32_t" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsPRInt64",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute int64_t data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut int64_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "int64_t" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsFloat",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute float data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut libc::c_float" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsDouble",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute double data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsInterfacePointer",
            base: Some("nsISupportsPrimitive"),
            methods: Some(&[
                    /* attribute nsISupports data; */
                    Method {
                        name: "get_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_data",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* attribute nsIDPtr dataIID; */
                    Method {
                        name: "get_dataIID",
                        abi: "C",
                        params: &[Param { name: "aDataIID", ty: "*mut *const nsID" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_dataIID",
                        abi: "C",
                        params: &[Param { name: "aDataIID", ty: "*const nsID" }],
                        ret: "nsresult",
                    },

                    /* string toString (); */
                    Method {
                        name: "toString",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


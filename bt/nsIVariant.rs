//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIVariant.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDataType",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "nsIVariant",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIWritableVariant",
            base: Some("nsIVariant"),
            methods: Some(&[
                    /* attribute boolean writable; */
                    Method {
                        name: "get_writable",
                        abi: "C",
                        params: &[Param { name: "aWritable", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_writable",
                        abi: "C",
                        params: &[Param { name: "aWritable", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void setAsInt8 (in uint8_t aValue); */
                    Method {
                        name: "setAsInt8",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "uint8_t" }],
                        ret: "nsresult",
                    },

                    /* void setAsInt16 (in int16_t aValue); */
                    Method {
                        name: "setAsInt16",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "int16_t" }],
                        ret: "nsresult",
                    },

                    /* void setAsInt32 (in int32_t aValue); */
                    Method {
                        name: "setAsInt32",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setAsInt64 (in int64_t aValue); */
                    Method {
                        name: "setAsInt64",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "int64_t" }],
                        ret: "nsresult",
                    },

                    /* void setAsUint8 (in uint8_t aValue); */
                    Method {
                        name: "setAsUint8",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "uint8_t" }],
                        ret: "nsresult",
                    },

                    /* void setAsUint16 (in uint16_t aValue); */
                    Method {
                        name: "setAsUint16",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "uint16_t" }],
                        ret: "nsresult",
                    },

                    /* void setAsUint32 (in uint32_t aValue); */
                    Method {
                        name: "setAsUint32",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void setAsUint64 (in uint64_t aValue); */
                    Method {
                        name: "setAsUint64",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "uint64_t" }],
                        ret: "nsresult",
                    },

                    /* void setAsFloat (in float aValue); */
                    Method {
                        name: "setAsFloat",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "libc::c_float" }],
                        ret: "nsresult",
                    },

                    /* void setAsDouble (in double aValue); */
                    Method {
                        name: "setAsDouble",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* void setAsBool (in boolean aValue); */
                    Method {
                        name: "setAsBool",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void setAsChar (in char aValue); */
                    Method {
                        name: "setAsChar",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void setAsWChar (in wchar aValue); */
                    Method {
                        name: "setAsWChar",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void setAsID (in nsIDRef aValue); */
                    Method {
                        name: "setAsID",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const nsID" }],
                        ret: "nsresult",
                    },

                    /* void setAsAString (in AString aValue); */
                    Method {
                        name: "setAsAString",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setAsDOMString (in DOMString aValue); */
                    Method {
                        name: "setAsDOMString",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setAsACString (in ACString aValue); */
                    Method {
                        name: "setAsACString",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void setAsAUTF8String (in AUTF8String aValue); */
                    Method {
                        name: "setAsAUTF8String",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void setAsString (in string aValue); */
                    Method {
                        name: "setAsString",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void setAsWString (in wstring aValue); */
                    Method {
                        name: "setAsWString",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void setAsISupports (in nsISupports aValue); */
                    Method {
                        name: "setAsISupports",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void setAsInterface (in nsIIDRef iid, [iid_is (iid)] in nsQIResult iface); */
                    Method {
                        name: "setAsInterface",
                        abi: "C",
                        params: &[Param { name: "iid", ty: "*const nsIID" }, Param { name: "iface", ty: "*const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void setAsArray (in uint16_t type, in nsIIDPtr iid, in uint32_t count, in voidPtr ptr); */
                    Method {
                        name: "setAsArray",
                        abi: "C",
                        params: &[Param { name: "type_", ty: "uint16_t" }, Param { name: "iid", ty: "*const nsIID" }, Param { name: "count", ty: "uint32_t" }, Param { name: "ptr", ty: "*const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* void setAsStringWithSize (in uint32_t size, [size_is (size)] in string str); */
                    Method {
                        name: "setAsStringWithSize",
                        abi: "C",
                        params: &[Param { name: "size", ty: "uint32_t" }, Param { name: "str", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void setAsWStringWithSize (in uint32_t size, [size_is (size)] in wstring str); */
                    Method {
                        name: "setAsWStringWithSize",
                        abi: "C",
                        params: &[Param { name: "size", ty: "uint32_t" }, Param { name: "str", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void setAsVoid (); */
                    Method {
                        name: "setAsVoid",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void setAsEmpty (); */
                    Method {
                        name: "setAsEmpty",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void setAsEmptyArray (); */
                    Method {
                        name: "setAsEmptyArray",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void setFromVariant (in nsIVariant aValue); */
                    Method {
                        name: "setFromVariant",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const nsIVariant" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


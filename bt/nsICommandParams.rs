//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICommandParams.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICommandParams",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* short getValueType (in string name); */
                    Method {
                        name: "getValueType",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* boolean getBooleanValue (in string name); */
                    Method {
                        name: "getBooleanValue",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* long getLongValue (in string name); */
                    Method {
                        name: "getLongValue",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* double getDoubleValue (in string name); */
                    Method {
                        name: "getDoubleValue",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* AString getStringValue (in string name); */
                    Method {
                        name: "getStringValue",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* string getCStringValue (in string name); */
                    Method {
                        name: "getCStringValue",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* nsISupports getISupportsValue (in string name); */
                    Method {
                        name: "getISupportsValue",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void setBooleanValue (in string name, in boolean value); */
                    Method {
                        name: "setBooleanValue",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void setLongValue (in string name, in long value); */
                    Method {
                        name: "setLongValue",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setDoubleValue (in string name, in double value); */
                    Method {
                        name: "setDoubleValue",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* void setStringValue (in string name, in AString value); */
                    Method {
                        name: "setStringValue",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setCStringValue (in string name, in string value); */
                    Method {
                        name: "setCStringValue",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void setISupportsValue (in string name, in nsISupports value); */
                    Method {
                        name: "setISupportsValue",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void removeValue (in string name); */
                    Method {
                        name: "removeValue",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


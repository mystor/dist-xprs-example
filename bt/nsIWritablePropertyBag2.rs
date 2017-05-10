//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWritablePropertyBag2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWritablePropertyBag2",
            base: Some("nsIPropertyBag2"),
            methods: Some(&[
                    /* void setPropertyAsInt32 (in AString prop, in int32_t value); */
                    Method {
                        name: "setPropertyAsInt32",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "value", ty: "int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setPropertyAsUint32 (in AString prop, in uint32_t value); */
                    Method {
                        name: "setPropertyAsUint32",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "value", ty: "uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void setPropertyAsInt64 (in AString prop, in int64_t value); */
                    Method {
                        name: "setPropertyAsInt64",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "value", ty: "int64_t" }],
                        ret: "nsresult",
                    },

                    /* void setPropertyAsUint64 (in AString prop, in uint64_t value); */
                    Method {
                        name: "setPropertyAsUint64",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "value", ty: "uint64_t" }],
                        ret: "nsresult",
                    },

                    /* void setPropertyAsDouble (in AString prop, in double value); */
                    Method {
                        name: "setPropertyAsDouble",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "value", ty: "libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* void setPropertyAsAString (in AString prop, in AString value); */
                    Method {
                        name: "setPropertyAsAString",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setPropertyAsACString (in AString prop, in ACString value); */
                    Method {
                        name: "setPropertyAsACString",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void setPropertyAsAUTF8String (in AString prop, in AUTF8String value); */
                    Method {
                        name: "setPropertyAsAUTF8String",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* void setPropertyAsBool (in AString prop, in boolean value); */
                    Method {
                        name: "setPropertyAsBool",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "value", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void setPropertyAsInterface (in AString prop, in nsISupports value); */
                    Method {
                        name: "setPropertyAsInterface",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "value", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPropertyBag2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPropertyBag2",
            base: Some("nsIPropertyBag"),
            methods: Some(&[
                    /* int32_t getPropertyAsInt32 (in AString prop); */
                    Method {
                        name: "getPropertyAsInt32",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut int32_t" }],
                        ret: "nsresult",
                    },

                    /* uint32_t getPropertyAsUint32 (in AString prop); */
                    Method {
                        name: "getPropertyAsUint32",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* int64_t getPropertyAsInt64 (in AString prop); */
                    Method {
                        name: "getPropertyAsInt64",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut int64_t" }],
                        ret: "nsresult",
                    },

                    /* uint64_t getPropertyAsUint64 (in AString prop); */
                    Method {
                        name: "getPropertyAsUint64",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut uint64_t" }],
                        ret: "nsresult",
                    },

                    /* double getPropertyAsDouble (in AString prop); */
                    Method {
                        name: "getPropertyAsDouble",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut libc::c_double" }],
                        ret: "nsresult",
                    },

                    /* AString getPropertyAsAString (in AString prop); */
                    Method {
                        name: "getPropertyAsAString",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* ACString getPropertyAsACString (in AString prop); */
                    Method {
                        name: "getPropertyAsACString",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String getPropertyAsAUTF8String (in AString prop); */
                    Method {
                        name: "getPropertyAsAUTF8String",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean getPropertyAsBool (in AString prop); */
                    Method {
                        name: "getPropertyAsBool",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void getPropertyAsInterface (in AString prop, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
                    Method {
                        name: "getPropertyAsInterface",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "iid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* nsIVariant get (in AString prop); */
                    Method {
                        name: "get",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIVariant" }],
                        ret: "nsresult",
                    },

                    /* boolean hasKey (in AString prop); */
                    Method {
                        name: "hasKey",
                        abi: "C",
                        params: &[Param { name: "prop", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


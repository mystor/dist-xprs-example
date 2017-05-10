//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFResource.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFResource",
            base: Some("nsIRDFNode"),
            methods: Some(&[
                    /* readonly attribute string Value; */
                    Method {
                        name: "get_Value",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AUTF8String ValueUTF8; */
                    Method {
                        name: "get_ValueUTF8",
                        abi: "C",
                        params: &[Param { name: "aValueUTF8", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void GetValueConst ([shared] out string aConstValue); */
                    Method {
                        name: "GetValueConst",
                        abi: "C",
                        params: &[Param { name: "aConstValue", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void Init (in string uri); */
                    Method {
                        name: "Init",
                        abi: "C",
                        params: &[Param { name: "uri", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* boolean EqualsString (in string aURI); */
                    Method {
                        name: "EqualsString",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void GetDelegate (in string aKey, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult aResult); */
                    Method {
                        name: "GetDelegate",
                        abi: "C",
                        params: &[Param { name: "aKey", ty: "*const libc::c_char" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "aResult", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* void ReleaseDelegate (in string aKey); */
                    Method {
                        name: "ReleaseDelegate",
                        abi: "C",
                        params: &[Param { name: "aKey", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


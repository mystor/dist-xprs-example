//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAuthModule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAuthModule",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in string aServiceName, in unsigned long aServiceFlags, in wstring aDomain, in wstring aUsername, in wstring aPassword); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aServiceName", ty: "*const libc::c_char" }, Param { name: "aServiceFlags", ty: "libc::uint32_t" }, Param { name: "aDomain", ty: "*const libc::int16_t" }, Param { name: "aUsername", ty: "*const libc::int16_t" }, Param { name: "aPassword", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void getNextToken ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
                    Method {
                        name: "getNextToken",
                        abi: "C",
                        params: &[Param { name: "aInToken", ty: "*const libc::c_void" }, Param { name: "aInTokenLength", ty: "libc::uint32_t" }, Param { name: "aOutToken", ty: "*mut *const libc::c_void" }, Param { name: "aOutTokenLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void wrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, in boolean confidential, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
                    Method {
                        name: "wrap",
                        abi: "C",
                        params: &[Param { name: "aInToken", ty: "*const libc::c_void" }, Param { name: "aInTokenLength", ty: "libc::uint32_t" }, Param { name: "confidential", ty: "bool" }, Param { name: "aOutToken", ty: "*mut *const libc::c_void" }, Param { name: "aOutTokenLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void unwrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
                    Method {
                        name: "unwrap",
                        abi: "C",
                        params: &[Param { name: "aInToken", ty: "*const libc::c_void" }, Param { name: "aInTokenLength", ty: "libc::uint32_t" }, Param { name: "aOutToken", ty: "*mut *const libc::c_void" }, Param { name: "aOutTokenLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


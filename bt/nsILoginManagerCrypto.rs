//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoginManagerCrypto.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILoginManagerCrypto",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* AString encrypt (in AString plainText); */
                    Method {
                        name: "encrypt",
                        abi: "C",
                        params: &[Param { name: "plainText", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString decrypt (in AString cipherText); */
                    Method {
                        name: "decrypt",
                        abi: "C",
                        params: &[Param { name: "cipherText", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean uiBusy; */
                    Method {
                        name: "get_uiBusy",
                        abi: "C",
                        params: &[Param { name: "aUiBusy", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isLoggedIn; */
                    Method {
                        name: "get_isLoggedIn",
                        abi: "C",
                        params: &[Param { name: "aIsLoggedIn", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long defaultEncType; */
                    Method {
                        name: "get_defaultEncType",
                        abi: "C",
                        params: &[Param { name: "aDefaultEncType", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


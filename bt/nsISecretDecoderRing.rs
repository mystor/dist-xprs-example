//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecretDecoderRing.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISecretDecoderRing",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* ACString encryptString (in ACString text); */
                    Method {
                        name: "encryptString",
                        abi: "C",
                        params: &[Param { name: "text", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* ACString decryptString (in ACString encryptedBase64Text); */
                    Method {
                        name: "decryptString",
                        abi: "C",
                        params: &[Param { name: "encryptedBase64Text", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* void changePassword (); */
                    Method {
                        name: "changePassword",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void logout (); */
                    Method {
                        name: "logout",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void logoutAndTeardown (); */
                    Method {
                        name: "logoutAndTeardown",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


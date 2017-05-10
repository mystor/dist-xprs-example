//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDataSignatureVerifier.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDataSignatureVerifier",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean verifyData (in ACString aData, in ACString aSignature, in ACString aPublicKey); */
                    Method {
                        name: "verifyData",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*const nsACString" }, Param { name: "aSignature", ty: "*const nsACString" }, Param { name: "aPublicKey", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIX509Cert verifySignature (in ACString signature, in ACString plaintext, out long errorCode); */
                    Method {
                        name: "verifySignature",
                        abi: "C",
                        params: &[Param { name: "signature", ty: "*const nsACString" }, Param { name: "plaintext", ty: "*const nsACString" }, Param { name: "errorCode", ty: "*mut libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIX509Cert" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


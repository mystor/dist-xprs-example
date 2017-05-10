//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptableBase64Encoder.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptableBase64Encoder",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* ACString encodeToCString (in nsIInputStream stream, in unsigned long length); */
                    Method {
                        name: "encodeToCString",
                        abi: "C",
                        params: &[Param { name: "stream", ty: "*const nsIInputStream" }, Param { name: "length", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* AString encodeToString (in nsIInputStream stream, in unsigned long length); */
                    Method {
                        name: "encodeToString",
                        abi: "C",
                        params: &[Param { name: "stream", ty: "*const nsIInputStream" }, Param { name: "length", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


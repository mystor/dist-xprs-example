//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIConverterOutputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIConverterOutputStream",
            base: Some("nsIUnicharOutputStream"),
            methods: Some(&[
                    /* void init (in nsIOutputStream aOutStream, in string aCharset, in unsigned long aBufferSize, in char16_t aReplacementCharacter); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aOutStream", ty: "*const nsIOutputStream" }, Param { name: "aCharset", ty: "*const libc::c_char" }, Param { name: "aBufferSize", ty: "libc::uint32_t" }, Param { name: "aReplacementCharacter", ty: "char16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


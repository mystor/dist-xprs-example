//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIConverterInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIConverterInputStream",
            base: Some("nsIUnicharInputStream"),
            methods: Some(&[
                    /* void init (in nsIInputStream aStream, in string aCharset, in long aBufferSize, in char16_t aReplacementChar); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aCharset", ty: "*const libc::c_char" }, Param { name: "aBufferSize", ty: "libc::int32_t" }, Param { name: "aReplacementChar", ty: "char16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


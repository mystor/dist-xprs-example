//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISaveAsCharset.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISaveAsCharset",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute AUTF8String charset; */
                    Method {
                        name: "get_charset",
                        abi: "C",
                        params: &[Param { name: "aCharset", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* void Init (in AUTF8String charset, in unsigned long ignored, in unsigned long alsoIgnored); */
                    Method {
                        name: "Init",
                        abi: "C",
                        params: &[Param { name: "charset", ty: "*const nsACString" }, Param { name: "ignored", ty: "libc::uint32_t" }, Param { name: "alsoIgnored", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* ACString Convert (in AString inString); */
                    Method {
                        name: "Convert",
                        abi: "C",
                        params: &[Param { name: "inString", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


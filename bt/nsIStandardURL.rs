//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStandardURL.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStandardURL",
            base: Some("nsIMutable"),
            methods: Some(&[
                    /* void init (in unsigned long aUrlType, in long aDefaultPort, in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aUrlType", ty: "libc::uint32_t" }, Param { name: "aDefaultPort", ty: "libc::int32_t" }, Param { name: "aSpec", ty: "*const nsACString" }, Param { name: "aOriginCharset", ty: "*const libc::c_char" }, Param { name: "aBaseURI", ty: "*const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* void setDefaultPort (in long aNewDefaultPort); */
                    Method {
                        name: "setDefaultPort",
                        abi: "C",
                        params: &[Param { name: "aNewDefaultPort", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIDNService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIDNService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* ACString convertUTF8toACE (in AUTF8String input); */
                    Method {
                        name: "convertUTF8toACE",
                        abi: "C",
                        params: &[Param { name: "input", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String convertACEtoUTF8 (in ACString input); */
                    Method {
                        name: "convertACEtoUTF8",
                        abi: "C",
                        params: &[Param { name: "input", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* boolean isACE (in ACString input); */
                    Method {
                        name: "isACE",
                        abi: "C",
                        params: &[Param { name: "input", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String normalize (in AUTF8String input); */
                    Method {
                        name: "normalize",
                        abi: "C",
                        params: &[Param { name: "input", ty: "*const nsACString" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* AUTF8String convertToDisplayIDN (in AUTF8String input, out boolean isASCII); */
                    Method {
                        name: "convertToDisplayIDN",
                        abi: "C",
                        params: &[Param { name: "input", ty: "*const nsACString" }, Param { name: "isASCII", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


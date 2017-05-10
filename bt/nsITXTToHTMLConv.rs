//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITXTToHTMLConv.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITXTToHTMLConv",
            base: Some("nsIStreamConverter"),
            methods: Some(&[
                    /* void setTitle (in wstring text); */
                    Method {
                        name: "setTitle",
                        abi: "C",
                        params: &[Param { name: "text", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void preFormatHTML (in boolean value); */
                    Method {
                        name: "preFormatHTML",
                        abi: "C",
                        params: &[Param { name: "value", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


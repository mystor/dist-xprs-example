//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITextScroll.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITextScroll",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void scrollByLines (in long numLines); */
                    Method {
                        name: "scrollByLines",
                        abi: "C",
                        params: &[Param { name: "numLines", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void scrollByPages (in long numPages); */
                    Method {
                        name: "scrollByPages",
                        abi: "C",
                        params: &[Param { name: "numPages", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


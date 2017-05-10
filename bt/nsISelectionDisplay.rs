//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISelectionDisplay.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISelectionDisplay",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setSelectionFlags (in short toggle); */
                    Method {
                        name: "setSelectionFlags",
                        abi: "C",
                        params: &[Param { name: "toggle", ty: "libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* short getSelectionFlags (); */
                    Method {
                        name: "getSelectionFlags",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


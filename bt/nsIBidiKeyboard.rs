//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBidiKeyboard.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBidiKeyboard",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void reset (); */
                    Method {
                        name: "reset",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean isLangRTL (); */
                    Method {
                        name: "isLangRTL",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean haveBidiKeyboards; */
                    Method {
                        name: "get_haveBidiKeyboards",
                        abi: "C",
                        params: &[Param { name: "aHaveBidiKeyboards", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


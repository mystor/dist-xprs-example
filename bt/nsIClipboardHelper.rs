//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClipboardHelper.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClipboardHelper",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void copyStringToClipboard (in AString aString, in long aClipboardID); */
                    Method {
                        name: "copyStringToClipboard",
                        abi: "C",
                        params: &[Param { name: "aString", ty: "*const nsAString" }, Param { name: "aClipboardID", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void copyString (in AString aString); */
                    Method {
                        name: "copyString",
                        abi: "C",
                        params: &[Param { name: "aString", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITooltipTextProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITooltipTextProvider",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean getNodeText (in nsIDOMNode aNode, out wstring aText, out wstring aDirection); */
                    Method {
                        name: "getNodeText",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsIDOMNode" }, Param { name: "aText", ty: "*mut *const libc::int16_t" }, Param { name: "aDirection", ty: "*mut *const libc::int16_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


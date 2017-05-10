//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITooltipListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITooltipListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onShowTooltip (in long aXCoords, in long aYCoords, in wstring aTipText, in wstring aTipDir); */
                    Method {
                        name: "onShowTooltip",
                        abi: "C",
                        params: &[Param { name: "aXCoords", ty: "libc::int32_t" }, Param { name: "aYCoords", ty: "libc::int32_t" }, Param { name: "aTipText", ty: "*const libc::int16_t" }, Param { name: "aTipDir", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void onHideTooltip (); */
                    Method {
                        name: "onHideTooltip",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


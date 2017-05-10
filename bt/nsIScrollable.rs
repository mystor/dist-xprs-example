//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScrollable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScrollable",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* long getDefaultScrollbarPreferences (in long scrollOrientation); */
                    Method {
                        name: "getDefaultScrollbarPreferences",
                        abi: "C",
                        params: &[Param { name: "scrollOrientation", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void setDefaultScrollbarPreferences (in long scrollOrientation, in long scrollbarPref); */
                    Method {
                        name: "setDefaultScrollbarPreferences",
                        abi: "C",
                        params: &[Param { name: "scrollOrientation", ty: "libc::int32_t" }, Param { name: "scrollbarPref", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void getScrollbarVisibility (out boolean verticalVisible, out boolean horizontalVisible); */
                    Method {
                        name: "getScrollbarVisibility",
                        abi: "C",
                        params: &[Param { name: "verticalVisible", ty: "*mut bool" }, Param { name: "horizontalVisible", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


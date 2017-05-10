//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoCompletePopup.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAutoCompletePopup",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIAutoCompleteInput input; */
                    Method {
                        name: "get_input",
                        abi: "C",
                        params: &[Param { name: "aInput", ty: "*mut *const nsIAutoCompleteInput" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString overrideValue; */
                    Method {
                        name: "get_overrideValue",
                        abi: "C",
                        params: &[Param { name: "aOverrideValue", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute long selectedIndex; */
                    Method {
                        name: "get_selectedIndex",
                        abi: "C",
                        params: &[Param { name: "aSelectedIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_selectedIndex",
                        abi: "C",
                        params: &[Param { name: "aSelectedIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean popupOpen; */
                    Method {
                        name: "get_popupOpen",
                        abi: "C",
                        params: &[Param { name: "aPopupOpen", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void openAutocompletePopup (in nsIAutoCompleteInput input, in nsIDOMElement element); */
                    Method {
                        name: "openAutocompletePopup",
                        abi: "C",
                        params: &[Param { name: "input", ty: "*const nsIAutoCompleteInput" }, Param { name: "element", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void closePopup (); */
                    Method {
                        name: "closePopup",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void invalidate (in unsigned short reason); */
                    Method {
                        name: "invalidate",
                        abi: "C",
                        params: &[Param { name: "reason", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* void selectBy (in boolean reverse, in boolean page); */
                    Method {
                        name: "selectBy",
                        abi: "C",
                        params: &[Param { name: "reverse", ty: "bool" }, Param { name: "page", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


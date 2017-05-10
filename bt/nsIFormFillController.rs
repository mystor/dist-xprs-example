//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFormFillController.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFormFillController",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMHTMLInputElement focusedInput; */
                    Method {
                        name: "get_focusedInput",
                        abi: "C",
                        params: &[Param { name: "aFocusedInput", ty: "*mut *const nsIDOMHTMLInputElement" }],
                        ret: "nsresult",
                    },

                    /* void attachToBrowser (in nsIDocShell docShell, in nsIAutoCompletePopup popup); */
                    Method {
                        name: "attachToBrowser",
                        abi: "C",
                        params: &[Param { name: "docShell", ty: "*const nsIDocShell" }, Param { name: "popup", ty: "*const nsIAutoCompletePopup" }],
                        ret: "nsresult",
                    },

                    /* void detachFromBrowser (in nsIDocShell docShell); */
                    Method {
                        name: "detachFromBrowser",
                        abi: "C",
                        params: &[Param { name: "docShell", ty: "*const nsIDocShell" }],
                        ret: "nsresult",
                    },

                    /* void markAsLoginManagerField (in nsIDOMHTMLInputElement aInput); */
                    Method {
                        name: "markAsLoginManagerField",
                        abi: "C",
                        params: &[Param { name: "aInput", ty: "*const nsIDOMHTMLInputElement" }],
                        ret: "nsresult",
                    },

                    /* void markAsAutofillField (in nsIDOMHTMLInputElement aInput); */
                    Method {
                        name: "markAsAutofillField",
                        abi: "C",
                        params: &[Param { name: "aInput", ty: "*const nsIDOMHTMLInputElement" }],
                        ret: "nsresult",
                    },

                    /* void showPopup (); */
                    Method {
                        name: "showPopup",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


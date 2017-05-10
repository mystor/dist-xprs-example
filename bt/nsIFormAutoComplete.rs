//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFormAutoComplete.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFormAutoComplete",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void autoCompleteSearchAsync (in AString aInputName, in AString aSearchString, in nsIDOMHTMLInputElement aField, in nsIAutoCompleteResult aPreviousResult, in nsIAutoCompleteResult aDatalistResult, in nsIFormAutoCompleteObserver aListener); */
                    Method {
                        name: "autoCompleteSearchAsync",
                        abi: "C",
                        params: &[Param { name: "aInputName", ty: "*const nsAString" }, Param { name: "aSearchString", ty: "*const nsAString" }, Param { name: "aField", ty: "*const nsIDOMHTMLInputElement" }, Param { name: "aPreviousResult", ty: "*const nsIAutoCompleteResult" }, Param { name: "aDatalistResult", ty: "*const nsIAutoCompleteResult" }, Param { name: "aListener", ty: "*const nsIFormAutoCompleteObserver" }],
                        ret: "nsresult",
                    },

                    /* void stopAutoCompleteSearch (); */
                    Method {
                        name: "stopAutoCompleteSearch",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void stopControllingInput (in nsIDOMHTMLInputElement aField); */
                    Method {
                        name: "stopControllingInput",
                        abi: "C",
                        params: &[Param { name: "aField", ty: "*const nsIDOMHTMLInputElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIFormAutoCompleteObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onSearchCompletion (in nsIAutoCompleteResult result); */
                    Method {
                        name: "onSearchCompletion",
                        abi: "C",
                        params: &[Param { name: "result", ty: "*const nsIAutoCompleteResult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


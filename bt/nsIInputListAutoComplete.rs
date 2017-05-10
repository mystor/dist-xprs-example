//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInputListAutoComplete.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInputListAutoComplete",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIAutoCompleteResult autoCompleteSearch (in AString aSearchString, in nsIDOMHTMLInputElement aField); */
                    Method {
                        name: "autoCompleteSearch",
                        abi: "C",
                        params: &[Param { name: "aSearchString", ty: "*const nsAString" }, Param { name: "aField", ty: "*const nsIDOMHTMLInputElement" }, Param { name: "_retval", ty: "*mut *const nsIAutoCompleteResult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


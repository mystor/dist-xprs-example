//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoCompleteSearch.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAutoCompleteSearch",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void startSearch (in AString searchString, in AString searchParam, in nsIAutoCompleteResult previousResult, in nsIAutoCompleteObserver listener); */
                    Method {
                        name: "startSearch",
                        abi: "C",
                        params: &[Param { name: "searchString", ty: "*const nsAString" }, Param { name: "searchParam", ty: "*const nsAString" }, Param { name: "previousResult", ty: "*const nsIAutoCompleteResult" }, Param { name: "listener", ty: "*const nsIAutoCompleteObserver" }],
                        ret: "nsresult",
                    },

                    /* void stopSearch (); */
                    Method {
                        name: "stopSearch",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAutoCompleteObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onSearchResult (in nsIAutoCompleteSearch search, in nsIAutoCompleteResult result); */
                    Method {
                        name: "onSearchResult",
                        abi: "C",
                        params: &[Param { name: "search", ty: "*const nsIAutoCompleteSearch" }, Param { name: "result", ty: "*const nsIAutoCompleteResult" }],
                        ret: "nsresult",
                    },

                    /* void onUpdateSearchResult (in nsIAutoCompleteSearch search, in nsIAutoCompleteResult result); */
                    Method {
                        name: "onUpdateSearchResult",
                        abi: "C",
                        params: &[Param { name: "search", ty: "*const nsIAutoCompleteSearch" }, Param { name: "result", ty: "*const nsIAutoCompleteResult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIAutoCompleteSearchDescriptor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute unsigned short searchType; */
                    Method {
                        name: "get_searchType",
                        abi: "C",
                        params: &[Param { name: "aSearchType", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean clearingAutoFillSearchesAgain; */
                    Method {
                        name: "get_clearingAutoFillSearchesAgain",
                        abi: "C",
                        params: &[Param { name: "aClearingAutoFillSearchesAgain", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


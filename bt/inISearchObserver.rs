//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/inISearchObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "inISearchObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onSearchStart (in inISearchProcess aModule); */
                    Method {
                        name: "onSearchStart",
                        abi: "C",
                        params: &[Param { name: "aModule", ty: "*const inISearchProcess" }],
                        ret: "nsresult",
                    },

                    /* void onSearchResult (in inISearchProcess aModule); */
                    Method {
                        name: "onSearchResult",
                        abi: "C",
                        params: &[Param { name: "aModule", ty: "*const inISearchProcess" }],
                        ret: "nsresult",
                    },

                    /* void onSearchEnd (in inISearchProcess aModule, in short aResult); */
                    Method {
                        name: "onSearchEnd",
                        abi: "C",
                        params: &[Param { name: "aModule", ty: "*const inISearchProcess" }, Param { name: "aResult", ty: "libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void onSearchError (in inISearchProcess aModule, in AString aMessage); */
                    Method {
                        name: "onSearchError",
                        abi: "C",
                        params: &[Param { name: "aModule", ty: "*const inISearchProcess" }, Param { name: "aMessage", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


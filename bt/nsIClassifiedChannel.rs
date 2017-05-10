//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClassifiedChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClassifiedChannel",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void setMatchedInfo (in ACString aList, in ACString aProvider, in ACString aPrefix); */
                    Method {
                        name: "setMatchedInfo",
                        abi: "C",
                        params: &[Param { name: "aList", ty: "*const nsACString" }, Param { name: "aProvider", ty: "*const nsACString" }, Param { name: "aPrefix", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString matchedList; */
                    Method {
                        name: "get_matchedList",
                        abi: "C",
                        params: &[Param { name: "aMatchedList", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString matchedProvider; */
                    Method {
                        name: "get_matchedProvider",
                        abi: "C",
                        params: &[Param { name: "aMatchedProvider", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString matchedPrefix; */
                    Method {
                        name: "get_matchedPrefix",
                        abi: "C",
                        params: &[Param { name: "aMatchedPrefix", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


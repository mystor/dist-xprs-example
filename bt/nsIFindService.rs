//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFindService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFindService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute AString searchString; */
                    Method {
                        name: "get_searchString",
                        abi: "C",
                        params: &[Param { name: "aSearchString", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_searchString",
                        abi: "C",
                        params: &[Param { name: "aSearchString", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString replaceString; */
                    Method {
                        name: "get_replaceString",
                        abi: "C",
                        params: &[Param { name: "aReplaceString", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_replaceString",
                        abi: "C",
                        params: &[Param { name: "aReplaceString", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean findBackwards; */
                    Method {
                        name: "get_findBackwards",
                        abi: "C",
                        params: &[Param { name: "aFindBackwards", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_findBackwards",
                        abi: "C",
                        params: &[Param { name: "aFindBackwards", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean wrapFind; */
                    Method {
                        name: "get_wrapFind",
                        abi: "C",
                        params: &[Param { name: "aWrapFind", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_wrapFind",
                        abi: "C",
                        params: &[Param { name: "aWrapFind", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean entireWord; */
                    Method {
                        name: "get_entireWord",
                        abi: "C",
                        params: &[Param { name: "aEntireWord", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_entireWord",
                        abi: "C",
                        params: &[Param { name: "aEntireWord", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean matchCase; */
                    Method {
                        name: "get_matchCase",
                        abi: "C",
                        params: &[Param { name: "aMatchCase", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_matchCase",
                        abi: "C",
                        params: &[Param { name: "aMatchCase", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


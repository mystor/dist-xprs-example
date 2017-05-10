//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFind.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFind",
            base: Some("nsISupports"),
            methods: Some(&[
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

                    /* attribute boolean caseSensitive; */
                    Method {
                        name: "get_caseSensitive",
                        abi: "C",
                        params: &[Param { name: "aCaseSensitive", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_caseSensitive",
                        abi: "C",
                        params: &[Param { name: "aCaseSensitive", ty: "bool" }],
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

                    /* nsIDOMRange Find (in wstring aPatText, in nsIDOMRange aSearchRange, in nsIDOMRange aStartPoint, in nsIDOMRange aEndPoint); */
                    Method {
                        name: "Find",
                        abi: "C",
                        params: &[Param { name: "aPatText", ty: "*const libc::int16_t" }, Param { name: "aSearchRange", ty: "*const nsIDOMRange" }, Param { name: "aStartPoint", ty: "*const nsIDOMRange" }, Param { name: "aEndPoint", ty: "*const nsIDOMRange" }, Param { name: "_retval", ty: "*mut *const nsIDOMRange" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


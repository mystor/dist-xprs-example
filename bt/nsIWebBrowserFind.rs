//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserFind.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserFind",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean findNext (); */
                    Method {
                        name: "findNext",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* attribute wstring searchString; */
                    Method {
                        name: "get_searchString",
                        abi: "C",
                        params: &[Param { name: "aSearchString", ty: "*mut *const libc::int16_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_searchString",
                        abi: "C",
                        params: &[Param { name: "aSearchString", ty: "*const libc::int16_t" }],
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

                    /* attribute boolean searchFrames; */
                    Method {
                        name: "get_searchFrames",
                        abi: "C",
                        params: &[Param { name: "aSearchFrames", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_searchFrames",
                        abi: "C",
                        params: &[Param { name: "aSearchFrames", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIWebBrowserFindInFrames",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute mozIDOMWindowProxy currentSearchFrame; */
                    Method {
                        name: "get_currentSearchFrame",
                        abi: "C",
                        params: &[Param { name: "aCurrentSearchFrame", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_currentSearchFrame",
                        abi: "C",
                        params: &[Param { name: "aCurrentSearchFrame", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* attribute mozIDOMWindowProxy rootSearchFrame; */
                    Method {
                        name: "get_rootSearchFrame",
                        abi: "C",
                        params: &[Param { name: "aRootSearchFrame", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_rootSearchFrame",
                        abi: "C",
                        params: &[Param { name: "aRootSearchFrame", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean searchSubframes; */
                    Method {
                        name: "get_searchSubframes",
                        abi: "C",
                        params: &[Param { name: "aSearchSubframes", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_searchSubframes",
                        abi: "C",
                        params: &[Param { name: "aSearchSubframes", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean searchParentFrames; */
                    Method {
                        name: "get_searchParentFrames",
                        abi: "C",
                        params: &[Param { name: "aSearchParentFrames", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_searchParentFrames",
                        abi: "C",
                        params: &[Param { name: "aSearchParentFrames", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


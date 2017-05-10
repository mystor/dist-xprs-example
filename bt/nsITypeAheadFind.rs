//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITypeAheadFind.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITypeAheadFind",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in nsIDocShell aDocShell); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aDocShell", ty: "*const nsIDocShell" }],
                        ret: "nsresult",
                    },

                    /* unsigned short find (in AString aSearchString, in boolean aLinksOnly); */
                    Method {
                        name: "find",
                        abi: "C",
                        params: &[Param { name: "aSearchString", ty: "*const nsAString" }, Param { name: "aLinksOnly", ty: "bool" }, Param { name: "_retval", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* unsigned short findAgain (in boolean findBackwards, in boolean aLinksOnly); */
                    Method {
                        name: "findAgain",
                        abi: "C",
                        params: &[Param { name: "findBackwards", ty: "bool" }, Param { name: "aLinksOnly", ty: "bool" }, Param { name: "_retval", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* nsIDOMRange getFoundRange (); */
                    Method {
                        name: "getFoundRange",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIDOMRange" }],
                        ret: "nsresult",
                    },

                    /* void setDocShell (in nsIDocShell aDocShell); */
                    Method {
                        name: "setDocShell",
                        abi: "C",
                        params: &[Param { name: "aDocShell", ty: "*const nsIDocShell" }],
                        ret: "nsresult",
                    },

                    /* void setSelectionModeAndRepaint (in short toggle); */
                    Method {
                        name: "setSelectionModeAndRepaint",
                        abi: "C",
                        params: &[Param { name: "toggle", ty: "libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void collapseSelection (); */
                    Method {
                        name: "collapseSelection",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean isRangeVisible (in nsIDOMRange aRange, in boolean aMustBeInViewPort); */
                    Method {
                        name: "isRangeVisible",
                        abi: "C",
                        params: &[Param { name: "aRange", ty: "*const nsIDOMRange" }, Param { name: "aMustBeInViewPort", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString searchString; */
                    Method {
                        name: "get_searchString",
                        abi: "C",
                        params: &[Param { name: "aSearchString", ty: "*mut nsAString" }],
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

                    /* readonly attribute nsIDOMElement foundLink; */
                    Method {
                        name: "get_foundLink",
                        abi: "C",
                        params: &[Param { name: "aFoundLink", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMElement foundEditable; */
                    Method {
                        name: "get_foundEditable",
                        abi: "C",
                        params: &[Param { name: "aFoundEditable", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute mozIDOMWindow currentWindow; */
                    Method {
                        name: "get_currentWindow",
                        abi: "C",
                        params: &[Param { name: "aCurrentWindow", ty: "*mut *const mozIDOMWindow" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoCompleteController.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAutoCompleteController",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIAutoCompleteInput input; */
                    Method {
                        name: "get_input",
                        abi: "C",
                        params: &[Param { name: "aInput", ty: "*mut *const nsIAutoCompleteInput" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_input",
                        abi: "C",
                        params: &[Param { name: "aInput", ty: "*const nsIAutoCompleteInput" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned short searchStatus; */
                    Method {
                        name: "get_searchStatus",
                        abi: "C",
                        params: &[Param { name: "aSearchStatus", ty: "*mut libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long matchCount; */
                    Method {
                        name: "get_matchCount",
                        abi: "C",
                        params: &[Param { name: "aMatchCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void startSearch (in AString searchString); */
                    Method {
                        name: "startSearch",
                        abi: "C",
                        params: &[Param { name: "searchString", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void stopSearch (); */
                    Method {
                        name: "stopSearch",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean handleText (); */
                    Method {
                        name: "handleText",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean handleEnter (in boolean aIsPopupSelection, [optional] in nsIDOMEvent aEvent); */
                    Method {
                        name: "handleEnter",
                        abi: "C",
                        params: &[Param { name: "aIsPopupSelection", ty: "bool" }, Param { name: "aEvent", ty: "*const nsIDOMEvent" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean handleEscape (); */
                    Method {
                        name: "handleEscape",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void handleStartComposition (); */
                    Method {
                        name: "handleStartComposition",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void handleEndComposition (); */
                    Method {
                        name: "handleEndComposition",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void handleTab (); */
                    Method {
                        name: "handleTab",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean handleKeyNavigation (in unsigned long key); */
                    Method {
                        name: "handleKeyNavigation",
                        abi: "C",
                        params: &[Param { name: "key", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean handleDelete (); */
                    Method {
                        name: "handleDelete",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* AString getValueAt (in long index); */
                    Method {
                        name: "getValueAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getLabelAt (in long index); */
                    Method {
                        name: "getLabelAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getCommentAt (in long index); */
                    Method {
                        name: "getCommentAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getStyleAt (in long index); */
                    Method {
                        name: "getStyleAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getImageAt (in long index); */
                    Method {
                        name: "getImageAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* AString getFinalCompleteValueAt (in long index); */
                    Method {
                        name: "getFinalCompleteValueAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

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

                    /* void setInitiallySelectedIndex (in long index); */
                    Method {
                        name: "setInitiallySelectedIndex",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


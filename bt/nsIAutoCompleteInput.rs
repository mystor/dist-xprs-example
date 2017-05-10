//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoCompleteInput.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAutoCompleteInput",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIAutoCompletePopup popup; */
                    Method {
                        name: "get_popup",
                        abi: "C",
                        params: &[Param { name: "aPopup", ty: "*mut *const nsIAutoCompletePopup" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIAutoCompleteController controller; */
                    Method {
                        name: "get_controller",
                        abi: "C",
                        params: &[Param { name: "aController", ty: "*mut *const nsIAutoCompleteController" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean popupOpen; */
                    Method {
                        name: "get_popupOpen",
                        abi: "C",
                        params: &[Param { name: "aPopupOpen", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_popupOpen",
                        abi: "C",
                        params: &[Param { name: "aPopupOpen", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean disableAutoComplete; */
                    Method {
                        name: "get_disableAutoComplete",
                        abi: "C",
                        params: &[Param { name: "aDisableAutoComplete", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_disableAutoComplete",
                        abi: "C",
                        params: &[Param { name: "aDisableAutoComplete", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean completeDefaultIndex; */
                    Method {
                        name: "get_completeDefaultIndex",
                        abi: "C",
                        params: &[Param { name: "aCompleteDefaultIndex", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_completeDefaultIndex",
                        abi: "C",
                        params: &[Param { name: "aCompleteDefaultIndex", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean completeSelectedIndex; */
                    Method {
                        name: "get_completeSelectedIndex",
                        abi: "C",
                        params: &[Param { name: "aCompleteSelectedIndex", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_completeSelectedIndex",
                        abi: "C",
                        params: &[Param { name: "aCompleteSelectedIndex", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean forceComplete; */
                    Method {
                        name: "get_forceComplete",
                        abi: "C",
                        params: &[Param { name: "aForceComplete", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_forceComplete",
                        abi: "C",
                        params: &[Param { name: "aForceComplete", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long minResultsForPopup; */
                    Method {
                        name: "get_minResultsForPopup",
                        abi: "C",
                        params: &[Param { name: "aMinResultsForPopup", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_minResultsForPopup",
                        abi: "C",
                        params: &[Param { name: "aMinResultsForPopup", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long maxRows; */
                    Method {
                        name: "get_maxRows",
                        abi: "C",
                        params: &[Param { name: "aMaxRows", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_maxRows",
                        abi: "C",
                        params: &[Param { name: "aMaxRows", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean showCommentColumn; */
                    Method {
                        name: "get_showCommentColumn",
                        abi: "C",
                        params: &[Param { name: "aShowCommentColumn", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_showCommentColumn",
                        abi: "C",
                        params: &[Param { name: "aShowCommentColumn", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean showImageColumn; */
                    Method {
                        name: "get_showImageColumn",
                        abi: "C",
                        params: &[Param { name: "aShowImageColumn", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_showImageColumn",
                        abi: "C",
                        params: &[Param { name: "aShowImageColumn", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute unsigned long timeout; */
                    Method {
                        name: "get_timeout",
                        abi: "C",
                        params: &[Param { name: "aTimeout", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_timeout",
                        abi: "C",
                        params: &[Param { name: "aTimeout", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute AString searchParam; */
                    Method {
                        name: "get_searchParam",
                        abi: "C",
                        params: &[Param { name: "aSearchParam", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_searchParam",
                        abi: "C",
                        params: &[Param { name: "aSearchParam", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long searchCount; */
                    Method {
                        name: "get_searchCount",
                        abi: "C",
                        params: &[Param { name: "aSearchCount", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* ACString getSearchAt (in unsigned long index); */
                    Method {
                        name: "getSearchAt",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* attribute AString textValue; */
                    Method {
                        name: "get_textValue",
                        abi: "C",
                        params: &[Param { name: "aTextValue", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_textValue",
                        abi: "C",
                        params: &[Param { name: "aTextValue", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void setTextValueWithReason (in AString aValue, in unsigned short aReason); */
                    Method {
                        name: "setTextValueWithReason",
                        abi: "C",
                        params: &[Param { name: "aValue", ty: "*const nsAString" }, Param { name: "aReason", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long selectionStart; */
                    Method {
                        name: "get_selectionStart",
                        abi: "C",
                        params: &[Param { name: "aSelectionStart", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long selectionEnd; */
                    Method {
                        name: "get_selectionEnd",
                        abi: "C",
                        params: &[Param { name: "aSelectionEnd", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void selectTextRange (in long startIndex, in long endIndex); */
                    Method {
                        name: "selectTextRange",
                        abi: "C",
                        params: &[Param { name: "startIndex", ty: "libc::int32_t" }, Param { name: "endIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void onSearchBegin (); */
                    Method {
                        name: "onSearchBegin",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onSearchComplete (); */
                    Method {
                        name: "onSearchComplete",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* boolean onTextEntered ([optional] in nsIDOMEvent aEvent); */
                    Method {
                        name: "onTextEntered",
                        abi: "C",
                        params: &[Param { name: "aEvent", ty: "*const nsIDOMEvent" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean onTextReverted (); */
                    Method {
                        name: "onTextReverted",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean consumeRollupEvent; */
                    Method {
                        name: "get_consumeRollupEvent",
                        abi: "C",
                        params: &[Param { name: "aConsumeRollupEvent", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean inPrivateContext; */
                    Method {
                        name: "get_inPrivateContext",
                        abi: "C",
                        params: &[Param { name: "aInPrivateContext", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean noRollupOnCaretMove; */
                    Method {
                        name: "get_noRollupOnCaretMove",
                        abi: "C",
                        params: &[Param { name: "aNoRollupOnCaretMove", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long userContextId; */
                    Method {
                        name: "get_userContextId",
                        abi: "C",
                        params: &[Param { name: "aUserContextId", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


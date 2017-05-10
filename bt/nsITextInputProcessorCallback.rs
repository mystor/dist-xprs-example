//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITextInputProcessorCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITextInputProcessorNotification",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute ACString type; */
                    Method {
                        name: "get_type_",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long offset; */
                    Method {
                        name: "get_offset",
                        abi: "C",
                        params: &[Param { name: "aOffset", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString text; */
                    Method {
                        name: "get_text",
                        abi: "C",
                        params: &[Param { name: "aText", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean collapsed; */
                    Method {
                        name: "get_collapsed",
                        abi: "C",
                        params: &[Param { name: "aCollapsed", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute uint32_t length; */
                    Method {
                        name: "get_length",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "*mut uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean reversed; */
                    Method {
                        name: "get_reversed",
                        abi: "C",
                        params: &[Param { name: "aReversed", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute ACString writingMode; */
                    Method {
                        name: "get_writingMode",
                        abi: "C",
                        params: &[Param { name: "aWritingMode", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean causedByComposition; */
                    Method {
                        name: "get_causedByComposition",
                        abi: "C",
                        params: &[Param { name: "aCausedByComposition", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean causedBySelectionEvent; */
                    Method {
                        name: "get_causedBySelectionEvent",
                        abi: "C",
                        params: &[Param { name: "aCausedBySelectionEvent", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean occurredDuringComposition; */
                    Method {
                        name: "get_occurredDuringComposition",
                        abi: "C",
                        params: &[Param { name: "aOccurredDuringComposition", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long removedLength; */
                    Method {
                        name: "get_removedLength",
                        abi: "C",
                        params: &[Param { name: "aRemovedLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute unsigned long addedLength; */
                    Method {
                        name: "get_addedLength",
                        abi: "C",
                        params: &[Param { name: "aAddedLength", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean causedOnlyByComposition; */
                    Method {
                        name: "get_causedOnlyByComposition",
                        abi: "C",
                        params: &[Param { name: "aCausedOnlyByComposition", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean includingChangesDuringComposition; */
                    Method {
                        name: "get_includingChangesDuringComposition",
                        abi: "C",
                        params: &[Param { name: "aIncludingChangesDuringComposition", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean includingChangesWithoutComposition; */
                    Method {
                        name: "get_includingChangesWithoutComposition",
                        abi: "C",
                        params: &[Param { name: "aIncludingChangesWithoutComposition", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsITextInputProcessorCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean onNotify (in nsITextInputProcessor aTextInputProcessor, in nsITextInputProcessorNotification aNotification); */
                    Method {
                        name: "onNotify",
                        abi: "C",
                        params: &[Param { name: "aTextInputProcessor", ty: "*const nsITextInputProcessor" }, Param { name: "aNotification", ty: "*const nsITextInputProcessorNotification" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


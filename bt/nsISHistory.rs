//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISHistory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISHistory",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute bool isPartial; */
                    Method {
                        name: "get_isPartial",
                        abi: "C",
                        params: &[Param { name: "aIsPartial", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long count; */
                    Method {
                        name: "get_count",
                        abi: "C",
                        params: &[Param { name: "aCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long globalCount; */
                    Method {
                        name: "get_globalCount",
                        abi: "C",
                        params: &[Param { name: "aGlobalCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long globalIndexOffset; */
                    Method {
                        name: "get_globalIndexOffset",
                        abi: "C",
                        params: &[Param { name: "aGlobalIndexOffset", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long index; */
                    Method {
                        name: "get_index",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long globalIndex; */
                    Method {
                        name: "get_globalIndex",
                        abi: "C",
                        params: &[Param { name: "aGlobalIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long requestedIndex; */
                    Method {
                        name: "get_requestedIndex",
                        abi: "C",
                        params: &[Param { name: "aRequestedIndex", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute long maxLength; */
                    Method {
                        name: "get_maxLength",
                        abi: "C",
                        params: &[Param { name: "aMaxLength", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_maxLength",
                        abi: "C",
                        params: &[Param { name: "aMaxLength", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* nsISHEntry getEntryAtIndex (in long index, in boolean modifyIndex); */
                    Method {
                        name: "getEntryAtIndex",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "modifyIndex", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsISHEntry" }],
                        ret: "nsresult",
                    },

                    /* void restoreToEntryAtIndex (in long index); */
                    Method {
                        name: "restoreToEntryAtIndex",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void PurgeHistory (in long numEntries); */
                    Method {
                        name: "PurgeHistory",
                        abi: "C",
                        params: &[Param { name: "numEntries", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void addSHistoryListener (in nsISHistoryListener aListener); */
                    Method {
                        name: "addSHistoryListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsISHistoryListener" }],
                        ret: "nsresult",
                    },

                    /* void removeSHistoryListener (in nsISHistoryListener aListener); */
                    Method {
                        name: "removeSHistoryListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsISHistoryListener" }],
                        ret: "nsresult",
                    },

                    /* void setPartialSHistoryListener (in nsIPartialSHistoryListener aListener); */
                    Method {
                        name: "setPartialSHistoryListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIPartialSHistoryListener" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsISimpleEnumerator SHistoryEnumerator; */
                    Method {
                        name: "get_SHistoryEnumerator",
                        abi: "C",
                        params: &[Param { name: "aSHistoryEnumerator", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* void reloadCurrentEntry (); */
                    Method {
                        name: "reloadCurrentEntry",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* long getIndexOfEntry (in nsISHEntry aEntry); */
                    Method {
                        name: "getIndexOfEntry",
                        abi: "C",
                        params: &[Param { name: "aEntry", ty: "*const nsISHEntry" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void onPartialSHistoryActive (in long globalLength, in long targetIndex); */
                    Method {
                        name: "onPartialSHistoryActive",
                        abi: "C",
                        params: &[Param { name: "globalLength", ty: "libc::int32_t" }, Param { name: "targetIndex", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void onPartialSHistoryDeactive (); */
                    Method {
                        name: "onPartialSHistoryDeactive",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onAttachGroupedSHistory (in long offset); */
                    Method {
                        name: "onAttachGroupedSHistory",
                        abi: "C",
                        params: &[Param { name: "offset", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransactionManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITransactionManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void doTransaction (in nsITransaction aTransaction); */
                    Method {
                        name: "doTransaction",
                        abi: "C",
                        params: &[Param { name: "aTransaction", ty: "*const nsITransaction" }],
                        ret: "nsresult",
                    },

                    /* void undoTransaction (); */
                    Method {
                        name: "undoTransaction",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void redoTransaction (); */
                    Method {
                        name: "redoTransaction",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void clear (); */
                    Method {
                        name: "clear",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void clearUndoStack (); */
                    Method {
                        name: "clearUndoStack",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void clearRedoStack (); */
                    Method {
                        name: "clearRedoStack",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void beginBatch (in nsISupports aData); */
                    Method {
                        name: "beginBatch",
                        abi: "C",
                        params: &[Param { name: "aData", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void endBatch (in boolean aAllowEmpty); */
                    Method {
                        name: "endBatch",
                        abi: "C",
                        params: &[Param { name: "aAllowEmpty", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long numberOfUndoItems; */
                    Method {
                        name: "get_numberOfUndoItems",
                        abi: "C",
                        params: &[Param { name: "aNumberOfUndoItems", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute long numberOfRedoItems; */
                    Method {
                        name: "get_numberOfRedoItems",
                        abi: "C",
                        params: &[Param { name: "aNumberOfRedoItems", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* attribute long maxTransactionCount; */
                    Method {
                        name: "get_maxTransactionCount",
                        abi: "C",
                        params: &[Param { name: "aMaxTransactionCount", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_maxTransactionCount",
                        abi: "C",
                        params: &[Param { name: "aMaxTransactionCount", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void batchTopUndo (); */
                    Method {
                        name: "batchTopUndo",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void removeTopUndo (); */
                    Method {
                        name: "removeTopUndo",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* nsITransaction peekUndoStack (); */
                    Method {
                        name: "peekUndoStack",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsITransaction" }],
                        ret: "nsresult",
                    },

                    /* nsITransaction peekRedoStack (); */
                    Method {
                        name: "peekRedoStack",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsITransaction" }],
                        ret: "nsresult",
                    },

                    /* nsITransactionList getUndoList (); */
                    Method {
                        name: "getUndoList",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsITransactionList" }],
                        ret: "nsresult",
                    },

                    /* nsITransactionList getRedoList (); */
                    Method {
                        name: "getRedoList",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsITransactionList" }],
                        ret: "nsresult",
                    },

                    /* void AddListener (in nsITransactionListener aListener); */
                    Method {
                        name: "AddListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsITransactionListener" }],
                        ret: "nsresult",
                    },

                    /* void RemoveListener (in nsITransactionListener aListener); */
                    Method {
                        name: "RemoveListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsITransactionListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


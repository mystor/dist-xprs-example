//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransactionListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITransactionListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean willDo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
                    Method {
                        name: "willDo",
                        abi: "C",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTransaction", ty: "*const nsITransaction" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void didDo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aDoResult); */
                    Method {
                        name: "didDo",
                        abi: "C",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTransaction", ty: "*const nsITransaction" }, Param { name: "aDoResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* boolean willUndo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
                    Method {
                        name: "willUndo",
                        abi: "C",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTransaction", ty: "*const nsITransaction" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void didUndo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aUndoResult); */
                    Method {
                        name: "didUndo",
                        abi: "C",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTransaction", ty: "*const nsITransaction" }, Param { name: "aUndoResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* boolean willRedo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
                    Method {
                        name: "willRedo",
                        abi: "C",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTransaction", ty: "*const nsITransaction" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void didRedo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aRedoResult); */
                    Method {
                        name: "didRedo",
                        abi: "C",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTransaction", ty: "*const nsITransaction" }, Param { name: "aRedoResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* boolean willBeginBatch (in nsITransactionManager aManager); */
                    Method {
                        name: "willBeginBatch",
                        abi: "C",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void didBeginBatch (in nsITransactionManager aManager, in nsresult aResult); */
                    Method {
                        name: "didBeginBatch",
                        abi: "C",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* boolean willEndBatch (in nsITransactionManager aManager); */
                    Method {
                        name: "willEndBatch",
                        abi: "C",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void didEndBatch (in nsITransactionManager aManager, in nsresult aResult); */
                    Method {
                        name: "didEndBatch",
                        abi: "C",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* boolean willMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge); */
                    Method {
                        name: "willMerge",
                        abi: "C",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTopTransaction", ty: "*const nsITransaction" }, Param { name: "aTransactionToMerge", ty: "*const nsITransaction" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void didMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge, in boolean aDidMerge, in nsresult aMergeResult); */
                    Method {
                        name: "didMerge",
                        abi: "C",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTopTransaction", ty: "*const nsITransaction" }, Param { name: "aTransactionToMerge", ty: "*const nsITransaction" }, Param { name: "aDidMerge", ty: "bool" }, Param { name: "aMergeResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


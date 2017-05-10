//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULTemplateBuilder.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXULTemplateBuilder",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMElement root; */
                    Method {
                        name: "get_root",
                        abi: "C",
                        params: &[Param { name: "aRoot", ty: "*mut *const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* attribute nsISupports datasource; */
                    Method {
                        name: "get_datasource",
                        abi: "C",
                        params: &[Param { name: "aDatasource", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_datasource",
                        abi: "C",
                        params: &[Param { name: "aDatasource", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIRDFCompositeDataSource database; */
                    Method {
                        name: "get_database",
                        abi: "C",
                        params: &[Param { name: "aDatabase", ty: "*mut *const nsIRDFCompositeDataSource" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIXULTemplateResult rootResult; */
                    Method {
                        name: "get_rootResult",
                        abi: "C",
                        params: &[Param { name: "aRootResult", ty: "*mut *const nsIXULTemplateResult" }],
                        ret: "nsresult",
                    },

                    /* [noscript] readonly attribute nsIXULTemplateQueryProcessor queryProcessor; */
                    Method {
                        name: "get_queryProcessor",
                        abi: "C",
                        params: &[Param { name: "aQueryProcessor", ty: "*mut *const nsIXULTemplateQueryProcessor" }],
                        ret: "nsresult",
                    },

                    /* void rebuild (); */
                    Method {
                        name: "rebuild",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void refresh (); */
                    Method {
                        name: "refresh",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void addResult (in nsIXULTemplateResult aResult, in nsIDOMNode aQueryNode); */
                    Method {
                        name: "addResult",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "*const nsIXULTemplateResult" }, Param { name: "aQueryNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void removeResult (in nsIXULTemplateResult aResult); */
                    Method {
                        name: "removeResult",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "*const nsIXULTemplateResult" }],
                        ret: "nsresult",
                    },

                    /* void replaceResult (in nsIXULTemplateResult aOldResult, in nsIXULTemplateResult aNewResult, in nsIDOMNode aQueryNode); */
                    Method {
                        name: "replaceResult",
                        abi: "C",
                        params: &[Param { name: "aOldResult", ty: "*const nsIXULTemplateResult" }, Param { name: "aNewResult", ty: "*const nsIXULTemplateResult" }, Param { name: "aQueryNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void resultBindingChanged (in nsIXULTemplateResult aResult); */
                    Method {
                        name: "resultBindingChanged",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "*const nsIXULTemplateResult" }],
                        ret: "nsresult",
                    },

                    /* nsIXULTemplateResult getResultForId (in AString aId); */
                    Method {
                        name: "getResultForId",
                        abi: "C",
                        params: &[Param { name: "aId", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIXULTemplateResult" }],
                        ret: "nsresult",
                    },

                    /* nsIXULTemplateResult getResultForContent (in nsIDOMElement aElement); */
                    Method {
                        name: "getResultForContent",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIDOMElement" }, Param { name: "_retval", ty: "*mut *const nsIXULTemplateResult" }],
                        ret: "nsresult",
                    },

                    /* boolean hasGeneratedContent (in nsIRDFResource aNode, in nsIAtom aTag); */
                    Method {
                        name: "hasGeneratedContent",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsIRDFResource" }, Param { name: "aTag", ty: "*const nsIAtom" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void addRuleFilter (in nsIDOMNode aRule, in nsIXULTemplateRuleFilter aFilter); */
                    Method {
                        name: "addRuleFilter",
                        abi: "C",
                        params: &[Param { name: "aRule", ty: "*const nsIDOMNode" }, Param { name: "aFilter", ty: "*const nsIXULTemplateRuleFilter" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void init (in nsIContent aElement); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIContent" }],
                        ret: "nsresult",
                    },

                    /* [noscript] void createContents (in nsIContent aElement, in boolean aForceCreation); */
                    Method {
                        name: "createContents",
                        abi: "C",
                        params: &[Param { name: "aElement", ty: "*const nsIContent" }, Param { name: "aForceCreation", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void addListener (in nsIXULBuilderListener aListener); */
                    Method {
                        name: "addListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIXULBuilderListener" }],
                        ret: "nsresult",
                    },

                    /* void removeListener (in nsIXULBuilderListener aListener); */
                    Method {
                        name: "removeListener",
                        abi: "C",
                        params: &[Param { name: "aListener", ty: "*const nsIXULBuilderListener" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIXULTreeBuilderObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean canDrop (in long index, in long orientation, in nsIDOMDataTransfer dataTransfer); */
                    Method {
                        name: "canDrop",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }, Param { name: "orientation", ty: "libc::int32_t" }, Param { name: "dataTransfer", ty: "*const nsIDOMDataTransfer" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void onDrop (in long row, in long orientation, in nsIDOMDataTransfer dataTransfer); */
                    Method {
                        name: "onDrop",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "orientation", ty: "libc::int32_t" }, Param { name: "dataTransfer", ty: "*const nsIDOMDataTransfer" }],
                        ret: "nsresult",
                    },

                    /* void onToggleOpenState (in long index); */
                    Method {
                        name: "onToggleOpenState",
                        abi: "C",
                        params: &[Param { name: "index", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void onCycleHeader (in wstring colID, in nsIDOMElement elt); */
                    Method {
                        name: "onCycleHeader",
                        abi: "C",
                        params: &[Param { name: "colID", ty: "*const libc::int16_t" }, Param { name: "elt", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void onCycleCell (in long row, in wstring colID); */
                    Method {
                        name: "onCycleCell",
                        abi: "C",
                        params: &[Param { name: "row", ty: "libc::int32_t" }, Param { name: "colID", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void onSelectionChanged (); */
                    Method {
                        name: "onSelectionChanged",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void onPerformAction (in wstring action); */
                    Method {
                        name: "onPerformAction",
                        abi: "C",
                        params: &[Param { name: "action", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    /* void onPerformActionOnRow (in wstring action, in long row); */
                    Method {
                        name: "onPerformActionOnRow",
                        abi: "C",
                        params: &[Param { name: "action", ty: "*const libc::int16_t" }, Param { name: "row", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void onPerformActionOnCell (in wstring action, in long row, in wstring colID); */
                    Method {
                        name: "onPerformActionOnCell",
                        abi: "C",
                        params: &[Param { name: "action", ty: "*const libc::int16_t" }, Param { name: "row", ty: "libc::int32_t" }, Param { name: "colID", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIXULTreeBuilder",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIRDFResource getResourceAtIndex (in long aRowIndex); */
                    Method {
                        name: "getResourceAtIndex",
                        abi: "C",
                        params: &[Param { name: "aRowIndex", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIRDFResource" }],
                        ret: "nsresult",
                    },

                    /* long getIndexOfResource (in nsIRDFResource resource); */
                    Method {
                        name: "getIndexOfResource",
                        abi: "C",
                        params: &[Param { name: "resource", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void addObserver (in nsIXULTreeBuilderObserver aObserver); */
                    Method {
                        name: "addObserver",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIXULTreeBuilderObserver" }],
                        ret: "nsresult",
                    },

                    /* void removeObserver (in nsIXULTreeBuilderObserver aObserver); */
                    Method {
                        name: "removeObserver",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIXULTreeBuilderObserver" }],
                        ret: "nsresult",
                    },

                    /* void sort (in nsIDOMElement aColumnElement); */
                    Method {
                        name: "sort",
                        abi: "C",
                        params: &[Param { name: "aColumnElement", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


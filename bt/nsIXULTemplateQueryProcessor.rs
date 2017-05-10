//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULTemplateQueryProcessor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXULTemplateQueryProcessor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISupports getDatasource (in nsIArray aDataSources, in nsIDOMNode aRootNode, in boolean aIsTrusted, in nsIXULTemplateBuilder aBuilder, out boolean aShouldDelayBuilding); */
                    Method {
                        name: "getDatasource",
                        abi: "C",
                        params: &[Param { name: "aDataSources", ty: "*const nsIArray" }, Param { name: "aRootNode", ty: "*const nsIDOMNode" }, Param { name: "aIsTrusted", ty: "bool" }, Param { name: "aBuilder", ty: "*const nsIXULTemplateBuilder" }, Param { name: "aShouldDelayBuilding", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void initializeForBuilding (in nsISupports aDatasource, in nsIXULTemplateBuilder aBuilder, in nsIDOMNode aRootNode); */
                    Method {
                        name: "initializeForBuilding",
                        abi: "C",
                        params: &[Param { name: "aDatasource", ty: "*const nsISupports" }, Param { name: "aBuilder", ty: "*const nsIXULTemplateBuilder" }, Param { name: "aRootNode", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void done (); */
                    Method {
                        name: "done",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* nsISupports compileQuery (in nsIXULTemplateBuilder aBuilder, in nsIDOMNode aQuery, in nsIAtom aRefVariable, in nsIAtom aMemberVariable); */
                    Method {
                        name: "compileQuery",
                        abi: "C",
                        params: &[Param { name: "aBuilder", ty: "*const nsIXULTemplateBuilder" }, Param { name: "aQuery", ty: "*const nsIDOMNode" }, Param { name: "aRefVariable", ty: "*const nsIAtom" }, Param { name: "aMemberVariable", ty: "*const nsIAtom" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator generateResults (in nsISupports aDatasource, in nsIXULTemplateResult aRef, in nsISupports aQuery); */
                    Method {
                        name: "generateResults",
                        abi: "C",
                        params: &[Param { name: "aDatasource", ty: "*const nsISupports" }, Param { name: "aRef", ty: "*const nsIXULTemplateResult" }, Param { name: "aQuery", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* void addBinding (in nsIDOMNode aRuleNode, in nsIAtom aVar, in nsIAtom aRef, in AString aExpr); */
                    Method {
                        name: "addBinding",
                        abi: "C",
                        params: &[Param { name: "aRuleNode", ty: "*const nsIDOMNode" }, Param { name: "aVar", ty: "*const nsIAtom" }, Param { name: "aRef", ty: "*const nsIAtom" }, Param { name: "aExpr", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* nsIXULTemplateResult translateRef (in nsISupports aDatasource, in AString aRefString); */
                    Method {
                        name: "translateRef",
                        abi: "C",
                        params: &[Param { name: "aDatasource", ty: "*const nsISupports" }, Param { name: "aRefString", ty: "*const nsAString" }, Param { name: "_retval", ty: "*mut *const nsIXULTemplateResult" }],
                        ret: "nsresult",
                    },

                    /* int32_t compareResults (in nsIXULTemplateResult aLeft, in nsIXULTemplateResult aRight, in nsIAtom aVar, in unsigned long aSortHints); */
                    Method {
                        name: "compareResults",
                        abi: "C",
                        params: &[Param { name: "aLeft", ty: "*const nsIXULTemplateResult" }, Param { name: "aRight", ty: "*const nsIXULTemplateResult" }, Param { name: "aVar", ty: "*const nsIAtom" }, Param { name: "aSortHints", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


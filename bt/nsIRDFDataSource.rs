//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFDataSource.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFDataSource",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute string URI; */
                    Method {
                        name: "get_URI",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* nsIRDFResource GetSource (in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
                    Method {
                        name: "GetSource",
                        abi: "C",
                        params: &[Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aTarget", ty: "*const nsIRDFNode" }, Param { name: "aTruthValue", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIRDFResource" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator GetSources (in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
                    Method {
                        name: "GetSources",
                        abi: "C",
                        params: &[Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aTarget", ty: "*const nsIRDFNode" }, Param { name: "aTruthValue", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsIRDFNode GetTarget (in nsIRDFResource aSource, in nsIRDFResource aProperty, in boolean aTruthValue); */
                    Method {
                        name: "GetTarget",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIRDFResource" }, Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aTruthValue", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsIRDFNode" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator GetTargets (in nsIRDFResource aSource, in nsIRDFResource aProperty, in boolean aTruthValue); */
                    Method {
                        name: "GetTargets",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIRDFResource" }, Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aTruthValue", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* void Assert (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
                    Method {
                        name: "Assert",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIRDFResource" }, Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aTarget", ty: "*const nsIRDFNode" }, Param { name: "aTruthValue", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void Unassert (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
                    Method {
                        name: "Unassert",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIRDFResource" }, Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aTarget", ty: "*const nsIRDFNode" }],
                        ret: "nsresult",
                    },

                    /* void Change (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aOldTarget, in nsIRDFNode aNewTarget); */
                    Method {
                        name: "Change",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIRDFResource" }, Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aOldTarget", ty: "*const nsIRDFNode" }, Param { name: "aNewTarget", ty: "*const nsIRDFNode" }],
                        ret: "nsresult",
                    },

                    /* void Move (in nsIRDFResource aOldSource, in nsIRDFResource aNewSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
                    Method {
                        name: "Move",
                        abi: "C",
                        params: &[Param { name: "aOldSource", ty: "*const nsIRDFResource" }, Param { name: "aNewSource", ty: "*const nsIRDFResource" }, Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aTarget", ty: "*const nsIRDFNode" }],
                        ret: "nsresult",
                    },

                    /* boolean HasAssertion (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
                    Method {
                        name: "HasAssertion",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIRDFResource" }, Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aTarget", ty: "*const nsIRDFNode" }, Param { name: "aTruthValue", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void AddObserver (in nsIRDFObserver aObserver); */
                    Method {
                        name: "AddObserver",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIRDFObserver" }],
                        ret: "nsresult",
                    },

                    /* void RemoveObserver (in nsIRDFObserver aObserver); */
                    Method {
                        name: "RemoveObserver",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIRDFObserver" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator ArcLabelsIn (in nsIRDFNode aNode); */
                    Method {
                        name: "ArcLabelsIn",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsIRDFNode" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator ArcLabelsOut (in nsIRDFResource aSource); */
                    Method {
                        name: "ArcLabelsOut",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator GetAllResources (); */
                    Method {
                        name: "GetAllResources",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* boolean IsCommandEnabled (in nsISupports aSources, in nsIRDFResource aCommand, in nsISupports aArguments); */
                    Method {
                        name: "IsCommandEnabled",
                        abi: "C",
                        params: &[Param { name: "aSources", ty: "*const nsISupports" }, Param { name: "aCommand", ty: "*const nsIRDFResource" }, Param { name: "aArguments", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void DoCommand (in nsISupports aSources, in nsIRDFResource aCommand, in nsISupports aArguments); */
                    Method {
                        name: "DoCommand",
                        abi: "C",
                        params: &[Param { name: "aSources", ty: "*const nsISupports" }, Param { name: "aCommand", ty: "*const nsIRDFResource" }, Param { name: "aArguments", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator GetAllCmds (in nsIRDFResource aSource); */
                    Method {
                        name: "GetAllCmds",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    /* boolean hasArcIn (in nsIRDFNode aNode, in nsIRDFResource aArc); */
                    Method {
                        name: "hasArcIn",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsIRDFNode" }, Param { name: "aArc", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean hasArcOut (in nsIRDFResource aSource, in nsIRDFResource aArc); */
                    Method {
                        name: "hasArcOut",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIRDFResource" }, Param { name: "aArc", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void beginUpdateBatch (); */
                    Method {
                        name: "beginUpdateBatch",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void endUpdateBatch (); */
                    Method {
                        name: "endUpdateBatch",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


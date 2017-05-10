//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onAssert (in nsIRDFDataSource aDataSource, in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
                    Method {
                        name: "onAssert",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aSource", ty: "*const nsIRDFResource" }, Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aTarget", ty: "*const nsIRDFNode" }],
                        ret: "nsresult",
                    },

                    /* void onUnassert (in nsIRDFDataSource aDataSource, in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
                    Method {
                        name: "onUnassert",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aSource", ty: "*const nsIRDFResource" }, Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aTarget", ty: "*const nsIRDFNode" }],
                        ret: "nsresult",
                    },

                    /* void onChange (in nsIRDFDataSource aDataSource, in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aOldTarget, in nsIRDFNode aNewTarget); */
                    Method {
                        name: "onChange",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aSource", ty: "*const nsIRDFResource" }, Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aOldTarget", ty: "*const nsIRDFNode" }, Param { name: "aNewTarget", ty: "*const nsIRDFNode" }],
                        ret: "nsresult",
                    },

                    /* void onMove (in nsIRDFDataSource aDataSource, in nsIRDFResource aOldSource, in nsIRDFResource aNewSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget); */
                    Method {
                        name: "onMove",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aOldSource", ty: "*const nsIRDFResource" }, Param { name: "aNewSource", ty: "*const nsIRDFResource" }, Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aTarget", ty: "*const nsIRDFNode" }],
                        ret: "nsresult",
                    },

                    /* void onBeginUpdateBatch (in nsIRDFDataSource aDataSource); */
                    Method {
                        name: "onBeginUpdateBatch",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }],
                        ret: "nsresult",
                    },

                    /* void onEndUpdateBatch (in nsIRDFDataSource aDataSource); */
                    Method {
                        name: "onEndUpdateBatch",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


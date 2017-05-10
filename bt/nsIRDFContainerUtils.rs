//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFContainerUtils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFContainerUtils",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean IsOrdinalProperty (in nsIRDFResource aProperty); */
                    Method {
                        name: "IsOrdinalProperty",
                        abi: "C",
                        params: &[Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIRDFResource IndexToOrdinalResource (in long aIndex); */
                    Method {
                        name: "IndexToOrdinalResource",
                        abi: "C",
                        params: &[Param { name: "aIndex", ty: "libc::int32_t" }, Param { name: "_retval", ty: "*mut *const nsIRDFResource" }],
                        ret: "nsresult",
                    },

                    /* long OrdinalResourceToIndex (in nsIRDFResource aOrdinal); */
                    Method {
                        name: "OrdinalResourceToIndex",
                        abi: "C",
                        params: &[Param { name: "aOrdinal", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean IsContainer (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
                    Method {
                        name: "IsContainer",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aResource", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean IsEmpty (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
                    Method {
                        name: "IsEmpty",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aResource", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean IsBag (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
                    Method {
                        name: "IsBag",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aResource", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean IsSeq (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
                    Method {
                        name: "IsSeq",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aResource", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean IsAlt (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
                    Method {
                        name: "IsAlt",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aResource", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* nsIRDFContainer MakeBag (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
                    Method {
                        name: "MakeBag",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aResource", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut *const nsIRDFContainer" }],
                        ret: "nsresult",
                    },

                    /* nsIRDFContainer MakeSeq (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
                    Method {
                        name: "MakeSeq",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aResource", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut *const nsIRDFContainer" }],
                        ret: "nsresult",
                    },

                    /* nsIRDFContainer MakeAlt (in nsIRDFDataSource aDataSource, in nsIRDFResource aResource); */
                    Method {
                        name: "MakeAlt",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aResource", ty: "*const nsIRDFResource" }, Param { name: "_retval", ty: "*mut *const nsIRDFContainer" }],
                        ret: "nsresult",
                    },

                    /* long indexOf (in nsIRDFDataSource aDataSource, in nsIRDFResource aContainer, in nsIRDFNode aElement); */
                    Method {
                        name: "indexOf",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }, Param { name: "aContainer", ty: "*const nsIRDFResource" }, Param { name: "aElement", ty: "*const nsIRDFNode" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


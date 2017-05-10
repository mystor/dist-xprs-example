//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFCompositeDataSource.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFCompositeDataSource",
            base: Some("nsIRDFDataSource"),
            methods: Some(&[
                    /* attribute boolean allowNegativeAssertions; */
                    Method {
                        name: "get_allowNegativeAssertions",
                        abi: "C",
                        params: &[Param { name: "aAllowNegativeAssertions", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_allowNegativeAssertions",
                        abi: "C",
                        params: &[Param { name: "aAllowNegativeAssertions", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* attribute boolean coalesceDuplicateArcs; */
                    Method {
                        name: "get_coalesceDuplicateArcs",
                        abi: "C",
                        params: &[Param { name: "aCoalesceDuplicateArcs", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_coalesceDuplicateArcs",
                        abi: "C",
                        params: &[Param { name: "aCoalesceDuplicateArcs", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void AddDataSource (in nsIRDFDataSource aDataSource); */
                    Method {
                        name: "AddDataSource",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }],
                        ret: "nsresult",
                    },

                    /* void RemoveDataSource (in nsIRDFDataSource aDataSource); */
                    Method {
                        name: "RemoveDataSource",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }],
                        ret: "nsresult",
                    },

                    /* nsISimpleEnumerator GetDataSources (); */
                    Method {
                        name: "GetDataSources",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFPurgeableDataSource.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFPurgeableDataSource",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean Mark (in nsIRDFResource aSource, in nsIRDFResource aProperty, in nsIRDFNode aTarget, in boolean aTruthValue); */
                    Method {
                        name: "Mark",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIRDFResource" }, Param { name: "aProperty", ty: "*const nsIRDFResource" }, Param { name: "aTarget", ty: "*const nsIRDFNode" }, Param { name: "aTruthValue", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void Sweep (); */
                    Method {
                        name: "Sweep",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


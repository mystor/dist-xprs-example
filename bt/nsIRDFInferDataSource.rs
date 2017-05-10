//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFInferDataSource.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFInferDataSource",
            base: Some("nsIRDFDataSource"),
            methods: Some(&[
                    /* attribute nsIRDFDataSource baseDataSource; */
                    Method {
                        name: "get_baseDataSource",
                        abi: "C",
                        params: &[Param { name: "aBaseDataSource", ty: "*mut *const nsIRDFDataSource" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_baseDataSource",
                        abi: "C",
                        params: &[Param { name: "aBaseDataSource", ty: "*const nsIRDFDataSource" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


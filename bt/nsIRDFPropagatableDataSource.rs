//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFPropagatableDataSource.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFPropagatableDataSource",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute boolean propagateChanges; */
                    Method {
                        name: "get_propagateChanges",
                        abi: "C",
                        params: &[Param { name: "aPropagateChanges", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_propagateChanges",
                        abi: "C",
                        params: &[Param { name: "aPropagateChanges", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


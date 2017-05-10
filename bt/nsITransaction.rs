//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransaction.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITransaction",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void doTransaction (); */
                    Method {
                        name: "doTransaction",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void undoTransaction (); */
                    Method {
                        name: "undoTransaction",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void redoTransaction (); */
                    Method {
                        name: "redoTransaction",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* readonly attribute boolean isTransient; */
                    Method {
                        name: "get_isTransient",
                        abi: "C",
                        params: &[Param { name: "aIsTransient", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* boolean merge (in nsITransaction aTransaction); */
                    Method {
                        name: "merge",
                        abi: "C",
                        params: &[Param { name: "aTransaction", ty: "*const nsITransaction" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


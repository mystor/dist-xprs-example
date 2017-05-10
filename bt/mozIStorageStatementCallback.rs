//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageStatementCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageStatementCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleResult (in mozIStorageResultSet aResultSet); */
                    Method {
                        name: "handleResult",
                        abi: "C",
                        params: &[Param { name: "aResultSet", ty: "*const mozIStorageResultSet" }],
                        ret: "nsresult",
                    },

                    /* void handleError (in mozIStorageError aError); */
                    Method {
                        name: "handleError",
                        abi: "C",
                        params: &[Param { name: "aError", ty: "*const mozIStorageError" }],
                        ret: "nsresult",
                    },

                    /* void handleCompletion (in unsigned short aReason); */
                    Method {
                        name: "handleCompletion",
                        abi: "C",
                        params: &[Param { name: "aReason", ty: "libc::uint16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


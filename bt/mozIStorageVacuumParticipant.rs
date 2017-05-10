//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageVacuumParticipant.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageVacuumParticipant",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute long expectedDatabasePageSize; */
                    Method {
                        name: "get_expectedDatabasePageSize",
                        abi: "C",
                        params: &[Param { name: "aExpectedDatabasePageSize", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute mozIStorageConnection databaseConnection; */
                    Method {
                        name: "get_databaseConnection",
                        abi: "C",
                        params: &[Param { name: "aDatabaseConnection", ty: "*mut *const mozIStorageConnection" }],
                        ret: "nsresult",
                    },

                    /* boolean onBeginVacuum (); */
                    Method {
                        name: "onBeginVacuum",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void onEndVacuum (in boolean aSucceeded); */
                    Method {
                        name: "onEndVacuum",
                        abi: "C",
                        params: &[Param { name: "aSucceeded", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageCompletionCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageCompletionCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void complete (in nsresult status, [optional] in nsISupports value); */
                    Method {
                        name: "complete",
                        abi: "C",
                        params: &[Param { name: "status", ty: "nsresult" }, Param { name: "value", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


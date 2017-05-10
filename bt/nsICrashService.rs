//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICrashService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICrashService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void addCrash (in long processType, in long crashType, in AString id); */
                    Method {
                        name: "addCrash",
                        abi: "C",
                        params: &[Param { name: "processType", ty: "libc::int32_t" }, Param { name: "crashType", ty: "libc::int32_t" }, Param { name: "id", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFileURL.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFileURL",
            base: Some("nsIURL"),
            methods: Some(&[
                    /* attribute nsIFile file; */
                    Method {
                        name: "get_file",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_file",
                        abi: "C",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


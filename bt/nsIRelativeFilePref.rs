//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRelativeFilePref.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRelativeFilePref",
            base: Some("nsISupports"),
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

                    /* attribute ACString relativeToKey; */
                    Method {
                        name: "get_relativeToKey",
                        abi: "C",
                        params: &[Param { name: "aRelativeToKey", ty: "*mut nsACString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_relativeToKey",
                        abi: "C",
                        params: &[Param { name: "aRelativeToKey", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDirectoryEnumerator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDirectoryEnumerator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIFile nextFile; */
                    Method {
                        name: "get_nextFile",
                        abi: "C",
                        params: &[Param { name: "aNextFile", ty: "*mut *const nsIFile" }],
                        ret: "nsresult",
                    },

                    /* void close (); */
                    Method {
                        name: "close",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


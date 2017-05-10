//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMemory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMemory",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void heapMinimize (in boolean immediate); */
                    Method {
                        name: "heapMinimize",
                        abi: "C",
                        params: &[Param { name: "immediate", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* boolean isLowMemoryPlatform (); */
                    Method {
                        name: "isLowMemoryPlatform",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


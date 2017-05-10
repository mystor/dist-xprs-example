//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIApplicationCacheContainer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIApplicationCacheContainer",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute nsIApplicationCache applicationCache; */
                    Method {
                        name: "get_applicationCache",
                        abi: "C",
                        params: &[Param { name: "aApplicationCache", ty: "*mut *const nsIApplicationCache" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_applicationCache",
                        abi: "C",
                        params: &[Param { name: "aApplicationCache", ty: "*const nsIApplicationCache" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISocketFilter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISocketFilter",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsISocketFilterHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISocketFilter newFilter (); */
                    Method {
                        name: "newFilter",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISocketFilter" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


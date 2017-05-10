//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMutable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMutable",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute boolean mutable; */
                    Method {
                        name: "get_mutable",
                        abi: "C",
                        params: &[Param { name: "aMutable", ty: "*mut bool" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_mutable",
                        abi: "C",
                        params: &[Param { name: "aMutable", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


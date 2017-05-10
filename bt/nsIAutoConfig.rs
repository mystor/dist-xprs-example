//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoConfig.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAutoConfig",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute string configURL; */
                    Method {
                        name: "get_configURL",
                        abi: "C",
                        params: &[Param { name: "aConfigURL", ty: "*mut *const libc::c_char" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_configURL",
                        abi: "C",
                        params: &[Param { name: "aConfigURL", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


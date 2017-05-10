//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebNavigationInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebNavigationInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* unsigned long isTypeSupported (in ACString aType, in nsIWebNavigation aWebNav); */
                    Method {
                        name: "isTypeSupported",
                        abi: "C",
                        params: &[Param { name: "aType", ty: "*const nsACString" }, Param { name: "aWebNav", ty: "*const nsIWebNavigation" }, Param { name: "_retval", ty: "*mut libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


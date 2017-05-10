//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRecoveryService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRecoveryService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void factoryReset (in string reason); */
                    Method {
                        name: "factoryReset",
                        abi: "C",
                        params: &[Param { name: "reason", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* void installFotaUpdate (in string updatePath); */
                    Method {
                        name: "installFotaUpdate",
                        abi: "C",
                        params: &[Param { name: "updatePath", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    /* long getFotaUpdateStatus (); */
                    Method {
                        name: "getFotaUpdateStatus",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


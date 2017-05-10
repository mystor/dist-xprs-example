//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInterfaceRequestor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInterfaceRequestor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void getInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
                    Method {
                        name: "getInterface",
                        abi: "C",
                        params: &[Param { name: "uuid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


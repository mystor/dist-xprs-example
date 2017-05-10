//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISocketProviderService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISocketProviderService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISocketProvider getSocketProvider (in string socketType); */
                    Method {
                        name: "getSocketProvider",
                        abi: "C",
                        params: &[Param { name: "socketType", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsISocketProvider" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


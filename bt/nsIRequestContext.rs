//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRequestContext.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRequestContext",
            base: Some("nsISupports"),
            methods: None,
        },


        Interface {
            name: "nsIRequestContextService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIRequestContext getRequestContext (in unsigned long long id); */
                    Method {
                        name: "getRequestContext",
                        abi: "C",
                        params: &[Param { name: "id", ty: "libc::uint64_t" }, Param { name: "_retval", ty: "*mut *const nsIRequestContext" }],
                        ret: "nsresult",
                    },

                    /* nsIRequestContext newRequestContext (); */
                    Method {
                        name: "newRequestContext",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIRequestContext" }],
                        ret: "nsresult",
                    },

                    /* void removeRequestContext (in unsigned long long id); */
                    Method {
                        name: "removeRequestContext",
                        abi: "C",
                        params: &[Param { name: "id", ty: "libc::uint64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


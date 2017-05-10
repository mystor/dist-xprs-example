//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISupports.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISupports",
            base: None,
            methods: Some(&[
                    /* void QueryInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
                    Method {
                        name: "QueryInterface",
                        abi: "C",
                        params: &[Param { name: "uuid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* [noscript,notxpcom] nsrefcnt AddRef (); */
                    Method {
                        name: "AddRef",
                        abi: "C",
                        params: &[],
                        ret: "nsrefcnt",
                    },

                    /* [noscript,notxpcom] nsrefcnt Release (); */
                    Method {
                        name: "Release",
                        abi: "C",
                        params: &[],
                        ret: "nsrefcnt",
                    },

                    ]),
        },


        ]; D}


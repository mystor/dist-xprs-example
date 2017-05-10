//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWeakReference.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWeakReference",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void QueryReferent (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
                    Method {
                        name: "QueryReferent",
                        abi: "C",
                        params: &[Param { name: "uuid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsISupportsWeakReference",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIWeakReference GetWeakReference (); */
                    Method {
                        name: "GetWeakReference",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIWeakReference" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFactory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFactory",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void createInstance (in nsISupports aOuter, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
                    Method {
                        name: "createInstance",
                        abi: "C",
                        params: &[Param { name: "aOuter", ty: "*const nsISupports" }, Param { name: "iid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    /* void lockFactory (in boolean lock); */
                    Method {
                        name: "lockFactory",
                        abi: "C",
                        params: &[Param { name: "lock", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


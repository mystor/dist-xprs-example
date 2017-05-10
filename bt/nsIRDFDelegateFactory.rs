//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFDelegateFactory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFDelegateFactory",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void CreateDelegate (in nsIRDFResource aOuter, in string aKey, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult aResult); */
                    Method {
                        name: "CreateDelegate",
                        abi: "C",
                        params: &[Param { name: "aOuter", ty: "*const nsIRDFResource" }, Param { name: "aKey", ty: "*const libc::c_char" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "aResult", ty: "*mut *const libc::c_void" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


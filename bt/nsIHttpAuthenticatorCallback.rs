//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpAuthenticatorCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpAuthenticatorCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onCredsGenerated (in string aCreds, in unsigned long aFlags, in nsresult aResult, in nsISupports aSessionsState, in nsISupports aContinuationState); */
                    Method {
                        name: "onCredsGenerated",
                        abi: "C",
                        params: &[Param { name: "aCreds", ty: "*const libc::c_char" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "aResult", ty: "nsresult" }, Param { name: "aSessionsState", ty: "*const nsISupports" }, Param { name: "aContinuationState", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


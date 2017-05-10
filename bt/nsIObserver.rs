//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void observe (in nsISupports aSubject, in string aTopic, in wstring aData); */
                    Method {
                        name: "observe",
                        abi: "C",
                        params: &[Param { name: "aSubject", ty: "*const nsISupports" }, Param { name: "aTopic", ty: "*const libc::c_char" }, Param { name: "aData", ty: "*const libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


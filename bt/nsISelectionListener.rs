//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISelectionListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISelectionListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void notifySelectionChanged (in nsIDOMDocument doc, in nsISelection sel, in short reason); */
                    Method {
                        name: "notifySelectionChanged",
                        abi: "C",
                        params: &[Param { name: "doc", ty: "*const nsIDOMDocument" }, Param { name: "sel", ty: "*const nsISelection" }, Param { name: "reason", ty: "libc::int16_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


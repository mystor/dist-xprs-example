//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPICommandUpdater.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsPICommandUpdater",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* void commandStatusChanged (in string aCommandName); */
                    Method {
                        name: "commandStatusChanged",
                        abi: "C",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/NotXPCOMTest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "ScriptableOK",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void method1 (); */
                    Method {
                        name: "method1",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "ScriptableWithNotXPCOM",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [notxpcom] void method2 (); */
                    Method {
                        name: "method2",
                        abi: "C",
                        params: &[],
                        ret: "libc::c_void",
                    },

                    ]),
        },


        Interface {
            name: "ScriptableWithNotXPCOMBase",
            base: Some("ScriptableWithNotXPCOM"),
            methods: Some(&[
                    /* void method3 (); */
                    Method {
                        name: "method3",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


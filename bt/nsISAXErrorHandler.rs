//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXErrorHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISAXErrorHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void error (in nsISAXLocator locator, in AString error); */
                    Method {
                        name: "error",
                        abi: "C",
                        params: &[Param { name: "locator", ty: "*const nsISAXLocator" }, Param { name: "error", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void fatalError (in nsISAXLocator locator, in AString error); */
                    Method {
                        name: "fatalError",
                        abi: "C",
                        params: &[Param { name: "locator", ty: "*const nsISAXLocator" }, Param { name: "error", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void ignorableWarning (in nsISAXLocator locator, in AString error); */
                    Method {
                        name: "ignorableWarning",
                        abi: "C",
                        params: &[Param { name: "locator", ty: "*const nsISAXLocator" }, Param { name: "error", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptLoaderObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptLoaderObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void scriptAvailable (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInline, in nsIURI aURI, in int32_t aLineNo); */
                    Method {
                        name: "scriptAvailable",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "nsresult" }, Param { name: "aElement", ty: "*const nsIScriptElement" }, Param { name: "aIsInline", ty: "bool" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aLineNo", ty: "int32_t" }],
                        ret: "nsresult",
                    },

                    /* void scriptEvaluated (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInline); */
                    Method {
                        name: "scriptEvaluated",
                        abi: "C",
                        params: &[Param { name: "aResult", ty: "nsresult" }, Param { name: "aElement", ty: "*const nsIScriptElement" }, Param { name: "aIsInline", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


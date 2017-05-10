//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULBuilderListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXULBuilderListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void willRebuild (in nsIXULTemplateBuilder aBuilder); */
                    Method {
                        name: "willRebuild",
                        abi: "C",
                        params: &[Param { name: "aBuilder", ty: "*const nsIXULTemplateBuilder" }],
                        ret: "nsresult",
                    },

                    /* void didRebuild (in nsIXULTemplateBuilder aBuilder); */
                    Method {
                        name: "didRebuild",
                        abi: "C",
                        params: &[Param { name: "aBuilder", ty: "*const nsIXULTemplateBuilder" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


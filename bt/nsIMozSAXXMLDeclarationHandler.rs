//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMozSAXXMLDeclarationHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMozSAXXMLDeclarationHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void handleXMLDeclaration (in AString version, in AString encoding, in boolean standalone); */
                    Method {
                        name: "handleXMLDeclaration",
                        abi: "C",
                        params: &[Param { name: "version", ty: "*const nsAString" }, Param { name: "encoding", ty: "*const nsAString" }, Param { name: "standalone", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


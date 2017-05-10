//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXDTDHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISAXDTDHandler",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void notationDecl (in AString name, in AString publicId, in AString systemId); */
                    Method {
                        name: "notationDecl",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "publicId", ty: "*const nsAString" }, Param { name: "systemId", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void unparsedEntityDecl (in AString name, in AString publicId, in AString systemId, in AString notationName); */
                    Method {
                        name: "unparsedEntityDecl",
                        abi: "C",
                        params: &[Param { name: "name", ty: "*const nsAString" }, Param { name: "publicId", ty: "*const nsAString" }, Param { name: "systemId", ty: "*const nsAString" }, Param { name: "notationName", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


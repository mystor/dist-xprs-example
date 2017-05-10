//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocumentEncoder.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDocumentEncoderNodeFixup",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIDOMNode fixupNode (in nsIDOMNode aNode, out boolean aSerializeCloneKids); */
                    Method {
                        name: "fixupNode",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsIDOMNode" }, Param { name: "aSerializeCloneKids", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIDocumentEncoder",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}


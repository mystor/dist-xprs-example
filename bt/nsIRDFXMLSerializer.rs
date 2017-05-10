//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFXMLSerializer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFXMLSerializer",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void init (in nsIRDFDataSource aDataSource); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aDataSource", ty: "*const nsIRDFDataSource" }],
                        ret: "nsresult",
                    },

                    /* void addNameSpace (in nsIAtom aPrefix, in DOMString aURI); */
                    Method {
                        name: "addNameSpace",
                        abi: "C",
                        params: &[Param { name: "aPrefix", ty: "*const nsIAtom" }, Param { name: "aURI", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFXMLParser.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRDFXMLParser",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsIStreamListener parseAsync (in nsIRDFDataSource aSink, in nsIURI aBaseURI); */
                    Method {
                        name: "parseAsync",
                        abi: "C",
                        params: &[Param { name: "aSink", ty: "*const nsIRDFDataSource" }, Param { name: "aBaseURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut *const nsIStreamListener" }],
                        ret: "nsresult",
                    },

                    /* void parseString (in nsIRDFDataSource aSink, in nsIURI aBaseURI, in AUTF8String aSource); */
                    Method {
                        name: "parseString",
                        abi: "C",
                        params: &[Param { name: "aSink", ty: "*const nsIRDFDataSource" }, Param { name: "aBaseURI", ty: "*const nsIURI" }, Param { name: "aSource", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


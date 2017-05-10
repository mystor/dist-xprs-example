//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/rdfITripleVisitor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "rdfITripleVisitor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void visit (in nsIRDFNode aSubject, in nsIRDFResource aPredicate, in nsIRDFNode aObject, in boolean aTruthValue); */
                    Method {
                        name: "visit",
                        abi: "C",
                        params: &[Param { name: "aSubject", ty: "*const nsIRDFNode" }, Param { name: "aPredicate", ty: "*const nsIRDFResource" }, Param { name: "aObject", ty: "*const nsIRDFNode" }, Param { name: "aTruthValue", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/rdfIDataSource.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "rdfIDataSource",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void visitAllSubjects (in rdfITripleVisitor aVisitor); */
                    Method {
                        name: "visitAllSubjects",
                        abi: "C",
                        params: &[Param { name: "aVisitor", ty: "*const rdfITripleVisitor" }],
                        ret: "nsresult",
                    },

                    /* void visitAllTriples (in rdfITripleVisitor aVisitor); */
                    Method {
                        name: "visitAllTriples",
                        abi: "C",
                        params: &[Param { name: "aVisitor", ty: "*const rdfITripleVisitor" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


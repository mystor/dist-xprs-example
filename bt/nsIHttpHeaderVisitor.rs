//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpHeaderVisitor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpHeaderVisitor",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* [must_use] void visitHeader (in ACString aHeader, in ACString aValue); */
                    Method {
                        name: "visitHeader",
                        abi: "C",
                        params: &[Param { name: "aHeader", ty: "*const nsACString" }, Param { name: "aValue", ty: "*const nsACString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


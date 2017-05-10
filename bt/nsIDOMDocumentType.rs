//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDocumentType.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMDocumentType",
            base: Some("nsIDOMNode"),
            methods: Some(&[
                    /* readonly attribute DOMString name; */
                    Method {
                        name: "get_name",
                        abi: "C",
                        params: &[Param { name: "aName", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString publicId; */
                    Method {
                        name: "get_publicId",
                        abi: "C",
                        params: &[Param { name: "aPublicId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString systemId; */
                    Method {
                        name: "get_systemId",
                        abi: "C",
                        params: &[Param { name: "aSystemId", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute DOMString internalSubset; */
                    Method {
                        name: "get_internalSubset",
                        abi: "C",
                        params: &[Param { name: "aInternalSubset", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


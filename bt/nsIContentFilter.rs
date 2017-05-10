//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentFilter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentFilter",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void notifyOfInsertion (in AString mimeType, in nsIURL contentSourceURL, in nsIDOMDocument sourceDocument, in boolean willDeleteSelection, inout nsIDOMNode docFragment, inout nsIDOMNode contentStartNode, inout long contentStartOffset, inout nsIDOMNode contentEndNode, inout long contentEndOffset, inout nsIDOMNode insertionPointNode, inout long insertionPointOffset, out boolean continueWithInsertion); */
                    Method {
                        name: "notifyOfInsertion",
                        abi: "C",
                        params: &[Param { name: "mimeType", ty: "*const nsAString" }, Param { name: "contentSourceURL", ty: "*const nsIURL" }, Param { name: "sourceDocument", ty: "*const nsIDOMDocument" }, Param { name: "willDeleteSelection", ty: "bool" }, Param { name: "docFragment", ty: "*mut *const nsIDOMNode" }, Param { name: "contentStartNode", ty: "*mut *const nsIDOMNode" }, Param { name: "contentStartOffset", ty: "*mut libc::int32_t" }, Param { name: "contentEndNode", ty: "*mut *const nsIDOMNode" }, Param { name: "contentEndOffset", ty: "*mut libc::int32_t" }, Param { name: "insertionPointNode", ty: "*mut *const nsIDOMNode" }, Param { name: "insertionPointOffset", ty: "*mut libc::int32_t" }, Param { name: "continueWithInsertion", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


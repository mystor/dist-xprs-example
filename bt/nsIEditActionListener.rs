//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditActionListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEditActionListener",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void WillCreateNode (in DOMString aTag, in nsIDOMNode aParent, in long aPosition); */
                    Method {
                        name: "WillCreateNode",
                        abi: "C",
                        params: &[Param { name: "aTag", ty: "*const nsAString" }, Param { name: "aParent", ty: "*const nsIDOMNode" }, Param { name: "aPosition", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void DidCreateNode (in DOMString aTag, in nsIDOMNode aNode, in nsIDOMNode aParent, in long aPosition, in nsresult aResult); */
                    Method {
                        name: "DidCreateNode",
                        abi: "C",
                        params: &[Param { name: "aTag", ty: "*const nsAString" }, Param { name: "aNode", ty: "*const nsIDOMNode" }, Param { name: "aParent", ty: "*const nsIDOMNode" }, Param { name: "aPosition", ty: "libc::int32_t" }, Param { name: "aResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void WillInsertNode (in nsIDOMNode aNode, in nsIDOMNode aParent, in long aPosition); */
                    Method {
                        name: "WillInsertNode",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsIDOMNode" }, Param { name: "aParent", ty: "*const nsIDOMNode" }, Param { name: "aPosition", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void DidInsertNode (in nsIDOMNode aNode, in nsIDOMNode aParent, in long aPosition, in nsresult aResult); */
                    Method {
                        name: "DidInsertNode",
                        abi: "C",
                        params: &[Param { name: "aNode", ty: "*const nsIDOMNode" }, Param { name: "aParent", ty: "*const nsIDOMNode" }, Param { name: "aPosition", ty: "libc::int32_t" }, Param { name: "aResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void WillDeleteNode (in nsIDOMNode aChild); */
                    Method {
                        name: "WillDeleteNode",
                        abi: "C",
                        params: &[Param { name: "aChild", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void DidDeleteNode (in nsIDOMNode aChild, in nsresult aResult); */
                    Method {
                        name: "DidDeleteNode",
                        abi: "C",
                        params: &[Param { name: "aChild", ty: "*const nsIDOMNode" }, Param { name: "aResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void WillSplitNode (in nsIDOMNode aExistingRightNode, in long aOffset); */
                    Method {
                        name: "WillSplitNode",
                        abi: "C",
                        params: &[Param { name: "aExistingRightNode", ty: "*const nsIDOMNode" }, Param { name: "aOffset", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void DidSplitNode (in nsIDOMNode aExistingRightNode, in long aOffset, in nsIDOMNode aNewLeftNode, in nsresult aResult); */
                    Method {
                        name: "DidSplitNode",
                        abi: "C",
                        params: &[Param { name: "aExistingRightNode", ty: "*const nsIDOMNode" }, Param { name: "aOffset", ty: "libc::int32_t" }, Param { name: "aNewLeftNode", ty: "*const nsIDOMNode" }, Param { name: "aResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void WillJoinNodes (in nsIDOMNode aLeftNode, in nsIDOMNode aRightNode, in nsIDOMNode aParent); */
                    Method {
                        name: "WillJoinNodes",
                        abi: "C",
                        params: &[Param { name: "aLeftNode", ty: "*const nsIDOMNode" }, Param { name: "aRightNode", ty: "*const nsIDOMNode" }, Param { name: "aParent", ty: "*const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* void DidJoinNodes (in nsIDOMNode aLeftNode, in nsIDOMNode aRightNode, in nsIDOMNode aParent, in nsresult aResult); */
                    Method {
                        name: "DidJoinNodes",
                        abi: "C",
                        params: &[Param { name: "aLeftNode", ty: "*const nsIDOMNode" }, Param { name: "aRightNode", ty: "*const nsIDOMNode" }, Param { name: "aParent", ty: "*const nsIDOMNode" }, Param { name: "aResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void WillInsertText (in nsIDOMCharacterData aTextNode, in long aOffset, in DOMString aString); */
                    Method {
                        name: "WillInsertText",
                        abi: "C",
                        params: &[Param { name: "aTextNode", ty: "*const nsIDOMCharacterData" }, Param { name: "aOffset", ty: "libc::int32_t" }, Param { name: "aString", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void DidInsertText (in nsIDOMCharacterData aTextNode, in long aOffset, in DOMString aString, in nsresult aResult); */
                    Method {
                        name: "DidInsertText",
                        abi: "C",
                        params: &[Param { name: "aTextNode", ty: "*const nsIDOMCharacterData" }, Param { name: "aOffset", ty: "libc::int32_t" }, Param { name: "aString", ty: "*const nsAString" }, Param { name: "aResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void WillDeleteText (in nsIDOMCharacterData aTextNode, in long aOffset, in long aLength); */
                    Method {
                        name: "WillDeleteText",
                        abi: "C",
                        params: &[Param { name: "aTextNode", ty: "*const nsIDOMCharacterData" }, Param { name: "aOffset", ty: "libc::int32_t" }, Param { name: "aLength", ty: "libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* void DidDeleteText (in nsIDOMCharacterData aTextNode, in long aOffset, in long aLength, in nsresult aResult); */
                    Method {
                        name: "DidDeleteText",
                        abi: "C",
                        params: &[Param { name: "aTextNode", ty: "*const nsIDOMCharacterData" }, Param { name: "aOffset", ty: "libc::int32_t" }, Param { name: "aLength", ty: "libc::int32_t" }, Param { name: "aResult", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    /* void WillDeleteSelection (in nsISelection aSelection); */
                    Method {
                        name: "WillDeleteSelection",
                        abi: "C",
                        params: &[Param { name: "aSelection", ty: "*const nsISelection" }],
                        ret: "nsresult",
                    },

                    /* void DidDeleteSelection (in nsISelection aSelection); */
                    Method {
                        name: "DidDeleteSelection",
                        abi: "C",
                        params: &[Param { name: "aSelection", ty: "*const nsISelection" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


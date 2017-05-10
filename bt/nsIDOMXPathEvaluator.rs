//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXPathEvaluator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXPathEvaluator",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISupports evaluate (in DOMString expression, in nsIDOMNode contextNode, in nsIDOMNode resolver, in unsigned short type, in nsISupports result) raises (XPathException,DOMException); */
                    Method {
                        name: "evaluate",
                        abi: "C",
                        params: &[Param { name: "expression", ty: "*const nsAString" }, Param { name: "contextNode", ty: "*const nsIDOMNode" }, Param { name: "resolver", ty: "*const nsIDOMNode" }, Param { name: "type_", ty: "libc::uint16_t" }, Param { name: "result", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


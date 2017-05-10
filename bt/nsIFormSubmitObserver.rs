//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFormSubmitObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFormSubmitObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void notify (in nsIDOMHTMLFormElement formNode, in mozIDOMWindow window, in nsIURI actionURL, out boolean cancelSubmit); */
                    Method {
                        name: "notify",
                        abi: "C",
                        params: &[Param { name: "formNode", ty: "*const nsIDOMHTMLFormElement" }, Param { name: "window", ty: "*const mozIDOMWindow" }, Param { name: "actionURL", ty: "*const nsIURI" }, Param { name: "cancelSubmit", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void notifyInvalidSubmit (in nsIDOMHTMLFormElement formNode, in nsIArray invalidElements); */
                    Method {
                        name: "notifyInvalidSubmit",
                        abi: "C",
                        params: &[Param { name: "formNode", ty: "*const nsIDOMHTMLFormElement" }, Param { name: "invalidElements", ty: "*const nsIArray" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


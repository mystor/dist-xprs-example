//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationServiceCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void notifySuccess (in DOMString url); */
                    Method {
                        name: "notifySuccess",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void notifyError (in nsresult error); */
                    Method {
                        name: "notifyError",
                        abi: "C",
                        params: &[Param { name: "error", ty: "nsresult" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIPresentationService",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationRequestUIGlue.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationRequestUIGlue",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* nsISupports sendRequest (in DOMString url, in DOMString sessionId, in nsIPresentationDevice device); */
                    Method {
                        name: "sendRequest",
                        abi: "C",
                        params: &[Param { name: "url", ty: "*const nsAString" }, Param { name: "sessionId", ty: "*const nsAString" }, Param { name: "device", ty: "*const nsIPresentationDevice" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


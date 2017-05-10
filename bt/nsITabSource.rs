//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITabSource.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITabSource",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* mozIDOMWindowProxy getTabToStream (); */
                    Method {
                        name: "getTabToStream",
                        abi: "C",
                        params: &[Param { name: "_retval", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* void notifyStreamStart (in mozIDOMWindowProxy window); */
                    Method {
                        name: "notifyStreamStart",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    /* void notifyStreamStop (in mozIDOMWindowProxy window); */
                    Method {
                        name: "notifyStreamStop",
                        abi: "C",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


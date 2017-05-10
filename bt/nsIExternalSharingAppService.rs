//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIExternalSharingAppService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISharingHandlerApp",
            base: Some("nsIHandlerApp"),
            methods: Some(&[
                    /* void share (in AString data, [optional] in AString title); */
                    Method {
                        name: "share",
                        abi: "C",
                        params: &[Param { name: "data", ty: "*const nsAString" }, Param { name: "title", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIExternalSharingAppService",
            base: Some("nsISupports"),
            methods: None,
        },


        ]; D}


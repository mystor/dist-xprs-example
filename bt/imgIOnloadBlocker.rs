//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgIOnloadBlocker.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "imgIOnloadBlocker",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void blockOnload (in imgIRequest aRequest); */
                    Method {
                        name: "blockOnload",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "nsresult",
                    },

                    /* void unblockOnload (in imgIRequest aRequest); */
                    Method {
                        name: "unblockOnload",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


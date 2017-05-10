//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIColorAnalyzer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIRepresentativeColorCallback",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onComplete (in boolean success, [optional] in unsigned long color); */
                    Method {
                        name: "onComplete",
                        abi: "C",
                        params: &[Param { name: "success", ty: "bool" }, Param { name: "color", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "mozIColorAnalyzer",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void findRepresentativeColor (in nsIURI imageURI, in mozIRepresentativeColorCallback callback); */
                    Method {
                        name: "findRepresentativeColor",
                        abi: "C",
                        params: &[Param { name: "imageURI", ty: "*const nsIURI" }, Param { name: "callback", ty: "*const mozIRepresentativeColorCallback" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


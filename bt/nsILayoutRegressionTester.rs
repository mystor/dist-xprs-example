//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILayoutRegressionTester.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILayoutRegressionTester",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* long dumpFrameModel (in mozIDOMWindow aWindowToDump, in nsIFile aFile, in unsigned long aFlagsMask); */
                    Method {
                        name: "dumpFrameModel",
                        abi: "C",
                        params: &[Param { name: "aWindowToDump", ty: "*const mozIDOMWindow" }, Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "aFlagsMask", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut libc::int32_t" }],
                        ret: "nsresult",
                    },

                    /* boolean compareFrameModels (in nsIFile aBaseFile, in nsIFile aVerFile, in unsigned long aFlags); */
                    Method {
                        name: "compareFrameModels",
                        abi: "C",
                        params: &[Param { name: "aBaseFile", ty: "*const nsIFile" }, Param { name: "aVerFile", ty: "*const nsIFile" }, Param { name: "aFlags", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


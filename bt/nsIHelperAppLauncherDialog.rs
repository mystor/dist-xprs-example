//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHelperAppLauncherDialog.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHelperAppLauncherDialog",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void show (in nsIHelperAppLauncher aLauncher, in nsISupports aWindowContext, in unsigned long aReason); */
                    Method {
                        name: "show",
                        abi: "C",
                        params: &[Param { name: "aLauncher", ty: "*const nsIHelperAppLauncher" }, Param { name: "aWindowContext", ty: "*const nsISupports" }, Param { name: "aReason", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    /* void promptForSaveToFileAsync (in nsIHelperAppLauncher aLauncher, in nsISupports aWindowContext, in wstring aDefaultFileName, in wstring aSuggestedFileExtension, in boolean aForcePrompt); */
                    Method {
                        name: "promptForSaveToFileAsync",
                        abi: "C",
                        params: &[Param { name: "aLauncher", ty: "*const nsIHelperAppLauncher" }, Param { name: "aWindowContext", ty: "*const nsISupports" }, Param { name: "aDefaultFileName", ty: "*const libc::int16_t" }, Param { name: "aSuggestedFileExtension", ty: "*const libc::int16_t" }, Param { name: "aForcePrompt", ty: "bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


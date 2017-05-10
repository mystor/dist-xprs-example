//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPackageKitService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPackageKitService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void installPackages (in unsigned long packageKitMethod, in nsIArray packageArray, in nsIObserver observer); */
                    Method {
                        name: "installPackages",
                        abi: "C",
                        params: &[Param { name: "packageKitMethod", ty: "libc::uint32_t" }, Param { name: "packageArray", ty: "*const nsIArray" }, Param { name: "observer", ty: "*const nsIObserver" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


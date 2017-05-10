//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWindowProvider",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* mozIDOMWindowProxy provideWindow (in mozIDOMWindowProxy aParent, in unsigned long aChromeFlags, in boolean aCalledFromJS, in boolean aPositionSpecified, in boolean aSizeSpecified, in nsIURI aURI, in AString aName, in AUTF8String aFeatures, in boolean aForceNoOpener, out boolean aWindowIsNew); */
                    Method {
                        name: "provideWindow",
                        abi: "C",
                        params: &[Param { name: "aParent", ty: "*const mozIDOMWindowProxy" }, Param { name: "aChromeFlags", ty: "libc::uint32_t" }, Param { name: "aCalledFromJS", ty: "bool" }, Param { name: "aPositionSpecified", ty: "bool" }, Param { name: "aSizeSpecified", ty: "bool" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aName", ty: "*const nsAString" }, Param { name: "aFeatures", ty: "*const nsACString" }, Param { name: "aForceNoOpener", ty: "bool" }, Param { name: "aWindowIsNew", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut *const mozIDOMWindowProxy" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


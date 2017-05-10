//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGfxInfoDebug.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIGfxInfoDebug",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void spoofVendorID (in DOMString aVendorID); */
                    Method {
                        name: "spoofVendorID",
                        abi: "C",
                        params: &[Param { name: "aVendorID", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void spoofDeviceID (in DOMString aDeviceID); */
                    Method {
                        name: "spoofDeviceID",
                        abi: "C",
                        params: &[Param { name: "aDeviceID", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void spoofDriverVersion (in DOMString aDriverVersion); */
                    Method {
                        name: "spoofDriverVersion",
                        abi: "C",
                        params: &[Param { name: "aDriverVersion", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void spoofOSVersion (in unsigned long aVersion); */
                    Method {
                        name: "spoofOSVersion",
                        abi: "C",
                        params: &[Param { name: "aVersion", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/IPeerConnection.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "IPeerConnectionManager",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* boolean hasActivePeerConnection (in unsigned long innerWindowID); */
                    Method {
                        name: "hasActivePeerConnection",
                        abi: "C",
                        params: &[Param { name: "innerWindowID", ty: "libc::uint32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "IPeerConnectionObserver",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        Interface {
            name: "IPeerConnection",
            base: Some("nsISupports"),
            methods: Some(&[
                    ]),
        },


        ]; D}


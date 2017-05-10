//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMediaManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMediaManagerService",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIArray activeMediaCaptureWindows; */
                    Method {
                        name: "get_activeMediaCaptureWindows",
                        abi: "C",
                        params: &[Param { name: "aActiveMediaCaptureWindows", ty: "*mut *const nsIArray" }],
                        ret: "nsresult",
                    },

                    /* void mediaCaptureWindowState (in nsIDOMWindow aWindow, out boolean aVideo, out boolean aAudio, [optional] out boolean aScreenShare, [optional] out boolean aWindowShare, [optional] out boolean aAppShare, [optional] out boolean aBrowserShare); */
                    Method {
                        name: "mediaCaptureWindowState",
                        abi: "C",
                        params: &[Param { name: "aWindow", ty: "*const nsIDOMWindow" }, Param { name: "aVideo", ty: "*mut bool" }, Param { name: "aAudio", ty: "*mut bool" }, Param { name: "aScreenShare", ty: "*mut bool" }, Param { name: "aWindowShare", ty: "*mut bool" }, Param { name: "aAppShare", ty: "*mut bool" }, Param { name: "aBrowserShare", ty: "*mut bool" }],
                        ret: "nsresult",
                    },

                    /* void sanitizeDeviceIds (in long long sinceWhen); */
                    Method {
                        name: "sanitizeDeviceIds",
                        abi: "C",
                        params: &[Param { name: "sinceWhen", ty: "libc::int64_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


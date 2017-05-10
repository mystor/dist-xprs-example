//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamListener",
            base: Some("nsIRequestObserver"),
            methods: Some(&[
                    /* void onDataAvailable (in nsIRequest aRequest, in nsISupports aContext, in nsIInputStream aInputStream, in unsigned long long aOffset, in unsigned long aCount); */
                    Method {
                        name: "onDataAvailable",
                        abi: "C",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContext", ty: "*const nsISupports" }, Param { name: "aInputStream", ty: "*const nsIInputStream" }, Param { name: "aOffset", ty: "libc::uint64_t" }, Param { name: "aCount", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


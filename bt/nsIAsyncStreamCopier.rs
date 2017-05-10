//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAsyncStreamCopier.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAsyncStreamCopier",
            base: Some("nsIRequest"),
            methods: Some(&[
                    /* void init (in nsIInputStream aSource, in nsIOutputStream aSink, in nsIEventTarget aTarget, in boolean aSourceBuffered, in boolean aSinkBuffered, in unsigned long aChunkSize, in boolean aCloseSource, in boolean aCloseSink); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[Param { name: "aSource", ty: "*const nsIInputStream" }, Param { name: "aSink", ty: "*const nsIOutputStream" }, Param { name: "aTarget", ty: "*const nsIEventTarget" }, Param { name: "aSourceBuffered", ty: "bool" }, Param { name: "aSinkBuffered", ty: "bool" }, Param { name: "aChunkSize", ty: "libc::uint32_t" }, Param { name: "aCloseSource", ty: "bool" }, Param { name: "aCloseSink", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void asyncCopy (in nsIRequestObserver aObserver, in nsISupports aObserverContext); */
                    Method {
                        name: "asyncCopy",
                        abi: "C",
                        params: &[Param { name: "aObserver", ty: "*const nsIRequestObserver" }, Param { name: "aObserverContext", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


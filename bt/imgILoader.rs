//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgILoader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "imgILoader",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* imgIRequest loadImageXPCOM (in nsIURI aURI, in nsIURI aInitialDocumentURL, in nsIURI aReferrerURI, in AString aReferrerPolicy, in nsIPrincipal aLoadingPrincipal, in nsILoadGroup aLoadGroup, in imgINotificationObserver aObserver, in nsISupports aCX, in nsLoadFlags aLoadFlags, in nsISupports cacheKey, [optional] in nsContentPolicyType aContentPolicyType); */
                    Method {
                        name: "loadImageXPCOM",
                        abi: "C",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aInitialDocumentURL", ty: "*const nsIURI" }, Param { name: "aReferrerURI", ty: "*const nsIURI" }, Param { name: "aReferrerPolicy", ty: "*const nsAString" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aLoadGroup", ty: "*const nsILoadGroup" }, Param { name: "aObserver", ty: "*const imgINotificationObserver" }, Param { name: "aCX", ty: "*const nsISupports" }, Param { name: "aLoadFlags", ty: "nsLoadFlags" }, Param { name: "cacheKey", ty: "*const nsISupports" }, Param { name: "aContentPolicyType", ty: "nsContentPolicyType" }, Param { name: "_retval", ty: "*mut *const imgIRequest" }],
                        ret: "nsresult",
                    },

                    /* imgIRequest loadImageWithChannelXPCOM (in nsIChannel aChannel, in imgINotificationObserver aObserver, in nsISupports cx, out nsIStreamListener aListener); */
                    Method {
                        name: "loadImageWithChannelXPCOM",
                        abi: "C",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aObserver", ty: "*const imgINotificationObserver" }, Param { name: "cx", ty: "*const nsISupports" }, Param { name: "aListener", ty: "*mut *const nsIStreamListener" }, Param { name: "_retval", ty: "*mut *const imgIRequest" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


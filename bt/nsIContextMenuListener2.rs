//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContextMenuListener2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContextMenuListener2",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void onShowContextMenu (in unsigned long aContextFlags, in nsIContextMenuInfo aUtils); */
                    Method {
                        name: "onShowContextMenu",
                        abi: "C",
                        params: &[Param { name: "aContextFlags", ty: "libc::uint32_t" }, Param { name: "aUtils", ty: "*const nsIContextMenuInfo" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        Interface {
            name: "nsIContextMenuInfo",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* readonly attribute nsIDOMEvent mouseEvent; */
                    Method {
                        name: "get_mouseEvent",
                        abi: "C",
                        params: &[Param { name: "aMouseEvent", ty: "*mut *const nsIDOMEvent" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIDOMNode targetNode; */
                    Method {
                        name: "get_targetNode",
                        abi: "C",
                        params: &[Param { name: "aTargetNode", ty: "*mut *const nsIDOMNode" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute AString associatedLink; */
                    Method {
                        name: "get_associatedLink",
                        abi: "C",
                        params: &[Param { name: "aAssociatedLink", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute imgIContainer imageContainer; */
                    Method {
                        name: "get_imageContainer",
                        abi: "C",
                        params: &[Param { name: "aImageContainer", ty: "*mut *const imgIContainer" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI imageSrc; */
                    Method {
                        name: "get_imageSrc",
                        abi: "C",
                        params: &[Param { name: "aImageSrc", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute imgIContainer backgroundImageContainer; */
                    Method {
                        name: "get_backgroundImageContainer",
                        abi: "C",
                        params: &[Param { name: "aBackgroundImageContainer", ty: "*mut *const imgIContainer" }],
                        ret: "nsresult",
                    },

                    /* readonly attribute nsIURI backgroundImageSrc; */
                    Method {
                        name: "get_backgroundImageSrc",
                        abi: "C",
                        params: &[Param { name: "aBackgroundImageSrc", ty: "*mut *const nsIURI" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


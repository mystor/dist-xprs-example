//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULPopupElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULPopupElement",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* attribute DOMString position; */
                    Method {
                        name: "get_position",
                        abi: "C",
                        params: &[Param { name: "aPosition", ty: "*mut nsAString" }],
                        ret: "nsresult",
                    },
                    Method {
                        name: "set_position",
                        abi: "C",
                        params: &[Param { name: "aPosition", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void showPopup (in unsigned short alignment, in nsIDOMElement target, in nsIDOMElement anchor); */
                    Method {
                        name: "showPopup",
                        abi: "C",
                        params: &[Param { name: "alignment", ty: "libc::uint16_t" }, Param { name: "target", ty: "*const nsIDOMElement" }, Param { name: "anchor", ty: "*const nsIDOMElement" }],
                        ret: "nsresult",
                    },

                    /* void hidePopup (); */
                    Method {
                        name: "hidePopup",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


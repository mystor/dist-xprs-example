//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISound.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISound",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void play (in nsIURL aURL); */
                    Method {
                        name: "play",
                        abi: "C",
                        params: &[Param { name: "aURL", ty: "*const nsIURL" }],
                        ret: "nsresult",
                    },

                    /* void playSystemSound (in AString soundAlias); */
                    Method {
                        name: "playSystemSound",
                        abi: "C",
                        params: &[Param { name: "soundAlias", ty: "*const nsAString" }],
                        ret: "nsresult",
                    },

                    /* void beep (); */
                    Method {
                        name: "beep",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void init (); */
                    Method {
                        name: "init",
                        abi: "C",
                        params: &[],
                        ret: "nsresult",
                    },

                    /* void playEventSound (in unsigned long aEventId); */
                    Method {
                        name: "playEventSound",
                        abi: "C",
                        params: &[Param { name: "aEventId", ty: "libc::uint32_t" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


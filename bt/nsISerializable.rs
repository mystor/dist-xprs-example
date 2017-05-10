//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISerializable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISerializable",
            base: Some("nsISupports"),
            methods: Some(&[
                    /* void read (in nsIObjectInputStream aInputStream); */
                    Method {
                        name: "read",
                        abi: "C",
                        params: &[Param { name: "aInputStream", ty: "*const nsIObjectInputStream" }],
                        ret: "nsresult",
                    },

                    /* void write (in nsIObjectOutputStream aOutputStream); */
                    Method {
                        name: "write",
                        abi: "C",
                        params: &[Param { name: "aOutputStream", ty: "*const nsIObjectOutputStream" }],
                        ret: "nsresult",
                    },

                    ]),
        },


        ]; D}


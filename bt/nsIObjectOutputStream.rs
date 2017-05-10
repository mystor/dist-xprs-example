//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIObjectOutputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIObjectOutputStream",
            base: Some("nsIBinaryOutputStream"),
            methods: Some(&[
                    /* void writeObject (in nsISupports aObject, in boolean aIsStrongRef); */
                    Method {
                        name: "writeObject",
                        abi: "C",
                        params: &[Param { name: "aObject", ty: "*const nsISupports" }, Param { name: "aIsStrongRef", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void writeSingleRefObject (in nsISupports aObject); */
                    Method {
                        name: "writeSingleRefObject",
                        abi: "C",
                        params: &[Param { name: "aObject", ty: "*const nsISupports" }],
                        ret: "nsresult",
                    },

                    /* void writeCompoundObject (in nsISupports aObject, in nsIIDRef aIID, in boolean aIsStrongRef); */
                    Method {
                        name: "writeCompoundObject",
                        abi: "C",
                        params: &[Param { name: "aObject", ty: "*const nsISupports" }, Param { name: "aIID", ty: "*const nsIID" }, Param { name: "aIsStrongRef", ty: "bool" }],
                        ret: "nsresult",
                    },

                    /* void writeID (in nsIDRef aID); */
                    Method {
                        name: "writeID",
                        abi: "C",
                        params: &[Param { name: "aID", ty: "*const nsID" }],
                        ret: "nsresult",
                    },

                    /* [notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask); */
                    Method {
                        name: "getBuffer",
                        abi: "C",
                        params: &[Param { name: "aLength", ty: "uint32_t" }, Param { name: "aAlignMask", ty: "uint32_t" }],
                        ret: "*const u8",
                    },

                    /* [notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength); */
                    Method {
                        name: "putBuffer",
                        abi: "C",
                        params: &[Param { name: "aBuffer", ty: "*const u8" }, Param { name: "aLength", ty: "uint32_t" }],
                        ret: "libc::c_void",
                    },

                    ]),
        },


        ]; D}


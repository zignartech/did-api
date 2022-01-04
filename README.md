## IOTA Identity API Tutorial

Youtube: [IOTA - Decentralized Identities API - Explained](https://www.youtube.com/watch?v=mY0If3JZmhc&t)

### Problem Description
In this tutorial you will utilize the [Rust Account of the IOTA Identity framework](https://github.com/iotaledger/identity.rs/tree/dev/examples/account) to solve the problem described below. [identity.rs](https://github.com/iotaledger/identity.rs/blob/dev/README.md):
> Alice recently graduated from the University of Oslo with a Bachelor of Computer Science. Now she wants to apply for a remote job at the IOTA Foundation and needs to digitally prove the existence and validity of her degree. What she needs is an immutable and verifiable credential, which has been approved by both the University of Oslo and herself, before presenting it to her possible new employer.

### Roles
As described [here](https://www.iota.org/solutions/digital-identity), IOTA Identity builds on the W3C's proposed standards for a digital identity framework and thus is based on three roles:
- Holder (Alice)
- Issuer (University of Oslo)
- Verifier (IOTA Foundation)

### Flow-Chart
![banner](./identity_tutorial_chart.png)


### Steps
In this process, you will complete the different steps from the perspective of one of the mentioned roles above:

1. **Holder:** Create a DID (Decentralized Identifier) document for Alice.
     - **POST** => http://localhost:6000/create_did
    ```json
    {
        "nick_name": "Alice",
        "password": "my_password"
    }
    ```
    **Response**
    ```json
    {
        "Error": false,
       "Explorer": "https://explorer.iota.org/mainnet/identity-resolver/did:iota:6jAqUhoszQ79fWfx87Ga7o4TxkLcK6uBTm7LE2k26dAR",
        "Status": "Did Created",
        "did": "did:iota:6jAqUhoszQ79fWfx87Ga7o4TxkLcK6uBTm7LE2k26dAR"
    }
    ```

2. **Issuer:** Create a DID document for the University of Oslo.
    - **POST** => http://localhost:6000/create_did
    ```json
    {
        "nick_name": "University of Oslo",
        "password": "my_password"
    }
    ```
    
    **Response**
    ```json
    {
        "Error": false,
       "Explorer": "https://explorer.iota.org/mainnet/identity-resolver/did:iota:4wN7q5NZKwCnkFtcv6VM42EQR9vC3DqxsLXB6pkUR1wV",
        "Status": "Did Created",
        "did": "did:iota:4wN7q5NZKwCnkFtcv6VM42EQR9vC3DqxsLXB6pkUR1wV"
    }
    ```

3. **Issuer:** Add a verification method "degreeVerifications" to the University's DID document with the purpose to verify Alice's degree. Since it's expected, that the University will have to sign more than just Alice's degree.
    - **POST** => http://localhost:6000/add_verif_method
    ```json
    {
        "nick_name": "University of Oslo",
        "password": "my_password",
        "did": "did:iota:4wN7q5NZKwCnkFtcv6VM42EQR9vC3DqxsLXB6pkUR1wV"
    }
    ```
    **Response**
    ```json
    {
        "Error": false,
        "Explorer": "https://explorer.iota.org/mainnet/identity-resolver/did:iota:4wN7q5NZKwCnkFtcv6VM42EQR9vC3DqxsLXB6pkUR1wV",
        "Status": "Method Created",
        "vm_name": "XMCOV9LC"
    }
    ```

4. **Holder:** Add a verification method to Alice's DID document with the purpose to present her degree to a third party.
    - **POST** => http://localhost:6000/add_verif_method
    ```json
    {
        "nick_name": "Alice",
        "password": "my_password",
        "did": "did:iota:6jAqUhoszQ79fWfx87Ga7o4TxkLcK6uBTm7LE2k26dAR"
    }
    ```
    **Response**
    ```json
    {
        "Error": false,
        "Explorer": "https://explorer.iota.org/mainnet/identity-resolver/did:iota:6jAqUhoszQ79fWfx87Ga7o4TxkLcK6uBTm7LE2k26dAR",
        "Status": "Method Created",
        "vm_name": "MZXQMFPP"
    }
    ```


5. **Holder:** Setup a document representing Alice's degree, containing her DID.
    ```json
    "holder": {
        "id": "did:iota:6jAqUhoszQ79fWfx87Ga7o4TxkLcK6uBTm7LE2k26dAR",
        "name": "Alice",
        "degreeName": "Bachelor of Computer Science",
        "degreeType": "BachelorDegree",
        "GPA": "4.0"
    }
    ```

6. **Issuer:** Sign the degree document with the University's verification method to obtain a verifiable credential.
    - **POST** => http://localhost:6000/create_vc
    ```json
    {
        "issuer": {
            "nick_name": "University of Oslo",
            "password": "my_password",
            "did": "did:iota:4wN7q5NZKwCnkFtcv6VM42EQR9vC3DqxsLXB6pkUR1wV",
            "vm_name": "XMCOV9LC"
        },
        "holder": {
            "id": "did:iota:6jAqUhoszQ79fWfx87Ga7o4TxkLcK6uBTm7LE2k26dAR",
            "name": "Alice",
            "degreeName": "Bachelor of Computer Science",
            "degreeType": "BachelorDegree",
            "GPA": "4.0"
        }
    }
    ```
    **Response**
    ```json
    {
        "Credential": {
            "@context": "https://www.w3.org/2018/credentials/v1",
            "credentialSubject": {
                "GPA": "4.0",
                    ...
                }
                ...
            },
        "Error": false,
        "Status": "Credential Created",
        "Verified": true
    }
    ```

7. **Holder:** Alice verifies the credentials to make sure it was actually signed by a key associated to the University DID.
    - **POST** => http://localhost:6000/verify_validity_credential

    ```json
    {
        "@context": "https://www.w3.org/2018/credentials/v1",
        "credentialSubject": {
            "GPA": "4.0",
            ...
        }
        ...
    }
    ```
    **Response**
    ```json
    {
       "error": false,
        "validation": true
    }
    ```

8. **Holder:** Alice signs verifiable credential with private key of Alices's verification method in order to get a verifiable presentation.
    - **POST** => http://localhost:6000/create_vp
    ```json
    {
        "holder": {
            "nick_name": "Alice",
            "password": "my_password",
            "did": "did:iota:6jAqUhoszQ79fWfx87Ga7o4TxkLcK6uBTm7LE2k26dAR",
            "vm_name": "MZXQMFPP"
        },
        "holder_credential": {
            "@context": "https://www.w3.org/2018/credentials/v1",
            "credentialSubject": {
                "GPA": "4.0",
                ...
            }
            ...
        }
    }
    ```
    **Response**
    ```json
    {
        "Presentation": {
            "@context": "https://www.w3.org/2018/credentials/v1",
            "holder": "did:iota:6jAqUhoszQ79fWfx87Ga7o4TxkLcK6uBTm7LE2k26dAR",
            "proof": {
                "type": "JcsEd25519Signature2020",
                    ...
                }
                ...
            },
        "Error": false,
        "Status": "Presentation Created",
        "Verified": true
    }
    ```

9. **Verifier:** The IOTA Foundation verfies Alice's and the University's signatures with their respective public keys by checking the verifiable presentation.
    - **POST** => http://localhost:6000/verify_validity_presentation

    ```json
    {
        "@context": "https://www.w3.org/2018/credentials/v1",
        "holder": "did:iota:6jAqUhoszQ79fWfx87Ga7o4TxkLcK6uBTm7LE2k26dAR",
            "proof": {
                "type": "JcsEd25519Signature2020",
                    ...
            }
        ...
    }
    ```
    **Response**
    ```json
    {
       "error": false,
        "validation": true
    }
    ```

10. **Issuer:** Unfortunately, the University found out that Alice was cheating on her final exam. Therefore, the University revokes the verification of Alice's credential. Removing the verification method. Note that Alice could also revoke her signature on the verifiable presentation, removing the verification method from her.
    - **POST** => http://localhost:6000/remove_vm
    
    ```json
    {
        "issuer": {
            "nick_name": "University of Oslo",
            "password": "my_password",
            "did": "did:iota:4wN7q5NZKwCnkFtcv6VM42EQR9vC3DqxsLXB6pkUR1wV",
            "vm_name": "XMCOV9LC"
        }
    }
    ```
    **Response**
    ```json
    {
        "Status": "Removed VM",
        "error": false
    }
    ```

11. **Verifier:** The IOTA Foundation verifies Alice's and the University's signatures again by checking the verifiable presentation and finds out that the University revoked their signature.
    - **POST** => http://localhost:6000/verify_validity_presentation

    ```json
    {
        "@context": "https://www.w3.org/2018/credentials/v1",
        "holder": "did:iota:6jAqUhoszQ79fWfx87Ga7o4TxkLcK6uBTm7LE2k26dAR",
            "proof": {
                "type": "JcsEd25519Signature2020",
                    ...
            }
        ...
    }
    ```
    **Response**
    ```json
    {
        "Error": true,
        "validation": false
    }
    ```
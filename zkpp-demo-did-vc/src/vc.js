const { JCS } = require('jcs')
const SMT = require('circomlibjs').newMemEmptyTrie
const ffjavascript = require('ffjavascript')
const { newMemEmptyTrie } = require('circomlibjs')

const buildEddsa = require("circomlibjs").buildEddsa;
const buildBabyjub = require("circomlibjs").buildBabyjub;
JSON.canonify = JCS.cannonicalize
const verifableCredential = {
    "@context": [
        "https://www.w3.org/2018/credentials/v1",
        "https://www.w3.org/2018/credentials/examples/v1"
    ],
    "id": "http://example.edu/credentials/3732",
    "type": ["VerifiableCredential", "KYC"],
    "issuer": "did:example:123456789abcdefghi",
    "issuanceDate": "",
    "credentialSubject": {
        "id": "did:example:123456789abcdefghi",
        "name": "Jayden Doe",
        "age": 18n,
        "country": "Singapore",




    },


}



const generateCredential = async (issuerID, credentialSubject) => {
    verifableCredential.issuanceDate = Number(new Date())
    verifableCredential.issuer = issuerID
    verifableCredential.credentialSubject = credentialSubject
    return JSON.parse(JCS.cannonicalize(verifableCredential))


}

const generateTree = async (credential) => {
    const F=(await buildBabyjub()).F

    const credentialSubject = credential.credentialSubject
    

    Object.keys(credentialSubject).forEach((key) => {
        if (key == "id") {
            
            credentialSubject[key] =   credentialSubject[key]
        } else {
            if (typeof (credentialSubject[key]) == 'number') {
                credentialSubject[key] = BigInt(credentialSubject[key])

            } else {
                credentialSubject[key] = '0x' + Buffer.from(credentialSubject[key]).toString('hex')
            }
        }




    })




    const tree = await newMemEmptyTrie()
    const arr = Object.values(credentialSubject)

    for (let i = 0; i < arr.length; i++) {
        await tree.insert(i, arr[i])
    }


    return { rootHash: tree.F.toObject(tree.root), credentialSubject, smt: tree }
}


const issueCredential = async (credentialHash, privateKey) => {
    const babyJub=   await buildBabyjub()
    const eddsa = await buildEddsa()
    const F = babyJub.F;
    const pvk=Buffer.from(privateKey,'hex')
const msg=F.e(credentialHash,10)
    const signature = eddsa.signPoseidon(pvk, msg)
    const Proof={
        R8x:F.toObject(signature.R8[0]),
        R8y:F.toObject(signature.R8[1]),
        S:signature.S
    }
    return Proof
}


module.exports = {
    generateCredential, generateTree,issueCredential
}
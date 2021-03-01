use std::{fs::{self, File}, io::Write};

use pgp::{Deserializable, Message, SignedPublicKey, SignedSecretKey, composed::key::*, types::SecretKeyTrait};

#[derive(Clone)]
pub(crate) struct KeyPair {
    public_key: SignedPublicKey, 
    secret_key: SignedSecretKey
}

impl KeyPair {
    /// This method generates a new pgp secret and public key with a boltchat.net identity based on the nickname.
    pub(crate) fn new(nick: String, password: String) -> Self {
        let mut secret_params = SecretKeyParamsBuilder::default();
        secret_params
            .primary_user_id(format!("{} (generated by bolt) <identities@boltchat.net>", nick))
            .key_type(KeyType::Rsa(2048));
        
        let secret_key_unsigned = secret_params.build().unwrap().generate().unwrap();
        let secret_key = secret_key_unsigned.sign(|| password.clone()).unwrap();
        let public_key = secret_key.public_key().sign(&secret_key, || password).unwrap();

        KeyPair {
            public_key, 
            secret_key
        }
    }

    /// This method is used to load keys from the path passed in.
    pub(crate) fn load_keys(path: &String, password: String) -> Self {
        let contents = fs::read_to_string(path).unwrap();

        let secret_key = SignedSecretKey::from_string(&contents).unwrap().0;
        let public_key = secret_key.public_key().sign(&secret_key, || password).unwrap();

        KeyPair {
            public_key,
            secret_key,
        }
    }

    /// This method armors the secret key and saves it to the passed in path.
    pub(crate) fn save_secret_key(&self, path: &String) {
        let mut file = File::create(path).unwrap();
        let content = self.secret_key.to_armored_string(None).unwrap();
        file.write_all(&mut content.as_bytes()).unwrap();
    }

    /// This method armors the public key and returns it.
    pub(crate) fn armor_public_key(&self) -> String {
        self.public_key.to_armored_string(None).unwrap()
    }

    /// This method signs and armors the passed in content with the secret key.
    pub(crate) fn armor_message_signature(&self, content: String) -> String {
        let mut message = Message::new_literal_bytes("", content.as_bytes());
        message = message.sign(&self.secret_key, || "".into(), Default::default()).unwrap();

        message.into_signature().to_armored_string(None).unwrap()
    }
}
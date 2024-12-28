use crate::encryption::Encryption;

#[derive(Debug)] // Allows us to print the struct
pub struct Account{
   pub service_name: String,
   pub username: String,
   pub encrypted_password: Vec<u8>,
   pub nonce: Vec<u8>,
}

impl Account {
    pub fn new(service: String, user: String, pass: String) -> Self {
        let (encrypted_password, nonce) = Encryption::encrypt_password(&pass);

        Self {
            service_name: service,
            username: user,
            encrypted_password,
            nonce: nonce.to_vec(),
        }
    }

    // Retrieve the password...
    pub fn get_password(&self) -> String {
      Encryption::decrypt_password(&self.encrypted_password, &self.nonce)
    }
}
use std::error::Error;
use std::collections::HashMap;

use cryptoki::context::{CInitializeArgs, Pkcs11};
use cryptoki::slot::Slot;
use cryptoki::session::{UserType, Session};
use cryptoki::types::AuthPin;
use cryptoki::object::{Attribute, ObjectClass};
use cryptoki::mechanism::Mechanism;

pub struct Manager {
    pkcs: Pkcs11,
    map: HashMap<u64, Session>, 
}

impl Manager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let pkcs = Pkcs11::new("/usr/lib/softhsm/libsofthsm2.so")?;
        pkcs.initialize(CInitializeArgs::OsThreads)?;
        Ok(Self {pkcs:pkcs, map: HashMap::new()})
    }

    pub fn list(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let slots = self.pkcs.get_all_slots()?;
        let mut data: Vec<String>  = Vec::new();
        for slot in slots {
            let Ok(token) = self.pkcs.get_token_info(slot) else { continue; };
            if token.token_initialized() {
                let entry = format!("{:?}|{}", slot, token.label());
                data.push(entry.to_string());
            }
        }
        Ok(data)
    }

    pub fn session(&mut self, id: u64, pin: &str) -> Result<(), Box<dyn Error>> {
        let slot    = self.slot(id)?;
        let session = self.pkcs.open_rw_session(slot)?;

        session.login(UserType::User, Some(&AuthPin::new(pin.into())))?;

        self.map.insert(id, session);
        Ok(())
    }

    pub fn encrypt(&self, session: &Session, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let search  = vec![Attribute::Class(ObjectClass::PUBLIC_KEY)];
        let objects = session.find_objects(&search)?;
        let key     = objects.get(0).ok_or("No public key found.")?;
        let ciphertext = session.encrypt(
            &Mechanism::RsaPkcs, 
            *key, 
            data
        )?;
        Ok(ciphertext)
        }

    pub fn sign(&self, session: &Session, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let search  = vec![Attribute::Class(ObjectClass::PRIVATE_KEY)];
        let objects = session.find_objects(&search)?;
        let key     = objects.get(0).ok_or("No private key found.")?;
        let ciphertext = session.sign(
            &Mechanism::RsaPkcs, 
            *key, 
            data
        )?;

        Ok(ciphertext)
    }

    fn slot(&self, id: u64) -> Result<Slot, Box<dyn Error>> {
        let slots = self.pkcs.get_all_slots()?;
        for s in slots {
            if s.id() == id {
                return Ok(s);
            }
        }
        
        return Err(Box::from("Invalid Slot ID"));
    }
}

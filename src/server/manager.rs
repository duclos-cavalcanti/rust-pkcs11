use std::error::Error;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use cryptoki::context::{CInitializeArgs, Pkcs11};
use cryptoki::slot::Slot;
use cryptoki::session::{UserType};
use cryptoki::types::AuthPin;
use cryptoki::object::{Attribute, ObjectClass};
use cryptoki::mechanism::Mechanism;

pub struct Manager {
    pkcs: Pkcs11,
    slots: HashMap<u64, Arc<Mutex<Slot>>>,
}

impl Manager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let pkcs = Pkcs11::new("/usr/lib/softhsm/libsofthsm2.so")?;
        pkcs.initialize(CInitializeArgs::OsThreads)?;

        let mut map = HashMap::new();
        for slot in pkcs.get_all_slots()? {
            map.insert(slot.id(), Arc::new(Mutex::new(slot)));
        }

        Ok(Self { pkcs, slots: map })
    }

    pub fn list(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut data = Vec::new();
        for (id, slot) in &self.slots {
            let slot = slot.lock().unwrap();
            let Ok(token) = self.pkcs.get_token_info(*slot) else { continue; };
            if token.token_initialized() {
                let entry = format!("SlotID:{}, Token: {}", id, token.label());
                data.push(entry);
            }
        }
        Ok(data)
    }

    pub fn encrypt(&self, id: u64, pin: &str, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let slot = self.slot(id)?;
        let slot = slot.lock().unwrap();
        let session = self.pkcs.open_rw_session(*slot)?;

        session.login(UserType::User, Some(&AuthPin::new(pin.into())))?;

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

    pub fn decrypt(&self, id: u64, pin: &str, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let slot = self.slot(id)?;
        let slot = slot.lock().unwrap();
        let session = self.pkcs.open_rw_session(*slot)?;

        session.login(UserType::User, Some(&AuthPin::new(pin.into())))?;

        let search  = vec![Attribute::Class(ObjectClass::PUBLIC_KEY)];
        let objects = session.find_objects(&search)?;
        let key     = objects.get(0).ok_or("No public key found.")?;
        let ciphertext = session.decrypt(
            &Mechanism::RsaPkcs, 
            *key, 
            data
        )?;
        Ok(ciphertext)
    }

    pub fn sign(&self, id: u64, pin: &str, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let slot = self.slot(id)?;
        let slot = slot.lock().unwrap();
        let session = self.pkcs.open_rw_session(*slot)?;

        session.login(UserType::User, Some(&AuthPin::new(pin.into())))?;

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

    fn slot(&self, id: u64) -> Result<Arc<Mutex<Slot>>, Box<dyn Error>> {
        match self.slots.get(&id) {
            Some(slot) => Ok(Arc::clone(slot)),
            None => Err(Box::from(format!("Slot ID {} does not exist", id))),
        }
    }
}

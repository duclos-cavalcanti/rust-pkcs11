use std::error::Error;

use cryptoki::context::{CInitializeArgs, Pkcs11};
use cryptoki::slot::Slot;
use cryptoki::slot::TokenInfo;
use cryptoki::session::{UserType, Session};
use cryptoki::types::AuthPin;
use cryptoki::object::{Attribute, ObjectClass, ObjectHandle};
use cryptoki::mechanism::Mechanism;

struct Pkcs { 
    pkcs: Pkcs11,
}

impl Pkcs {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let pkcs = Pkcs11::new("/usr/lib/softhsm/libsofthsm2.so")?;
        pkcs.initialize(CInitializeArgs::OsThreads)?;
        Ok(Self {pkcs})
    }

    fn slots(&self) -> Result<Vec<Slot>, Box<dyn Error>> {
        let slots = self.pkcs.get_all_slots()?;
        Ok(slots)
    }

    fn token(&self, slot: Slot) -> Result<TokenInfo, Box<dyn Error>> {
        let token = self.pkcs.get_token_info(slot)?;
        Ok(token)
    }

    fn session(&self, slot: Slot) -> Result<Session, Box<dyn Error>> {
        let session = self.pkcs.open_rw_session(slot)?;
        Ok(session)
    }
}

pub struct Manager {
    p: Pkcs,
}

impl Manager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let p = Pkcs::new()?;
        Ok(Self {p})
    }

    pub fn list(&self) -> Result<(), Box<dyn Error>> {
        let slots = self.p.slots()?;
        for slot in slots {
            print!("{:?}: ", slot);
            match self.p.token(slot) {
                Ok(token) => {
                    println!(
                        "\nToken: Label: {} | Initialized: {}",
                        token.label(),
                        token.token_initialized()
                    );
                }
                Err(_) => {
                    println!("\tNo Token present in this slot.");
                }
            }
        }

        Ok(())
    }

    pub fn session(&self) -> Result<(), Box<dyn Error>> {
        let slot    = self.slot(1)?;
        let session = self.login(slot, "1234")?;

        let data = "DATA".as_bytes();

        let enc = self.encrypt(&session, data)?;
        let sgn = self.sign(&session, data)?;

        println!("DATA: {:02X?}", data);
        println!("ENC: {:02X?}", enc);
        println!("SGN: {:02X?}", sgn);

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

    pub fn login(&self, slot: Slot, pin: &str) -> Result<Session, Box<dyn Error>> {
        let session = self.p.session(slot)?;
        session.login(UserType::User, Some(&AuthPin::new(pin.into())))?;

        Ok(session)
    }

    pub fn slot(&self, id: u64) -> Result<Slot, Box<dyn Error>> {
        let slots = self.p.slots().unwrap();
        for s in slots {
            if s.id() == id {
                return Ok(s);
            }
        }
        
        return Err(Box::from("Invalid Slot ID"));
    }
}

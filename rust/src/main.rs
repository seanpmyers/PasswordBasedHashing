use pbh::hash_encryption_service::HashEncryptionService;
use pbh::ihash_encryption_service::IHashEncryptionService;

fn main(){
   let mut _pbh_service: HashEncryptionService = IHashEncryptionService::new();
   let example_password : String = "testPassword123!".to_string();
   let mut hash = String::new();
   match pbh::HashEncryptionService::hash(&example_password) {
      Ok(hash_result) => { hash = hash_result},
      Err(error) => {eprintln!("error: {}", error)}
   }
   if &hash.is_empty() == &false {
      println!("Result: {}", &hash);
   }
   let different_password : String = "testPasswordDifferent321&".to_string();
   match pbh::HashEncryptionService::check(&hash, &different_password){
      Ok(_check_result) => {},
      Err(error) => {eprintln!("error: {}", error)}
   }
}
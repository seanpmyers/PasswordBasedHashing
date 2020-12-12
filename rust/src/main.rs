use pbh::hash_encryption_service::HashEncryptionService;
use pbh::ihash_encryption_service::IHashEncryptionService;

fn main(){
   let mut _pbh_service: HashEncryptionService = IHashEncryptionService::new();
   let hash = pbh::HashEncryptionService::hash("test");
   println!("{}", hash);
}
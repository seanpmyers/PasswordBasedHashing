use pbh::hash_encryption_service::HashEncryptionService;


fn main(){
   let mut hashEncryptionService: HashEncryptionService = HashEncryptionService::new();
   let hash = hashEncryptionService.Hash("test");
   println!("{}", hash);
}
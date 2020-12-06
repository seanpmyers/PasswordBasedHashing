pub (crate) trait IHashEncryptionService{  
   const SALT_SIZE: i32; 
   const KEY_SIZE: i32;
   fn new() -> Self;
   fn hash(input: &str) -> &str;
   fn check(hash: &str, input: &str) -> (bool, bool);
}
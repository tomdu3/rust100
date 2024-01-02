fn main() {
   let enigma: i32;
   // even if initialized compiler wan't execute the code if there is a possibility that the
   // variable wouldn't be used
   if true {
       enigma = 42;
   } else {
       enigma = 7;
   }
}

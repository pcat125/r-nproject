/*A Marco Polo Game 
if the name Marco is given, the program will respond with Polo.
otherwise, the program will respond with "What is your name?".


 */

 pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        return "Polo".to_string();
    } else {
        "What is your name?".to_string();
    
}
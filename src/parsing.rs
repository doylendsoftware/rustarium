pub fn split_message(mut message: String) -> (String, String) {
    /*!
    Split a message into two parts: A command, and its parameters.
    A single trailing space after the command is removed, if present.
    
    Singleton commands will return an empty string "" for the parameters.
    */
    
    match message.find(' ') {
        Some(index) => {
            let mut parameters = message.split_off(index);
            parameters.remove(0);
            (message,parameters)
        },
        None => (message, String::new())
    }
}
use std::io::{self, Write};

fn main() {
    let bitmap = "
   **************   *  *** **  *      ******************************
  ********************* ** ** *  * ****************************** *
 **      *****************       ******************************
          *************          **  * **** ** ************** *
           *********            *******   **************** * *
            ********           ***************************  *
   *        * **** ***         *************** ******  ** *
               ****  *         ***************   *** ***  *
                 ******         *************    **   **  *
                 ********        *************    *  ** ***
                   ********         ********          * *** ****
                   *********         ******  *        **** ** * **
                   *********         ****** * *           *** *   *
                     ******          ***** **             *****   *
                     *****            **** *            ********
                    *****             ****              *********
                    ****              **                 *******   *
                    ***                                       *    *
                    **     *                    *
";

    println!("Bitmap Message.");
    println!("Enter the message to display with the bitmap.");

    let mut keyword = String::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush!");

        io::stdin()
            .read_line(&mut keyword)
            .expect("Failed to read line!");

        match keyword.trim().is_empty() {
            true => println!("Message cannot be empty!"),
            false => break,
        }
    }

    println!("....................................................................");

    for line in bitmap.lines() {
        let keyword_char: Vec<char> = keyword.trim().chars().collect();
        for (index, char) in line.chars().enumerate() {
            match char {
                ' ' => print!(" "),
                _ => print!("{}", keyword_char[index % keyword_char.len()]),
            }
        }
        println!();
    }

    print!("....................................................................");
}

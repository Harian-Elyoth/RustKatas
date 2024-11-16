

pub fn draw(letter: String) -> String {
    let mut diamond = String::new();
    /*
        Diamond drawing logic goes here

          AAAAAAAAAA
         AAAAAAAAAAAA
        AAAAAAAAAAAAAA
         AAAAAAAAAAAA
          AAAAAAAAAA
           AAAAAAAA
            AAAAAA
             AAAA
              A       
     */
    // Top of the diamond
    for i in (0..=2).rev() {
        diamond.push_str(&" ".repeat(i));
        diamond.push_str(&letter.repeat(14 - 2 * i));
        diamond.push_str(&" ".repeat(i));
        diamond.push_str("\n");
    };
    for i in 1..6 {
        diamond.push_str(&" ".repeat(i));
        diamond.push_str(&letter.repeat(14 - 2 * i));
        diamond.push_str(&" ".repeat(i));
        diamond.push_str("\n");
    };
    diamond
}
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    fn number_to_words(n: u32) -> String {
        match n {
            0 => "No".to_string(),
            1 => "One".to_string(),
            2 => "Two".to_string(),
            3 => "Three".to_string(),
            4 => "Four".to_string(),
            5 => "Five".to_string(),
            6 => "Six".to_string(),
            7 => "Seven".to_string(),
            8 => "Eight".to_string(),
            9 => "Nine".to_string(),
            10 => "Ten".to_string(),
            _ => n.to_string(),
        }
    }

    let mut verses = Vec::new();

    for i in 0..take_down {
        let bottles = start_bottles - i;
        let next_bottles = bottles.saturating_sub(1);

        let verse = format!(
            "{b} green bottle{s} hanging on the wall,\n\
             {b} green bottle{s} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {next} green bottle{next_s} hanging on the wall.",
            b = number_to_words(bottles),
            s = if bottles == 1 { "" } else { "s" },
            next = number_to_words(next_bottles).to_lowercase(),
            next_s = if next_bottles == 1 { "" } else { "s" }
        );

        verses.push(verse);
    }

    verses.join("\n\n")
}

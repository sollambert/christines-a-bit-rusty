const RESPONSES: [&str; 20] = [
    // Affirmatives
    "It is certain",
    "It is decidedly so",
    "Without a doubt",
    "Yes definitely",
    "You may rely on it",
    "As I see it, yes",
    "Most likely",
    "Outlook good",
    "Yes",
    "Signs point to yes",
    // Non-committal
    "Reply hazy, try again",
    "Ask again later",
    "Better not tell you now",
    "Cannot predict now",
    "Concentrate and ask again",
    // Negatives
    "Don't count on it",
    "My reply is no",
    "My sources say no",
    "Outlook not so good",
    "Very doubtful"
];

fn main() {
    let mut buffer: String = String::new();
    let stdin = std::io::stdin();
    let radius = 14;
    loop {
        let _ = stdin.read_line(&mut buffer);
        let mut chars = buffer.chars().into_iter().rev();
        let _ = chars.next();
        let last = chars.next();
        if last != Some('?') {
            println!("I'm sorry that is not a question");
            continue;
        }
        if buffer.as_str().to_ascii_lowercase() == "goodbye\n" {
            println!("Goodbye!");
            break;
        }
        let response = response_builder(radius / 2 - 1);
        let output = magic_8_ball(&response, radius, radius / 2);
        println!("{}", output);
    }
}

fn magic_8_ball(response: &str, l_rad: u8, s_rad: u8) -> String {
    let mut output_buffer = String::new();
    let mut chars = response.chars();
    for y in 0..l_rad * 2 {
        for x in 0..l_rad * 2 {
            let is_inside_large = is_inside(x as i16, y as i16, l_rad as i16, l_rad as i16, l_rad as i16);
            let is_inside_small_border = is_inside(x as i16, y as i16, s_rad as i16, l_rad as i16, l_rad as i16);
            let is_inside_small = is_inside(x as i16, y as i16, (s_rad - 1) as i16, l_rad as i16, l_rad as i16);
            if is_inside_large {
                if is_inside_small {
                    match chars.next() {
                        Some(c) => output_buffer.push(c),
                        None => output_buffer.push(' '),
                    }
                } else {
                    if is_inside_small_border {
                        output_buffer.push(' ');
                    } else {
                        output_buffer.push('*');
                    }
                }
            } else {
                output_buffer.push(' ');
            }
        }
        output_buffer.push('\n');
    }
    output_buffer.replace("", " ")
}

fn response_builder(radius: u8) -> String {
    let response = RESPONSES[rand::random_range(0..RESPONSES.len())];
    println!("{}", response);
    let mut words = response.split(" ");
    let mut response_lines = Vec::new();
    let mut response_buffer = String::new();
    let mut next_word = match words.next() {
        Some(word) => word,
        None => return String::new(),
    };
    for y in response_lines.len() as u8..radius * 2 {
        let mut x = 0;
        while x < radius * 2 {
            let mut has_space = true;
            if is_inside(
                x as i16,
                y as i16,
                radius as i16,
                radius as i16,
                radius as i16
            ) {
                let mut temp_next_word = next_word.to_string();
                if response_buffer.len() != 0 {
                    temp_next_word.insert(0, ' ');
                }
                for c in 0..temp_next_word.len() as u8 {
                    if !is_inside(
                        (x + c) as i16,
                        y as i16,
                        radius as i16,
                        radius as i16,
                        radius as i16
                    ) {
                        has_space = false;
                    }
                }
                if has_space {
                    response_buffer.push_str(temp_next_word.as_str());
                    x += temp_next_word.len() as u8 - 1;
                    next_word = match words.next() {
                        Some(word) => word,
                        None => "",
                    };
                } else {
                    response_buffer.push(' ');
                }
            }
            if next_word == "" {
                break;
            }
            x += 1;
        }
        response_lines.push(response_buffer);
        response_buffer = String::new();
        if next_word == "" {
            break;
        }
    }
    response_lines.join("")
}

fn is_inside(
    x: i16,
    y: i16,
    rad: i16,
    circle_x: i16,
    circle_y: i16
) -> bool {
    ((x - circle_x) * (x - circle_x)
    + (y - circle_y) * (y - circle_y)) < (rad * rad)
}
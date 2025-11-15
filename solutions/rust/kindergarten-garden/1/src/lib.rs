fn plant_name(c: char) -> &'static str {
    match c {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => "unknown", 
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let children = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry",
    ];
    let mut student_pos = 0;
    for (i, name) in children.iter().enumerate() {
        if *name == student {
            student_pos = i;
            break;
        }
    }
    let plant_index = student_pos * 2;
    let mut lines = diagram.lines();
    let row1: Vec<char> = lines.next().unwrap().chars().collect();
    let row2: Vec<char> = lines.next().unwrap().chars().collect();
    let mut my_plants = Vec::new();
    my_plants.push(plant_name(row1[plant_index]));
    my_plants.push(plant_name(row1[plant_index + 1]));
    my_plants.push(plant_name(row2[plant_index]));
    my_plants.push(plant_name(row2[plant_index + 1]));

    my_plants
}
// TASK: Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn main() {
    let days: [String; 12] = [
        String::from("a partridge in a pear tree"),
        String::from("Two turtle doves"),
        String::from("Three French hens"),
        String::from("Four calling birds"),
        String::from("Five golden rings"),
        String::from("Six geese a layin'"),
        String::from("Seven swans a swimmin'"),
        String::from("Eight maids milkin'"),
        String::from("Nine pipers pipin'"),
        String::from("Ten ladies dancin'"),
        String::from("Eleven Lords a leapin'"),
        String::from("Twelve drummers drummin'"),
    ];

    for day in 0..12 {
        let num: String = match day + 1 {
            1 => String::from("first"),
            2 => String::from("second"),
            3 => String::from("third"),
            4 => String::from("forth"),
            5 => String::from("fifth"),
            6 => String::from("sixth"),
            7 => String::from("seventh"),
            8 => String::from("eighth"),
            9 => String::from("ninth"),
            10 => String::from("tenth"),
            11 => String::from("eleventh"),
            12 => String::from("twelfth"),
            _ => String::from("Error"),
        };

        println!("");
        println!("On the {} day of Christmas My true love gave to me", num);
        for item in (0..day + 1).rev() {
            if day > 0 && item == 0 {
                print!("And ");
            }
            println!("{}", days[item]);
        }
    }
}

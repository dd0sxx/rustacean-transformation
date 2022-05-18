fn main() {
    let gifts: [&str; 12] = [
        "A partridge in a pear tree", 
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
        ];
    let days: [&str; 12] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let mut counter = 0;

    while counter < 12 {
        let mut counter2 = counter;
        println!("on the {} day of Christmas, my true love sent to me", days[counter]);
        while counter2 > 0 {
            println!("{}", gifts[counter2]);
            counter2 -= 1;
        }
        println!("{}", gifts[0]);
        counter += 1;
        println!("\n");
    }
}



// [Verse 1]
// On the first day of Christmas, my true love sent to me
// A partridge in a pear tree
// 
// [Verse 2]
// On the second day of Christmas, my true love sent to me
// Two turtle doves, and
// A partridge in a pear tree
// 
// [Verse 3]
// On the third day of Christmas, my true love sent to me
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree
// 
// [Verse 4]
// On the fourth day of Christmas, my true love sent to me
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree
// 
// [Verse 5]
// On the fifth day of Christmas, my true love sent to me
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree
// 
// [Verse 6]
// On the sixth day of Christmas, my true love sent to me
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree
// 
// [Verse 7]
// On the seventh day of Christmas, my true love sent to me
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// [Verse 8]
// On the eighth day of Christmas, my true love sent to me
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree
// 
// [Verse 9]
// On the ninth day of Christmas, my true love sent to me
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree
// 
// [Verse 10]
// On the tenth day of Christmas, my true love sent to me
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree
// [Verse 11]
// On the eleventh day of Christmas, my true love sent to me
// Eleven pipers piping
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree
// 
// [Verse 12]
// On the twelfth day of Christmas, my true love sent to me
// Twelve drummers drumming
// Eleven pipers piping
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree
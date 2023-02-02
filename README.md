<h1>Structs in Rust</h1>
A struct in Rust is a composite data type that allows you to store a collection of 
values under a single name. They are useful for grouping related data into a single unit, similar to objects in object-oriented programming languages.

<h1>Declaring a Struct</h1>
A struct is declared using the struct keyword followed by the name of the struct. The fields of the struct are enclosed within curly braces {}.

struct Person {
    name: String,
    age: u8,
}

<h1>Instantiating a Struct</h1>
To create an instance of a struct, you use the struct's name followed by curly braces {} containing the values for its fields.

let person = Person {
    name: String::from("John"),
    age: 30,
};

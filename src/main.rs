fn main() {
    println!("Hello, world!");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    println!("{:?}", everyother(numbers, 1))
}
// Im typing out all parts of the project even if the code for the scheme is not in vscode.
// Just so I can keep track of what I have done and what I need to do.

//              Part 1 - SML (not in vscode) & Rust Warmup 
// ngl when I first read the part for part 1 I didn't realize I had to choose one of the warm ups problems
// I thought I had to do all 3, that makes it easier a bit
// Imma do either problem 2 or 3
// Use recursion, no looping/iterating

// Every Other Element
// - Create functions in SML and Rust named everyother that takes an integer list as a parameter
// and returns an integer list consisting of every other element of the parameter list
// In the example a list with 11 elements returns a list with 5 elements, starting with the 2nd element
    // Find the length of the list
    // The index should increase by 2 each time, starting at 1 (the 2nd element)
    // - You should provide error checking to make sure the list is not empty. If it is, print an error message and return an empty list.
fn everyother(list1: Vec<i32>, mut list_index : usize) -> Vec<i32> {
    let mut resultlist : Vec<i32> = Vec::new();

    // // if result list is empty
    // if resultlist.is_empty(){
    //     // set index to 1
    //     list_index = 1;
    // }
    if list_index < list1.len(){
        resultlist.push(list1[list_index]);
        list_index += 2;
        
        let mut remaining = everyother(list1, list_index);
        resultlist.append(&mut remaining);
    }
    resultlist
     
}
    // Imma write it out in comments first, then write the code
    // I need to use recursion to call the function but also speciffy the index that I am currently on in the integer list
    // should have it all the everyother function after incrementing the index by 2
    // checks if the index is still less than the original list to keep going through
    // also a check to make sure the list has atleast 2 elements
    // returns empty list if 0 elements
    // returns list that has just 1 element
    // other do normal everyother functionality
    // if index < intlist.length && index%2==1)
        // add element at that index to result list
        // increment the index
        // return the list while also calling back the function with the current index

// Goldbach's Conjecture
// - Create functions in SML and Rust named goldbach that takes an integer n as a
// parameter and prints the solution satifying Goldbach conjecture.
// The conjecture states that every positive even integer greater than 2 can be expressed as the sum of two primes.
// Your function is to find the two primes that sum to n and print them in the form of "p1 + p2 = n" where p1 and p2 are the two primes that sum to n.
// - You should provide error checking to make sure the integer parameter is a positive even integer greater than 2. If it is not, print an error message and return.

//              Part 2 - Binary Addition / Subtraction
// Use recursion
// 
//              Part 3 - Language Math Translation
//
//              Part 4 - Scheme Preparation 
//              Part 5 - Math Translation in Scheme
//              Part 6 - Function Programming Reflection
//              Part 7 - Extra Credit - Drunk Homer
fn main() {
    println!("Hello, world!");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let numbers2 = vec![1, 2, 3, 4];
    let numbers3 = vec![1];
    let numbers4 = vec![0; 5];


    println!("{:?}", everyother(numbers, 1));
    println!("{:?}", everyother(numbers2, 1));
    println!("{:?}", everyother(numbers3, 1));
    println!("{:?}", everyother(numbers4, 1));

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

    if list1.is_empty(){
        println!("Error: the input list is empty");
        return resultlist;
    }
    if list1.len() == 1{
        print!("The list is the same because there is only one element.");
        return  list1;
    }
    if list_index < list1.len(){ // if index < intlist.length && index%2==1)
        resultlist.push(list1[list_index]); // add element at that index to result list
        list_index += 2; // increment the index by 2 - bc it's every other
        
        // Use recursion to call the function but also speciffy the index that I am currently on in the integer list

        let mut remaining = everyother(list1, list_index);
        resultlist.append(&mut remaining);
    }
    resultlist
}
    // should have it all the everyother function after incrementing the index by 2
    // checks if the index is still less than the original list to keep going through
    // also a check to make sure the list has atleast 2 elements
    // returns empty list if 0 elements
    // returns list that has just 1 element
    // otherwise do normal everyother functionality
        // return the list while also calling back the function with the current index

        // SML version
        // sosml.org/share/f2385ccbe46d1be7dcb2571330228e349625a0e1cc5b09a639c79b295bdbfffc

//              Part 2 - Binary Addition / Subtraction
// Use recursion
// Develop Rust Function to perform Binary addition given two lists of binary integers (1s and 0s)
// Adds topNum, bottomNum, carryNum
// Helper function to turn 1s and 0s to trues and false
// 1 and 1 = 0 AND carryNum = 1
// 0 and 0 = 0 AND carryNum = 0
// 1 and 0 = 1 AND carryNum = 0

// 1, 1, 1 -> 1 and carry 1
// 1, 1, 0 -> 0 and carry 1

// 0, 0, 1 -> 1
// 0, 0, 0 -> 0 

// 1, 0, 1 -> 0 and carry 1
// 1, 0, 0 -> 1 and carry 0


// Start and Go From Right to Left
// Get the length of longest list of binary integers
// Repeat for that length plus 1
// Add the main two numbers and carry
    // Determine the Result Number
    // Add the Result Number to the result list
    // Incre/Decre the index
    // Determine the New Carry Amount

    // let variable called remaing work as the recursive call
        // 
    // append the remaining to the result list

// return the resultlist
// Reverse the order of the result list - currently right to left
// 
// 
//              Part 3 - Language Math Translation
//
//              Part 4 - Scheme Preparation 
//              Part 5 - Math Translation in Scheme
//              Part 6 - Function Programming Reflection
//              Part 7 - Extra Credit - Drunk Homer
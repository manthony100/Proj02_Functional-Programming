fn main() {
    println!("Hello, world!");
    // let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    // let numbers2 = vec![1, 2, 3, 4];
    let numbers3 = vec![1, 0, 1];
    let numbers4 = vec![0, 1, 1];


    // println!("{:?}", everyother(numbers, 1));
    // println!("{:?}", everyother(numbers2, 1));
    // println!("{:?}", everyother(numbers3, 1));
    // println!("{:?}", everyother(numbers4, 1));

    println!("Testing Binary Addition:");
    // binaryaddition(numbers3, numbers4, 0, 0);
    println!("{:?}",binaryaddition(binaryaddition_before_reversal(numbers3, numbers4, 0, 0), 0));

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
fn binaryaddition_before_reversal (bi_nums1 : Vec<i32>, bi_nums2 : Vec<i32>, mut carry_bit : i32,mut num_traversed : usize) -> Vec<i32> {
        let list_length = if bi_nums1.len() > bi_nums2.len(){
        bi_nums1.len()
    } else {
        bi_nums2.len()
    };
        // Base Case
    // Have I processed all the columns in both lists?
    if num_traversed >= list_length{ // if traversed through longest list
        if carry_bit == 0{ // Return an empty list if there is no carry
            return vec![];
        }else { // Return a list containing one if there is a remaining carry
            return vec![1];
        }
    }
    // set to zero at default in case one list is longer than the other
    let mut num1:i32 = 0;
    let mut num2:i32 = 0;

    // for each list
        // if num_traversed is less than its length -> read its bit
        // else: use zero without calculating an index
    if num_traversed < bi_nums1.len(){
        num1 = bi_nums1[bi_nums1.len()-(1+num_traversed)];
    }
    if num_traversed < bi_nums2.len(){
        num2 = bi_nums2[bi_nums2.len()-(1+num_traversed)];
    }
    let column_total = num1 + num2 + carry_bit;

    let mut sumlist : Vec<i32> = Vec::new();
    let sum_digit : i32;

    match column_total {
        0 => {carry_bit=0; // 0 + 0 + 0 --> 0 in result
            sum_digit=0;
        },
        1 => {carry_bit=0;
            sum_digit=1;
        },
        2 => {carry_bit=1;
            sum_digit=0;
        },
        3 => {carry_bit=1;
            sum_digit=1;
        },
        _ => {println!("The input lists are either of the wrong type or are not lists consisting of just binary integers.");
                return sumlist;

        },
    }
    // Increment number of bits traversed
    num_traversed+=1;

    // Recursive Call

    // Combine the recursive call result with the current digit

    sumlist.push(sum_digit);

    let mut remaining = binaryaddition_before_reversal(bi_nums1, bi_nums2, carry_bit, num_traversed);
    sumlist.append(&mut remaining);
    //     resultlist.push(list1[list_index]); // add element at that index to result list
    //     list_index += 2; // increment the index by 2 - bc it's every other
        
    //     // Use recursion to call the function but also speciffy the index that I am currently on in the integer list

    //     let mut remaining = everyother(list1, list_index);
    //     resultlist.append(&mut remaining);
    // }
    // resultlist
    

    sumlist
}

fn binaryaddition(reversedlist : Vec<i32>, mut traversed: usize) -> Vec<i32> {
    let mut normallist : Vec<i32> = Vec::new();
    let num_elements = reversedlist.len();
    // Base Case
        // Have I gone through every item in the list
    if traversed < reversedlist.len(){
        //         resultlist.push(list1[list_index]); // add element at that index to result list
        // list_index += 2; // increment the index by 2 - bc it's every other
        
        // // Use recursion to call the function but also speciffy the index that I am currently on in the integer list

        // let mut remaining = everyother(list1, list_index);
        // resultlist.append(&mut remaining);
        normallist.push(reversedlist[num_elements-(traversed+1)]);
        traversed+=1;
        let mut remaining = binaryaddition(reversedlist, traversed);
        normallist.append(&mut remaining);
    }
    
    // return the normal order list
    normallist

}

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
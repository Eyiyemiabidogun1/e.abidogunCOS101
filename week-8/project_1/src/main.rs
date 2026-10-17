use std::io;
fn input_for_aps_level() -> usize {
      let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("failed to register input");
    let na:usize = input2.trim().parse().expect("Invalid input");
    if na == 0 {
        println!("Your Aps level is APS 1-2");
    }else if na == 1 {
        println!("Your Aps level is APS 3-5");
    }else if na == 2 {
        println!("Your Aps level is Aps 5-8");
    }else if na == 3 {
        println!("Your Aps level is EL1 8-10");
    }else if na == 4 {
        println!("Your aps level is El2 10-13");
    }else if na == 5 {
        println!("Your aps level is SES");
    }else {
        println!("You input the wrong value please try again");
    }
    na
}
fn main() {
    let office_admin= vec![
    "Intern(enter 0)",
    "Administrator(enter 1)",
    "Senior Administrator(enter 2)",
    "Office Manager(enter 3)",
    "Director(enter 4)",
    "Ceo(enter 5)"];
    let academic = vec![
    "You dont know(enter 0)",
    "Research assistant(enter 1)",
    "PhD  candidate(enter 2)",
    "Post-Doc Researcher(enter 3)",
    "Senior lecturer(enter 4)",
    "Dean(enter 5)"];
     let lawyer = vec![
    "Paralegal (enter 0)",
    "Junior Associate (enter 1)",
    "Associate (enter 2)",
    "Senior Associate 1-2 (enter 3)",
    "Senior Associate 3-4 (enter 4)",
    "Partner (enter 5)"
    ];

    let teacher = vec![
    "Placement (enter 0)",
    "Classroom Teacher (enter 1)",
    "Senior Teacher (enter 2)",
    "Leading Teacher (enter 3)",
    "Deputy Principal (enter 4)",
    "Principal (enter 5)"
    ];

    // there is a loop so as to prevent the code from crashing when the user puts in the wrong input
    loop {
            println!("===========THE APS LEVEL CHECKER===========");
    println!("      WELCOME TO THE APS CHECKER      ");
    println!("I would like to ask a few questions so i can get your aps level");
    println!("First, What is your field of work");
    println!("
            1. Office Admin(Enter 1)
            2. Academic(Enter 2)
            3. Law(Enter 3)
            4. Education(Enter 4) ");
    // collects the input of which job they worked
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let c:i32 = input1.trim().parse().expect("failed to input an integer");
    // Checks for which  sector they worked in and gives  out the respective jobs
    if c == 1 {
        println!("What Job did you work");
        let slice1 = &office_admin[0..5];
        println!("{:?}",slice1);
       let na = input_for_aps_level();
    let slice2 = &office_admin[na];
    println!("You worked as a {:?}",slice2);
    break;
    }else if c == 2 {
         println!("What Job did you work");
        let slice1 = &academic[0..5];
        println!("{:?}",slice1);
        let na =input_for_aps_level();
        let slice2 = &academic[na];
    println!("You worked as a {:?}",slice2);
    break;
    }else if c == 3 {
         println!("What Job did you work");
        let slice1 = &lawyer[0..5];
        println!("{:?}",slice1);
         let na =input_for_aps_level();
         let slice2 = &lawyer[na];
    println!("You worked as a {:?}",slice2);
    break;
    }else if c == 4 {
         println!("What Job did you work");
        let slice1 = &teacher[0..5];
        println!("{:?}",slice1);
        let na =input_for_aps_level();
         let slice2 = &teacher[na];
    println!("You worked as a {:?}",slice2);
    break;
    }else {
        println!{"You put the wrong input please try again by using either 0,1,2,3,4 or 5"};
    }
    }
}

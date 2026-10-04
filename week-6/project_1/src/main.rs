use std::io;
use std::{thread, time::Duration};

fn main() {
    let mut wallet: i32 = 0;

    println!("================================");
    println!("     WELCOME TO PAU FOOD");
    println!("================================");

    thread::sleep(Duration::from_secs(1));

    loop {
        println!();
        println!("========== MAIN MENU ==========");
        println!("1. Buy food");
        println!("2. Add money");
        println!("3. Check wallet");
        println!("4. Exit");
        println!("===============================");

        let mut input = String::new();

        println!("Choose an option:");
            
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let answer: i32 = input.trim().parse().expect("Please enter a number");

        if answer == 1 {
            println!();
            println!("========== FOOD MENU ==========");
            println!("P. Poundo Yam / Edikangikong Soup - ₦3500");
            println!("F. Fried Rice & Chicken - ₦3000");
            println!("A. Amala & Ewedu Soup - ₦2500");
            println!("E. Eba & Egusi Soup - ₦2000");
            println!("W. White Rice & Stew - ₦2500");
            println!("===============================");

            let food_price: i32 = loop {
                let mut food = String::new();

                println!("Choose your food:");

                io::stdin().read_line(&mut food).expect("Failed to read input");

                let food = food.trim();

                match food {
                    "P" | "p" => {
                        println!("You selected Poundo Yam / Edikangikong Soup");
                        break 3500;
                    }

                    "F" | "f" => {
                        println!("You selected Fried Rice & Chicken");
                        break 3000;
                    }

                    "A" | "a" => {
                        println!("You selected Amala & Ewedu Soup");
                        break 2500;
                    }

                    "E" | "e" => {
                        println!("You selected Eba & Egusi Soup");
                        break 2000;
                    }

                    "W" | "w" => {
                        println!("You selected White Rice & Stew");
                        break 2500;
                    }

                    _ => {
                        println!("Invalid food choice. Please try again.");
                    }
                }
            };

            let quantity: i32 = loop {
                let mut quantity_input = String::new();

                println!("How many do you want?");

                io::stdin().read_line(&mut quantity_input).expect("Failed to read input");

                let quantity: i32 = quantity_input.trim().parse().expect("Please enter a number");

                if quantity > 0 {
                    break quantity;
                } else {
                    println!("Quantity must be more than 0.");
                }
            };

            let mut total = food_price * quantity;

            println!();
            println!("Price: ₦{}", food_price);
            println!("Quantity: {}", quantity);
            println!("Total before discount: ₦{}", total);

            if total > 10000 {
                let discount = total * 5 / 100;

                total = total - discount;

                println!("You received a 5% discount!");
                println!("Discount: ₦{}", discount);
            }

            println!("Final price: ₦{}", total);

            println!();
            println!("========== CHECKOUT ==========");
            println!("Your total is ₦{}", total);
            println!("Your wallet has ₦{}", wallet);

            if wallet >= total {
                wallet = wallet - total;

                println!("Payment successful!");
                println!("You paid ₦{}", total);
                println!("Remaining wallet: ₦{}", wallet);
            } else {
                println!("You do not have enough money.");
                println!("Please add money to your wallet.");
            }
        }

        else if answer == 2 {
            println!();
            println!("========== ADD MONEY ==========");

            let amount: i32 = loop {
                let mut money = String::new();

                println!("How much money do you want to add?");

                io::stdin().read_line(&mut money).expect("Failed to read input");

                let amount: i32 = money.trim().parse().expect("Please enter a number");
                
                 if amount > 0 {
                    break amount;
                } else {
                    println!("Amount must be more than 0.");
                }
            };

            wallet = wallet + amount;

            println!("₦{} has been added to your wallet.", amount);
            println!("Your new wallet balance is ₦{}", wallet);
        }

        else if answer == 3 {
            println!();
            println!("========== WALLET ==========");
            println!("Your wallet balance is ₦{}", wallet);
        }

        else if answer == 4 {
            println!();
            println!("Thank you for using PAU Food!");
            println!("Goodbye!");

            break;
        }

        else {
            println!();
            println!("Invalid option.");
            println!("Please choose 1, 2, 3 or 4.");
        }
    }
}
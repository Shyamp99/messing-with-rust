// #![allow(warnings)]

use std::{fmt, io, thread};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::rc::Rc;
// use closure::closure

#[derive(Copy, Clone)]
pub struct bankAccount{
    balance: i16,
    id: i16,
}

impl bankAccount{
    pub fn new(balance: i16, id:i16) -> bankAccount{
        bankAccount{
            balance: balance,
            id: id,
        }
    }

    fn handle_command(&mut self, command:String, mut accounts: Arc<Mutex<Vec<bankAccount>>>) -> String{
        let v: Vec<&str> = command.split(' ').collect();
        if command.contains("deposit"){
            let mut tempStr = v[1].to_string();
            tempStr.pop();
            let temp:i16 = tempStr.parse().unwrap();
            // println!("v1: {:?}", v[1]);
            self.balance += temp;
            return format!("${} added to account {:?} has balance {:?}", tempStr, self.id, self.balance);
        }
        // not needed since implemented fmt::display trait
        // else if command.contains("display"){
        //     return format!("Account {:?} has balance {:?}", self.id, self.balance);
        // }
        else if command.contains("withdraw"){
            let mut tempStr = v[1].to_string();
            tempStr.pop();
            let temp:i16 = tempStr.parse().unwrap();
            println!("v1: {:?}", v[1]);
            self.balance -= temp;
            return format!("${} removed from account {:?} has balance {:?}", tempStr, self.id, self.balance);
        }
        else if command.contains("delete"){
            let mut accounts = accounts.lock().unwrap();
            accounts.retain(|x| x.id != self.id);
            return format!("Deleting account: {}, There are {} accounts left", self, accounts.len());
        }
        else if command.contains("exit"){
            return "shutting down".to_string();
        }
        else{
            return "Invalid Command".to_string();
        }

    }

}

impl fmt::Display for bankAccount{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {}, balance: {}", self.id, self.balance)
    }
}

// impl PartialEq for bankAccount{
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id;
//     }
// }


fn main() {
    // let addr = "127.0.0.1:8080".parse::<SocketAddr>()?;
    let listener = TcpListener::bind("127.0.0.1:8888").expect("unable to bind TCP listener");
    println!("Server listening on port 8888");
    let mut accounts= Arc::new(Mutex::new(Vec::new()));
    // let mut accounts = &[bankAccount];
    // let mut threads = vec![];
    
    
    let mut count:Arc<Mutex<i16>> = Arc::new(Mutex::new(0));
    
    // let server = listener.incoming()
    //     .map_err(|e| eprintln!("accept failed = {:?}", e))
    //     .for_each(|sock| {
    //         thread::spawn( |stream| {

    //         });
    //     })

    // let mut curr_account: bankAccount = bankAccount::new(0, 0);
    
    for stream in listener.incoming() {
        // println!("test");
        // let mut curr_account: bankAccount = bankAccount::new(0, 0);
        let stream = stream.unwrap();
        let currStream = stream.try_clone().expect("clone on thread failed");
        let mut input = String::new();
        let count = count.clone();
        let mut accounts = accounts.clone();
        thread::spawn( move || {
            
            
            // let currStream = stream.unwrap();
            // let input = input;
            println!("New Connection: New Thread Spawned");
            
            // let mut accounts = accounts.lock().unwrap();
            let input = handle_input(currStream, Arc::clone(&accounts), count.clone());
            // io::stdin().read_line(&mut input).expect("Failed to read line");
            // let input = input.trim_matches("\u{0}");
            // let input = input.to_lowercase();
            
            // println!("recieved: {}",input);
            // // if count == 0{
            // //     let curr_account = bankAccount::new(0, count);
            // //     count += 1;
            // // }
            // if !input.contains("create") {
            //     let mut accounts = accounts.lock().unwrap();
            //     if !accounts.iter().any(|i| i.id == curr_account.id) {
            //         accounts.push(curr_account.clone());
            //     }
            //     let response = curr_account.handle_command(input.to_string(), &mut accounts);
                
            // }
        
        });
        
    }
    
    drop(listener);
}

fn handle_input(mut stream: TcpStream, mut accounts: Arc<Mutex<Vec<bankAccount>>>, counter:Arc<Mutex<i16>>) {
    let mut curr_account:bankAccount = bankAccount::new(0, 0);
    loop{
        // let mut curr_account:bankAccount = curr_account.clone();
        let mut data = [0 as u8; 100];
        stream.read(&mut data).unwrap();
        let msg = match std::str::from_utf8(&data) {
            Ok(v) => &v[0..v.find(char::from(0)).unwrap()],
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        // let temp = msg.find(char::from(0)).unwrap();
        // println!("val = {}", temp);
        if msg == ""{
            break;
        }
        println!("user inputted: {:?}", msg);
        
        // return msg.to_string();

        if curr_account.id != 0{
            println!("Using Current Account. id = {}, balance = {}", curr_account.id, curr_account.balance);
        }
        else{
            if !msg.contains("open"){
                let mut accounts = accounts.lock().unwrap();
                let mut counter = counter.lock().unwrap();
                *counter += 1;
                curr_account.id = *counter;
                println!("{}", curr_account.clone());
                accounts.push(curr_account.clone());
                println!("Vector len: {}", accounts.len());
            }
            else{
                let v: Vec<&str> = msg.split(' ').collect();
                let account_no = v[1].parse::<i16>().unwrap() as usize;
                let mut accounts = accounts.lock().unwrap();
                if accounts.len() <= account_no-1{
                    let curr_account = accounts[account_no-1];
                    continue;
                }
                
            }

        }
        if msg.contains("create"){
            continue;
        }
        let response = curr_account.handle_command(msg.to_string(), Arc::clone(&accounts));
        // if response.contains("Deleting"){
        //     let curr_account = None;
        // }
        // else{
            println!("curr_account: {}", curr_account);
        // }
        println!("{}\n", response);
        

    }
}
fn main() {

    let mut database: Vec<User> = Vec::new();


    register("Hilal".to_string(), "TR23020420420240420".to_string(), 1343242343, &mut database);
    register("Mehmet".to_string(), "TR2302042443345324".to_string(), 2302429942, &mut database);



    deposit(1343242343, 1250.00, &mut database);
    total_amount(1343242343, &mut database);
    withdraw(1343242343, 500.00, &mut database);
    transfer(1343242343, "TR2302042443345324".to_string(), 100.00, &mut database);


}


#[derive(Debug,Clone)]
struct User {
    iban:String,
    name:String,
    tc:u32,
    amount:f32
}

fn register(name:String,iban:String,tc:u32,database:&mut Vec<User>) {

    let user = User{
        iban:iban,
        name:name,
        tc:tc,
        amount:0.0
    };

    database.push(user.clone());

    println!(" Ziraat Bankamıza hoşgeldiniz {}",user.name)


    
}



fn total_amount(tc:u32,database:&mut Vec<User>) {

    for data in database  {

        if tc == data.tc {
            println!("{:?} hanım bakiyeniz {:?}",data.name,data.amount);
            return;
            
        }
        else {
            println!("Bankamıza kayıtlı değilsiniz")
        }
        
    }
    
}


fn deposit(tc:u32,amount:f32,database:&mut Vec<User>) {

    for data in database  {
        if tc == data.tc {

            data.amount = data.amount + amount;

            println!("Paranız yatırıldı yeni bakiyeniz {:?}",data.amount)   ; 
            return;        
        }
        else {
            println!("Bankamıza kayıtlı değilsiniz")
        }
        
    }
    
}

fn withdraw(tc:u32,amount:f32,database:&mut Vec<User>) {

    for data in database  {
        if tc==data.tc {
            data.amount =data.amount -amount;
            println!("{:?} miktarda para çekildi yeni bakiyeniz {:?}",amount,data.amount);
            return;
            
        }
        else {
            println!("Bankamıza kayıtlı değilsiniz")
        }
        
    }
    
}
fn transfer(tc: u32, iban: String, amount: f32, database: &mut Vec<User>) {
    let mut sender_found = false;
    let mut receiver_found = false;

    for data in database {
        if tc == data.tc {
            sender_found = true;
            if data.amount < amount {
                println!("Yetersiz bakiye! Yeni bakiyeniz: {:?}", data.amount);
                return;
            }

            data.amount -= amount;
            println!("Hesabınızdan bu kadar para çekildi {:?}",data.amount) // Gönderenin bakiyesini azalt
        }

        if iban == data.iban {
            receiver_found = true;
            data.amount += amount;
            println!("Hesabınıza bu kadar para geldi {:?} toplam bakiyeniz : {:?}",amount,data.amount) // Alıcının bakiyesini artır
        }
    }

    if sender_found && receiver_found {
        println!("{:?} miktarda para transfer edildi.", amount);
    } else {
        if !sender_found {
            println!("Bankamıza kayıtlı değilsiniz.");
        }
        if !receiver_found {
            println!("IBAN ile eşleşen bir kullanıcı bulunamadı.");
        }
    }
}



use std::io;
enum Menu
{
    AddressInput(u8),
    AddressSearch(u8),
}

pub struct HumanInfomation
{
    UserName : String,
    Address : String,
}

impl HumanInfomation
{
    pub fn ShowInfomation(self)
    {
        println!("Infomation : {} , {}",self.UserName,self.Address);
    }
}

pub fn showMenu()
{
    println!("1. 주소 입력");
    println!("2. 주소 검색");
}

pub struct HumanInfomationSystem
{
    InfomationDatas : Vec<HumanInfomation>,
}

impl HumanInfomationSystem
{
    pub fn add(&mut self, name : String, address : String)
    {
        let data = HumanInfomation{ UserName: name, Address: address };

        self.InfomationDatas.push(data);
    }
}
fn main() {

    
    let mut lineBuf = String::new();
    loop 
    {
        showMenu();

        io::stdin().read_line(&mut lineBuf);
        
    }
    
}

use std::io;
enum Menu
{
    AddressInput(u8),
    AddressSearch(u8),
}

pub struct HumanAddressData
{
    UserName : String,
    Address : String,
}

impl HumanAddressData
{
    pub fn ShowInfomation(&self)
    {
        println!("Infomation : {} , {}",self.UserName,self.Address);
    }
}

pub fn showMenu()
{
    println!("1. 주소 입력");
    println!("2. 주소 검색");
}

pub struct HumanAddressInfomation
{
    InfomationDatas : Vec<HumanAddressData>,
}

impl HumanAddressInfomation
{
    pub fn add(&mut self, name : String, address : String)
    {
        let data = HumanAddressData{ UserName: name, Address: address };

        self.InfomationDatas.push(data);
        
        
    }

    pub fn sort(&mut self)
    {
        if self.InfomationDatas.len() % 3  == 0
        {
            //self.InfomationDatas.sort_by_key(|sorce| sorce.UserName.get(0..1) );
        }
    }

    pub fn remove(&mut self, name : &str) -> bool
    {
        true
    }

}


fn main() {

    let mut addressInfomationSystem = HumanAddressInfomation{ InfomationDatas: Vec::new() };

    addressInfomationSystem.add(String::from("현준"),String::from("서울 특별시"));

}

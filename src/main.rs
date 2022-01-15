#![allow(warnings)]
#[allow(non_snake_case)]


use std::io;

enum Menu
{
    AddressInput(u8),
    AddressSearch(u8),
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct HumanAddressData
{
    UserName : String,
    Address : String,
}

impl HumanAddressData
{
    pub fn ShowInfomation(&self)
    {
        println!("addressInfo : [{:3}] / [{}]",self.UserName,self.Address);
    }
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
            //self.InfomationDatas.sort_by(|first, next|  );
            self.InfomationDatas.sort();
        }
    }

    pub fn remove(&mut self, name : &str) -> bool
    {
        true
    }

    pub fn showAllData(&self)
    {
        for data in self.InfomationDatas.iter()
        {
            data.ShowInfomation();
        }
    }

}

use HumanAddressInfomation as AddressInfoSys;
fn main() {

    let mut addressInfoSys = AddressInfoSys{ InfomationDatas: Vec::new() };

    addressInfoSys.add(String::from("현준"),String::from("가시덤불골짜기"));
    addressInfoSys.add(String::from("동훈"),String::from("오그리마"));
    addressInfoSys.add(String::from("재민"),String::from("불모의땅"));
    addressInfoSys.add(String::from("용곤"),String::from("스톰윈드"));

    addressInfoSys.showAllData();

}

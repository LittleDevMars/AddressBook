#![allow(warnings)]
#[allow(non_snake_case)]


use std::io;
use std::fmt;

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
    pub fn addString(&mut self, name : String, address : String)
    {
        let data = HumanAddressData{ UserName: name, Address: address };

        self.InfomationDatas.push(data);
        
    }

    pub fn addTuple(&mut self, tuple : (&str,&str))
    {
        let data = HumanAddressData{ UserName: tuple.0.to_string(), Address: tuple.1.to_string() };

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

fn Testcode()
{
    let mut name = String::new();
    
    name = String::from("남현준");
    println!("{:?}",name.chars().next().unwrap());
    println!("{:?}",name.into_bytes().get(0..1).unwrap());

}

use HumanAddressInfomation as AddressInfoSys;

fn main() {

    let mut addressInfoSys = AddressInfoSys{ InfomationDatas: Vec::new() };

    addressInfoSys.addTuple(("현준","가시덤불골짜기"));
    addressInfoSys.addTuple(("동훈","오그리마"));
    addressInfoSys.addTuple(("재민","불모의땅"));
    addressInfoSys.addTuple(("용곤","스톰윈드"));
    

   addressInfoSys.showAllData();

}


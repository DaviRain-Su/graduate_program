use std::{collections::HashMap, collections::BTreeMap};


fn main() {

    let src = "AOPC GUDE YKRO IFKG BEFM CPIY CRAR DEPB
    AQUF EPGH KJPK DDCJ GKPJ IEVC GEBE BAYC
    FAMC XCER IARE HAFF ERJG HCRA OKBB KYAR
    RCED KFAI GHCP CDCK DFCB KKME FEMC GKXC
    OKRQ KYYE BKYC ERBH CCRJ KVEI BKPS AQKU
    FJRK BIDC EMEG HKFC ICRB CRQC ARQK YDER
    SERJ GEIQ KRIA JCPC JRKB BKKX PAOH B";
    count_ascii(src);
}


fn count_ascii(source: &str){
    let source = source
        .to_lowercase()
        .replace(" ", "")
        .replace("\n","");
    println!("source = {:?}", source);
    let ascii : Vec<char> = (97..97+26).map(|val| {
        val as u8 as char
    }).collect();
    println!("ascii = {:?}", ascii);

    let ascii_count : Vec<usize> = ascii.iter().map(|val|{
        let cnt = count(&source, *val);
        cnt
    }).collect(); 
    println!("ascii_count = {:?}", ascii_count);  

    // hashmap 
    let mut ascii_hashmap : BTreeMap<_, _> = ascii
        .iter()
        .zip(ascii_count.iter())
        .collect();
    
    println!("ascii_hashmap = {:?}", ascii_hashmap);
    
    let ascii_hashmap_vec : Vec<(&char, &u32)> = ascii_hashmap.iter().collect();

}


fn count(source: &str, temp_char: char) -> usize {
    let counts : usize = source
        .chars()
        .filter(|val| *val == temp_char)
        .count();
    counts
}

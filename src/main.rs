use crate::blockchain::Blockchain;

mod block;
mod blockchain;



fn main() -> Result<(), jiff::Error> {
    let mut blockchain = Blockchain::new();

    blockchain.mine_block(vec![], 2);

    // println!("{:#?}", blockchain);
    //println!("{:#?}", blockchain.check_validity());
    println!("{}", blockchain);

    Ok(())
}

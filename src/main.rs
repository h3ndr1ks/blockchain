use crate::blockchain::Blockchain;

mod block;
mod blockchain;
mod hash;

fn main() -> Result<(), jiff::Error> {
    let mut blockchain = Blockchain::new();

    blockchain.mine_block(vec![], 1);

    // println!("{:#?}", blockchain);
    println!("{}", blockchain);
    println!("{:#?}", blockchain.check_validity());

    Ok(())
}

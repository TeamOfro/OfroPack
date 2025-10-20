#[derive(clap::Parser, Debug)]
pub struct Add {}

impl super::Run for Add {
    fn run(&self) -> anyhow::Result<()> {
        // TODO:
        // まず処理する内容
        // - 普通のCustomModelの追加
        // - animation付きのCustomModelの追加
        // - blockbenchとかのelementsとか設定されるやつ (jsonの内容をinputで処理する)
        // これらの処理を実装する
        println!("Add command executed");
        Ok(())
    }
}

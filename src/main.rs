use crate::adsb_provider::adsbfi::AdsbFi;
use crate::adsb_provider::AdsbProvider;

mod adsb_provider;

#[tokio::main]
async fn main() {
    let mut prov = AdsbFi::new();
    dbg!(&prov.get_nearby().await);
}

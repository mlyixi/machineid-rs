use machineid::{Encryption, HWIDComponent, IdBuilder};
fn main() {
    // There are 3 different encryption types: MD5, SHA1 and SHA256.
    let mut builder = IdBuilder::new(Encryption::MD5);
    builder
        .add_component(HWIDComponent::SystemID)
        .add_component(HWIDComponent::CPUID);
    let hwid = builder.build("mykey").unwrap();
    let sid = HWIDComponent::SystemID;
    let cpuid = HWIDComponent::CPUID;
    println!("SystemID: {}", sid.to_string().unwrap());
    println!("CPUID: {}", cpuid.to_string().unwrap());
    println!("{}", hwid);
    println!("{}", hwid.len());
}

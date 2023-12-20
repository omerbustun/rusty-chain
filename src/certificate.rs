use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use openssl::x509::{X509NameBuilder, X509};
use openssl::asn1::Asn1Time;
use std::fs;

pub fn create_root_ca_certificate(key_length: u32, country: &str, state_or_province: &str, organization: &str, common_name: &str, validity_days: u32) -> Result<(), Box<dyn std::error::Error>> {
    let rsa = Rsa::generate(key_length)?;
    let pkey = PKey::from_rsa(rsa)?;

    let mut name_builder = X509NameBuilder::new()?;
    name_builder.append_entry_by_text("C", country)?;
    if !state_or_province.is_empty() {
        name_builder.append_entry_by_text("ST", state_or_province)?;
    }
    name_builder.append_entry_by_text("O", organization)?;  // Use provided organization
    name_builder.append_entry_by_text("CN", common_name)?; // Use provided common name
    let name = name_builder.build();

    let mut builder = X509::builder()?;
    builder.set_version(2)?; // X509 v3
    builder.set_subject_name(&name)?;
    builder.set_issuer_name(&name)?;
    builder.set_pubkey(&pkey)?;

    let not_before = Asn1Time::days_from_now(0)?;
    let not_after = Asn1Time::days_from_now(validity_days)?; // Use provided validity directly as u32
    builder.set_not_before(&not_before)?;
    builder.set_not_after(&not_after)?;

    builder.sign(&pkey, openssl::hash::MessageDigest::sha256())?;

    let certificate = builder.build();

    let certificate_pem = certificate.to_pem()?;
    let private_key_pem = pkey.private_key_to_pem_pkcs8()?;

    fs::write("root_ca_cert.pem", &certificate_pem)?;
    fs::write("root_ca_key.pem", &private_key_pem)?;

    Ok(())
}

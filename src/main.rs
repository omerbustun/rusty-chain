use dialoguer::{theme::ColorfulTheme, Select, Input};
mod certificate;

fn main() {
    let selections = &["Create End-Entity Certificate", "Create Intermediate CA Certificate", "Create Root CA Certificate", "Exit"];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Create Root CA Certificate" => create_root_ca_certificate_interactive(),
        //"Create Intermediate CA Certificate" => create_intermediate_ca_certificate(),
        //"Create End-Entity Certificate" => create_end_entity_certificate(),
        "Exit" => return,
        _ => unreachable!(),
    }
}

fn create_root_ca_certificate_interactive() {
    // 1. Key Length
    let key_lengths = vec!["2048", "4096"];
    let key_length_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the key length")
        .default(0) // default to 2048
        .items(&key_lengths)
        .interact()
        .unwrap();
        let key_length = key_lengths[key_length_selection].parse::<u32>().unwrap_or(2048);

    // 2. Country (as text input)
    let country = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the two-letter ISO country code for the Root CA (e.g., 'TR' for Turkey, 'US' for United States)")
        .interact_text()
        .unwrap();

    // 3. State or Province (optional)
    let state_or_province = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the state or province (optional, press Enter to skip)")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    // 4. Organization
    let organization = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the organization name")
        .interact_text()
        .unwrap();

    // 5. Common Name
    let common_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the Common Name (CN)")
        .interact_text()
        .unwrap();

    // 6. Validity in Days
    let validity = Input::<u32>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the validity period in days")
        .default(3650)
        .interact_text()
        .unwrap();

    // Call the function in certificate module with these parameters
    // Modify create_root_ca_certificate to accept these parameters
    match certificate::create_root_ca_certificate(key_length, &country, &state_or_province, &organization, &common_name, validity) {
        Ok(_) => println!("Root CA Certificate created successfully."),
        Err(e) => println!("Failed to create Root CA Certificate: {}", e),
    }
}
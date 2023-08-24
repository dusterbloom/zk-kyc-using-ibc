use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use rand::Rng;

use uuid::Uuid;


enum EntityType {
    HumanIndividual,
    HumanHousehold,
    HumanOrganization,
    OrganizationFormal,
    OrganizationInformal,
    MachineIsolation,
    MachineCluster,
    MachineNetwork,
    MachineProgrammable,
    MachineAutomated,
}

impl EntityType {
    fn to_code(&self) -> &str {
        match self {
            Self::HumanIndividual => "HI",
            Self::HumanHousehold => "HH",
            Self::HumanOrganization => "HO",
            Self::OrganizationFormal => "OF",
            Self::OrganizationInformal => "OI",
            Self::MachineIsolation => "MI",
            Self::MachineCluster => "MC",
            Self::MachineNetwork => "MN",
            Self::MachineProgrammable => "MP",
            Self::MachineAutomated => "MA",
        }
    }
}

struct IAN {
    network_id: String,
    registry_id: String,
    contract_id: String,
    region_id: String,
    entity_type: EntityType,
    entity_id: String,
    check_digits: String,
}

impl IAN {
    fn new(network: &str, registry: &str, contract: &str, region: &str, entity_type: EntityType, entity: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        network.hash(&mut hasher);
        registry.hash(&mut hasher);
        contract.hash(&mut hasher);
        region.hash(&mut hasher);
        entity_type.to_code().hash(&mut hasher);
        entity.hash(&mut hasher);
        let digest = hasher.finish();
        let check_digits = format!("{:x}", digest).to_uppercase();
        Self {
            network_id: network.to_string(),
            registry_id: registry.to_string(),
            contract_id: contract.to_string(),
            region_id: region.to_string(),
            entity_type,
            entity_id: entity.to_string(),
            check_digits,
        }
    }

    fn to_string(&self) -> String {
        let data = format!(
            "{}{}{}{}{}{}{}",
            self.network_id,
            self.registry_id,
            self.contract_id,
            self.region_id,
            self.entity_type.to_code(),
            self.entity_id,
            self.check_digits
        );

        // Hash the concatenated data
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let digest = hasher.finish() as u128; // Cast to u128

        // Generate a random value to combine with the hash
        let mut rng = rand::thread_rng();
        let random_value: u128 = rng.gen();

        // Combine the hash and random value to create a UUID
        let combined_value = digest ^ random_value;
        let ian_uuid = Uuid::from_u128(combined_value.into());

        // Convert to uppercase and add a 2-character prefix
        format!("IA-{}", ian_uuid.to_simple().to_string().to_uppercase())
    }
}

fn main() {
    let ian = IAN::new("ATOM","did:hid:z8uyZoEA2JTCMWfadrSPaqyWmwzwc3qAwAM4snVrfLKue","contract:localloops:cofi1grt939gj98g9kee6hq56wjhlauesx9a7p5r2ll", "CA", EntityType::HumanOrganization, "VAT12345657");
    println!("Generated IAN: {}", ian.to_string());
}
//
// {
//     "IAN": "IA-4611E18FCB762ED905C8E8EAE334D9FD",
//     "Details": {
//     "NetworkID": "ATOM",
//     "RegistryID": "did:hid:z8uyZoEA2JTCMWfadrSPaqyWmwzwc3qAwAM4snVrfLKue",
//     "ContractID": "contract:localloops:cofi1grt939gj98g9kee6hq56wjhlauesx9a7p5r2ll",
//     "RegionID": "UK",
//     "EntityType": "HumanOrganization",
//     "EntityID": "VAT12345657"
//     }
// }

// IA-4611E18FCB762ED905C8E8EAE334D9FD


//
// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};
//
// enum EntityType {
//     HumanIndividual,
//     HumanHousehold,
//     HumanOrganization,
//     OrganizationFormal,
//     OrganizationInformal,
//     MachineIsolation,
//     MachineCluster,
//     MachineNetwork,
//     MachineProgrammable,
//     MachineAutomated,
// }
//
//
//
//
// impl EntityType {
//     fn to_code(&self) -> &str {
//         match self {
//             Self::HumanIndividual => "HI",
//             Self::HumanHousehold => "HH",
//             Self::HumanOrganization => "HO",
//             Self::OrganizationFormal => "OF",
//             Self::OrganizationInformal => "OI",
//             Self::MachineIsolation => "MI",
//             Self::MachineCluster => "MC",
//             Self::MachineNetwork => "MN",
//             Self::MachineProgrammable => "MP",
//             Self::MachineAutomated => "MA",
//         }
//     }
// }
//
// struct IAN {
//     network_id: String,
//     entity_type: EntityType,
//     region_id: String,
//     entity_id: String,
//     check_digits: String,
// }
//
// impl IAN {
//     fn new(network: &str, entity_type: EntityType, region: &str, entity: &str) -> Self {
//         let check_digits = Self::generate_check_digits("random2_string");
//         Self {
//             network_id: network.to_string(),
//             entity_type,
//             region_id: region.to_string(),
//             entity_id: entity.to_string(),
//             check_digits,
//         }
//     }
//
//     fn generate_check_digits(data: &str) -> String {
//         let mut hasher = DefaultHasher::new();
//         data.hash(&mut hasher);
//         let digest = hasher.finish();
//         format!("{:x}", digest).to_uppercase()
//     }
//
//     fn to_string(&self) -> String {
//         let data = format!("{}{}{}{}", self.network_id, self.entity_type.to_code(), self.region_id, self.entity_id);
//         let check_digits = Self::generate_check_digits(&data);
//         format!("{}{}{}{}{}", self.network_id, self.entity_type.to_code(), self.region_id, self.entity_id, check_digits)
//     }
//
// }
//
// fn main() {
//     let ian = IAN::new("ATOM", EntityType::HumanOrganization, "CA", "1234577890");
//     println!("Generated IAN: {}", ian.to_string());
// }
//
//
// // Example output
// // ATOMHOCA12345778904A2B70B904D81A1F
//
// // https://app.coderpad.io/sandbox?snippet=d7f9ecb2
// // ATOMHOCA12345778904A2B70B904D81A1F

/// Signifies the performed operation on a customers appliance
///
/// Each have their own meaning, but honestly I lack the domain
/// knowledge for that.
#[derive(serde::Serialize, serde::Deserialize)]
pub enum OperationPerformed {
    VTP,
    INT,
    PIF,
    RGAZ,
    VGAZ
}

impl std::fmt::Display for OperationPerformed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OperationPerformed::VTP => write!(f, "VTP"),
            OperationPerformed::INT => write!(f, "INT"),
            OperationPerformed::PIF => write!(f, "PIF"),
            OperationPerformed::RGAZ => write!(f, "RGAZ"),
            OperationPerformed::VGAZ => write!(f, "VGAZ")
        }
    }
}

impl std::str::FromStr for OperationPerformed {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VTP" => Ok(Self::VTP),
            "INT" => Ok(Self::INT),
            "PIF" => Ok(Self::PIF),
            "RGAZ" => Ok(Self::RGAZ),
            "VGAZ" => Ok(Self::VGAZ),
            _ => anyhow::bail!(format!(
                "Cannot parse {} into OperationPerformed",
                s
            ))
        }
    }
}

impl TryFrom<String> for OperationPerformed {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "VTP" => Ok(Self::VTP),
            "INT" => Ok(Self::INT),
            "PIF" => Ok(Self::PIF),
            "RGAZ" => Ok(Self::RGAZ),
            "VGAZ" => Ok(Self::VGAZ),
            _ => anyhow::bail!(format!(
                "Cannot parse {} into OperationPerformed",
                value
            ))
        }
    }
}

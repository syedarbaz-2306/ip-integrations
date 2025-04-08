use std::net::IpAddr;

#[derive(Clone, Debug)]
pub enum Variable {
    Boolean(bool),
    Int(i128),
    Decimal(f64),
    Text(String),
    // Domain(fqdn::FQDN),
    // IpAddress(IpAddr),
    // MacAddress(MacAddr),
    // Md5(HexString<16>),
    // Sha1(HexString<20>),
    // Sha256(HexString<32>),
    // Sha512(HexString<64>),
    Null,
}

impl Variable {
    // pub fn can_convert_to(&self, other: CategoryFieldKind) -> bool {
    //     if self.is_boolean() && other == CategoryFieldKind::Boolean {
    //         return true;
    //     }

    //     if self.is_numeric() && matches!(other, CategoryFieldKind::Int | CategoryFieldKind::Decimal)
    //     {
    //         return true;
    //     }

    //     if self.is_string_like() && other == CategoryFieldKind::Text {
    //         return true;
    //     }

    //     false
    // }

    fn is_boolean(&self) -> bool {
        matches!(self, Variable::Boolean(_))
    }

    fn is_numeric(&self) -> bool {
        matches!(self, Variable::Int(_) | Variable::Decimal(_))
    }

    fn is_string_like(&self) -> bool {
        matches!(
            self,
            Variable::Text(_)
                // | Variable::Domain(_)
                // | Variable::IpAddress(_)
                // | Variable::MacAddress(_)
                // | Variable::Md5(_)
                // | Variable::Sha1(_)
                // | Variable::Sha256(_)
                // | Variable::Sha512(_)
                | Variable::Int(_)
                | Variable::Decimal(_)
                | Variable::Boolean(_)
        )
    }
}

// impl From<CategoryValue> for Variable {
//     fn from(value: CategoryValue) -> Self {
//         match value {
//             CategoryValue::Boolean(s) => s.into_single().map_or(Variable::Null, Variable::Boolean),
//             CategoryValue::Int(s) => s
//                 .into_single()
//                 .map_or(Variable::Null, |i| Variable::Int(i as i128)),
//             CategoryValue::Decimal(s) => {
//                 // Convert BigDecimal to f64 (using to_f64(), which returns an Option<f64>)
//                 s.into_single().map_or(Variable::Null, |d| {
//                     Variable::Decimal(d.to_f64().unwrap_or(0.0))
//                 })
//             }
//             CategoryValue::Text(s) => s.into_single().map_or(Variable::Null, Variable::Text),
//             CategoryValue::Domain(s) => s
//                 .into_single()
//                 .map_or(Variable::Null, |sr| Variable::Domain(sr.inner)),
//             CategoryValue::IpAddress(s) => s
//                 .into_single()
//                 .map_or(Variable::Null, |sr| Variable::IpAddress(sr.inner)),
//             CategoryValue::MacAddress(s) => s
//                 .into_single()
//                 .map_or(Variable::Null, |sr| Variable::MacAddress(sr.inner)),
//             CategoryValue::Md5(s) => s
//                 .into_single()
//                 .map_or(Variable::Null, |sr| Variable::Md5(sr.inner)),
//             CategoryValue::Sha1(s) => s
//                 .into_single()
//                 .map_or(Variable::Null, |sr| Variable::Sha1(sr.inner)),
//             CategoryValue::Sha256(s) => s
//                 .into_single()
//                 .map_or(Variable::Null, |sr| Variable::Sha256(sr.inner)),
//             CategoryValue::Sha512(s) => s
//                 .into_single()
//                 .map_or(Variable::Null, |sr| Variable::Sha512(sr.inner)),
//         }
//     }
// }

impl From<bool> for Variable {
    fn from(value: bool) -> Self {
        Variable::Boolean(value)
    }
}

impl From<Option<bool>> for Variable {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(bool) => Variable::Boolean(bool),
            None => Variable::Null,
        }
    }
}

impl From<u64> for Variable {
    fn from(value: u64) -> Self {
        Variable::Int(value.into())
    }
}

impl From<u8> for Variable {
    fn from(value: u8) -> Self {
        Variable::Int(value.into())
    }
}

impl From<Option<u8>> for Variable {
    fn from(value: Option<u8>) -> Self {
        match value {
            Some(integer) => Variable::Int(integer.into()),
            None => Variable::Null,
        }
    }
}

impl From<Option<i64>> for Variable {
    fn from(value: Option<i64>) -> Self {
        match value {
            Some(integer) => Variable::Int(integer.into()),
            None => Variable::Null,
        }
    }
}

impl From<Option<u64>> for Variable {
    fn from(value: Option<u64>) -> Self {
        match value {
            Some(integer) => Variable::Int(integer.into()),
            None => Variable::Null,
        }
    }
}

impl From<u16> for Variable {
    fn from(value: u16) -> Self {
        Variable::Int(value.into())
    }
}

impl From<u32> for Variable {
    fn from(value: u32) -> Self {
        Variable::Int(value.into())
    }
}

impl From<i128> for Variable {
    fn from(value: i128) -> Self {
        Variable::Int(value)
    }
}

impl From<i64> for Variable {
    fn from(value: i64) -> Self {
        Variable::Int(value.into())
    }
}

impl From<i32> for Variable {
    fn from(value: i32) -> Self {
        Variable::Int(value.into())
    }
}

impl From<Option<i32>> for Variable {
    fn from(value: Option<i32>) -> Self {
        match value {
            Some(integer) => Variable::Int(integer.into()),
            None => Variable::Null,
        }
    }
}

impl From<f32> for Variable {
    fn from(value: f32) -> Self {
        Variable::Decimal(value.into())
    }
}

impl From<f64> for Variable {
    fn from(value: f64) -> Self {
        Variable::Decimal(value)
    }
}

impl From<Option<f64>> for Variable {
    fn from(value: Option<f64>) -> Self {
        match value {
            Some(float_value) => Variable::Decimal(float_value),
            None => Variable::Null,
        }
    }
}

impl From<String> for Variable {
    fn from(value: String) -> Self {
        Variable::Text(value)
    }
}

impl From<Option<String>> for Variable {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(value_string) => Variable::Text(value_string),
            None => Variable::Null,
        }
    }
}

impl<'a> From<&'a str> for Variable {
    fn from(value: &'a str) -> Self {
        Variable::Text(value.to_owned())
    }
}

// impl From<IpAddr> for Variable {
//     fn from(value: IpAddr) -> Self {
//         Variable::IpAddress(value)
//     }
// }

// impl From<MacAddr> for Variable {
//     fn from(value: MacAddr) -> Self {
//         Variable::MacAddress(value)
//     }
// }

// impl From<HexString<16>> for Variable {
//     fn from(value: HexString<16>) -> Self {
//         Variable::Md5(value)
//     }
// }

// impl From<HexString<20>> for Variable {
//     fn from(value: HexString<20>) -> Self {
//         Variable::Sha1(value)
//     }
// }

// impl From<HexString<32>> for Variable {
//     fn from(value: HexString<32>) -> Self {
//         Variable::Sha256(value)
//     }
// }

// impl From<HexString<64>> for Variable {
//     fn from(value: HexString<64>) -> Self {
//         Variable::Sha512(value)
//     }
// }

// Try into

impl Into<String> for Variable {
    fn into(self) -> String {
        match self {
            Variable::Boolean(n) => match n {
                true => "true",
                false => "false",
            }
            .to_string(),
            Variable::Int(n) => n.to_string(),
            Variable::Decimal(n) => format!("{:.3}", n),
            Variable::Text(n) => n,
            // Variable::Domain(n) => n.to_string(),
            // Variable::IpAddress(n) => n.to_string(),
            // Variable::MacAddress(n) => n.to_string(),
            // Variable::Md5(n) => n.to_string(),
            // Variable::Sha1(n) => n.to_string(),
            // Variable::Sha256(n) => n.to_string(),
            // Variable::Sha512(n) => n.to_string(),
            Variable::Null => "null".to_string(),
        }
    }
}

// impl TryInto<TlpLevel> for Variable {
//     type Error = ();

//     fn try_into(self) -> Result<TlpLevel, Self::Error> {
//         match self {
//             Variable::Text(n) => match n.as_str() {
//                 "red" => Ok(TlpLevel::Red),
//                 "amber" => Ok(TlpLevel::Amber),
//                 "green" => Ok(TlpLevel::Green),
//                 "white" => Ok(TlpLevel::White),
//                 _ => Err(()),
//             },
//             _ => Err(()),
//         }
//     }
// }

// impl TryInto<IpAddr> for Variable {
//     type Error = ();

//     fn try_into(self) -> Result<IpAddr, Self::Error> {
//         match self {
//             Variable::IpAddress(n) => Ok(n),
//             Variable::Text(n) => n.parse().map_err(|_| ()),
//             _ => Err(()),
//         }
//     }
// }

// impl TryInto<FQDN> for Variable {
//     type Error = ();

//     fn try_into(self) -> Result<FQDN, Self::Error> {
//         match self {
//             Variable::Domain(n) => Ok(n),
//             Variable::Text(n) => n.parse().map_err(|_| ()),
//             _ => Err(()),
//         }
//     }
// }

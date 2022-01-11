use bytes::Bytes;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

use crate::hex::encode_hex;

//系统版本
#[derive(Serialize, Deserialize, Debug)]
pub struct OSVersion {
    pub incremental: String,
    pub release: String,
    pub codename: String,
    pub sdk: u32,
}

impl Default for OSVersion {
    fn default() -> Self {
        OSVersion {
            incremental: "5891938".to_string(),
            release: "10".to_string(),
            codename: "REL".to_string(),
            sdk: 29,
        }
    }
}

//手机设备信息
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Device {
    pub display: String,
    pub product: String,
    pub device: String,
    pub board: String,
    pub model: String,
    pub finger_print: String,
    pub boot_id: String,
    pub proc_version: String,
    pub imei: String,
    pub brand: String,
    pub bootloader: String,
    pub base_band: String,
    pub version: OSVersion,
    pub sim_info: String,
    pub os_type: String,
    pub mac_address: String,
    pub ip_address: Vec<u8>,
    pub wifi_bssid: String,
    pub wifi_ssid: String,
    pub imsi_md5: Vec<u8>,
    pub android_id: String,
    pub apn: String,
    pub vendor_name: String,
    pub vendor_os_name: String,
}

impl Device {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            display: "GMC.".to_string() + &rng.gen_range(100000..999999).to_string() + ".001",
            product: "iarim".to_string(),
            device: "sagit".to_string(),
            board: "eomam".to_string(),
            model: "MI 6".to_string(),
            finger_print: "xiaomi/iarim/sagit:10/eomam.200122.001/".to_string()
                + &rng.gen_range(1000000..9999999).to_string()
                + &":user/release-keys".to_string(),
            boot_id: random_uuid(),
            proc_version: "Linux 5.4.0-54-generic-".to_string()
                + &random_string(8)
                + &" (android-build@google.com)".to_string(),
            imei: random_imei(),
            brand: "Xiaomi".to_string(),
            bootloader: "U-boot".to_string(),
            base_band: "".to_string(),
            version: OSVersion::default(),
            sim_info: "T-Mobile".to_string(),
            os_type: "android".to_string(),
            mac_address: "00:50:56:C0:00:08".to_string(),
            ip_address: vec![10, 0, 1, 3],
            wifi_bssid: "00:50:56:C0:00:08".to_string(),
            wifi_ssid: "<unknown ssid>".to_string(),
            imsi_md5: md5::compute(rand::thread_rng().gen::<[u8; 16]>()).to_vec(),
            android_id: encode_hex(&rand::thread_rng().gen::<[u8; 8]>()),
            apn: "wifi".to_string(),
            vendor_name: "MIUI".to_string(),
            vendor_os_name: "gmc".to_string(),
        }
    }

    pub fn guid(&self) -> Bytes {
        Bytes::from(md5::compute(self.android_id.to_owned() + &self.mac_address).to_vec())
    }

    pub fn tgtgt_key(&self) -> Bytes {
        Bytes::from(md5::compute(self.guid()).to_vec())
    }
}

pub fn random_string(len: usize) -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
}

pub fn random_uuid() -> String {
    let r = md5::compute(&rand::thread_rng().gen::<[u8; 16]>()).to_vec();
    format!(
        "{}-{}-{}-{}-{}",
        encode_hex(&r[0..4]),
        encode_hex(&r[4..6]),
        encode_hex(&r[6..8]),
        encode_hex(&r[8..10]),
        encode_hex(&r[10..16])
    )
}

pub fn random_imei() -> String {
    let mut sum = 0;
    let mut str = String::new();
    let mut rng = rand::thread_rng();
    for i in 0..14 {
        let mut to_add = rng.gen_range(0..10);
        if (i + 2) % 2 == 0 {
            to_add *= 2;
            if to_add >= 10 {
                to_add = (to_add % 10) + 1
            }
        }
        sum += to_add;
        str.push_str(&to_add.to_string());
    }
    let ctrl_digit = (sum * 9) % 10;
    str.push_str(&ctrl_digit.to_string());
    str
}

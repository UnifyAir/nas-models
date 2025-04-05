/*
 * The MIT License
 *
 * Copyright (C) 2019-2023 by UnifyAir Inc. <info@unifyair.com>
 *
 * This file is part of UnifyAir Core.
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
 * LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 * OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
 * WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

/*******************************************************************************
 * This file had been created by rust-nas-message.py script v0.2.0
 * Please do not modify this file but regenerate it via script.
 * Created on: 2025-03-09 19:42:50.698547 by nr
 * from 24501-h90.docx
 ******************************************************************************/

//Auto-generated
use bitfield::bitfield;
use derive_more::{From, Into};
use tlv::prelude::*;
use tlv_derive::{TlvDecode, TlvEncode};

//Manually-generated
use crate::message::{
    NAS_EXTENDED_PROTOCOL_DISCRIMINATOR_5GMM, NAS_EXTENDED_PROTOCOL_DISCRIMINATOR_5GSM,
    NAS_MESSAGE_TYPE_5GMM_STATUS, NAS_MESSAGE_TYPE_5GSM_STATUS,
    NAS_MESSAGE_TYPE_AUTHENTICATION_FAILURE, NAS_MESSAGE_TYPE_AUTHENTICATION_REJECT,
    NAS_MESSAGE_TYPE_AUTHENTICATION_REQUEST, NAS_MESSAGE_TYPE_AUTHENTICATION_RESPONSE,
    NAS_MESSAGE_TYPE_AUTHENTICATION_RESULT, NAS_MESSAGE_TYPE_CONFIGURATION_UPDATE_COMMAND,
    NAS_MESSAGE_TYPE_CONFIGURATION_UPDATE_COMPLETE, NAS_MESSAGE_TYPE_DEREGISTRATION_ACCEPT_FROM_UE,
    NAS_MESSAGE_TYPE_DEREGISTRATION_ACCEPT_TO_UE, NAS_MESSAGE_TYPE_DEREGISTRATION_REQUEST_FROM_UE,
    NAS_MESSAGE_TYPE_DEREGISTRATION_REQUEST_TO_UE, NAS_MESSAGE_TYPE_DL_NAS_TRANSPORT,
    NAS_MESSAGE_TYPE_IDENTITY_REQUEST, NAS_MESSAGE_TYPE_IDENTITY_RESPONSE,
    NAS_MESSAGE_TYPE_NOTIFICATION, NAS_MESSAGE_TYPE_NOTIFICATION_RESPONSE,
    NAS_MESSAGE_TYPE_PDU_SESSION_AUTHENTICATION_COMMAND,
    NAS_MESSAGE_TYPE_PDU_SESSION_AUTHENTICATION_COMPLETE,
    NAS_MESSAGE_TYPE_PDU_SESSION_AUTHENTICATION_RESULT,
    NAS_MESSAGE_TYPE_PDU_SESSION_ESTABLISHMENT_ACCEPT,
    NAS_MESSAGE_TYPE_PDU_SESSION_ESTABLISHMENT_REJECT,
    NAS_MESSAGE_TYPE_PDU_SESSION_ESTABLISHMENT_REQUEST,
    NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_COMMAND,
    NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_COMMAND_REJECT,
    NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_COMPLETE,
    NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_REJECT,
    NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_REQUEST,
    NAS_MESSAGE_TYPE_PDU_SESSION_RELEASE_COMMAND,
    NAS_MESSAGE_TYPE_PDU_SESSION_RELEASE_COMPLETE,
    NAS_MESSAGE_TYPE_PDU_SESSION_RELEASE_REJECT,
    NAS_MESSAGE_TYPE_PDU_SESSION_RELEASE_REQUEST,
    NAS_MESSAGE_TYPE_REGISTRATION_ACCEPT,
    NAS_MESSAGE_TYPE_REGISTRATION_COMPLETE,
    NAS_MESSAGE_TYPE_REGISTRATION_REJECT,
    NAS_MESSAGE_TYPE_REGISTRATION_REQUEST,
    NAS_MESSAGE_TYPE_SECURITY_MODE_COMMAND,
    NAS_MESSAGE_TYPE_SECURITY_MODE_COMPLETE,
    NAS_MESSAGE_TYPE_SECURITY_MODE_REJECT,
    NAS_MESSAGE_TYPE_SERVICE_ACCEPT,
    NAS_MESSAGE_TYPE_SERVICE_REJECT,
    NAS_MESSAGE_TYPE_SERVICE_REQUEST,
    NAS_MESSAGE_TYPE_UL_NAS_TRANSPORT,
};

// ******************************************************************
// ExtendedProtocolDiscriminator
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ExtendedProtocolDiscriminator(u8);

impl ExtendedProtocolDiscriminator {
    pub const fn gmm() -> Self {
        ExtendedProtocolDiscriminator(NAS_EXTENDED_PROTOCOL_DISCRIMINATOR_5GMM)
    }
    pub const fn gsm() -> Self {
        ExtendedProtocolDiscriminator(NAS_EXTENDED_PROTOCOL_DISCRIMINATOR_5GSM)
    }
}

// ******************************************************************
// SecurityHeaderType
// ******************************************************************

// Auto-generated
// #[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
// pub struct SecurityHeaderType(u8);

bitfield! {
    #[derive(TlvEncode, TlvDecode, Into, From, Clone)]
    pub struct SecurityHeaderType(u8);
    impl Debug;
    u8;
    pub get_raw_security_header_type, set_raw_security_header_type: 3, 0;
}

pub enum SecurityType {
    PlainNasMessage = 0b0000,
    IntegrityProtected = 0b0001,
    IntegrityProtectedAndCiphered = 0b0010,
    IntegrityProtectedWithNew5gNasSecurityContext = 0b0011,
    IntegrityProtectedAndCipheredWithNew5gNasSecurityContext = 0b0100,
}

impl SecurityHeaderType {
    pub fn new(security_type: SecurityType) -> Self {
        Self(security_type as u8)
    }

    pub fn get_security_type(&self) -> Option<SecurityType> {
        match self.get_raw_security_header_type() {
            0b0000 => Some(SecurityType::PlainNasMessage),
            0b0001 => Some(SecurityType::IntegrityProtected),
            0b0010 => Some(SecurityType::IntegrityProtectedAndCiphered),
            0b0011 => Some(SecurityType::IntegrityProtectedWithNew5gNasSecurityContext),
            0b0100 => Some(SecurityType::IntegrityProtectedAndCipheredWithNew5gNasSecurityContext),
            _ => None,
        }
    }

    pub fn set_security_type_enum(&mut self, security_type: SecurityType) {
        self.set_raw_security_header_type(security_type as u8);
    }
}

// ******************************************************************
// SpareHalfOctet
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SpareHalfOctet(u8);

impl SpareHalfOctet {
    pub const fn zero() -> Self {
        Self(0u8)
    }
}

// ******************************************************************
// MessageType
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MessageType(u8);

impl MessageType {
    pub const fn registration_request() -> Self {
        MessageType(NAS_MESSAGE_TYPE_REGISTRATION_REQUEST)
    }
    
    pub const fn registration_accept() -> Self {
        MessageType(NAS_MESSAGE_TYPE_REGISTRATION_ACCEPT)
    }
    
    pub const fn registration_complete() -> Self {
        MessageType(NAS_MESSAGE_TYPE_REGISTRATION_COMPLETE)
    }
    
    pub const fn registration_reject() -> Self {
        MessageType(NAS_MESSAGE_TYPE_REGISTRATION_REJECT)
    }
    
    pub const fn deregistration_request_from_ue() -> Self {
        MessageType(NAS_MESSAGE_TYPE_DEREGISTRATION_REQUEST_FROM_UE)
    }
    
    pub const fn deregistration_accept_from_ue() -> Self {
        MessageType(NAS_MESSAGE_TYPE_DEREGISTRATION_ACCEPT_FROM_UE)
    }
    
    pub const fn deregistration_request_to_ue() -> Self {
        MessageType(NAS_MESSAGE_TYPE_DEREGISTRATION_REQUEST_TO_UE)
    }
    
    pub const fn deregistration_accept_to_ue() -> Self {
        MessageType(NAS_MESSAGE_TYPE_DEREGISTRATION_ACCEPT_TO_UE)
    }
    
    pub const fn service_request() -> Self {
        MessageType(NAS_MESSAGE_TYPE_SERVICE_REQUEST)
    }
    
    pub const fn service_reject() -> Self {
        MessageType(NAS_MESSAGE_TYPE_SERVICE_REJECT)
    }
    
    pub const fn service_accept() -> Self {
        MessageType(NAS_MESSAGE_TYPE_SERVICE_ACCEPT)
    }
    
    pub const fn configuration_update_command() -> Self {
        MessageType(NAS_MESSAGE_TYPE_CONFIGURATION_UPDATE_COMMAND)
    }
    
    pub const fn configuration_update_complete() -> Self {
        MessageType(NAS_MESSAGE_TYPE_CONFIGURATION_UPDATE_COMPLETE)
    }
    
    pub const fn authentication_request() -> Self {
        MessageType(NAS_MESSAGE_TYPE_AUTHENTICATION_REQUEST)
    }
    
    pub const fn authentication_response() -> Self {
        MessageType(NAS_MESSAGE_TYPE_AUTHENTICATION_RESPONSE)
    }
    
    pub const fn authentication_reject() -> Self {
        MessageType(NAS_MESSAGE_TYPE_AUTHENTICATION_REJECT)
    }
    
    pub const fn authentication_failure() -> Self {
        MessageType(NAS_MESSAGE_TYPE_AUTHENTICATION_FAILURE)
    }
    
    pub const fn authentication_result() -> Self {
        MessageType(NAS_MESSAGE_TYPE_AUTHENTICATION_RESULT)
    }
    
    pub const fn identity_request() -> Self {
        MessageType(NAS_MESSAGE_TYPE_IDENTITY_REQUEST)
    }
    
    pub const fn identity_response() -> Self {
        MessageType(NAS_MESSAGE_TYPE_IDENTITY_RESPONSE)
    }
    
    pub const fn security_mode_command() -> Self {
        MessageType(NAS_MESSAGE_TYPE_SECURITY_MODE_COMMAND)
    }
    
    pub const fn security_mode_complete() -> Self {
        MessageType(NAS_MESSAGE_TYPE_SECURITY_MODE_COMPLETE)
    }
    
    pub const fn security_mode_reject() -> Self {
        MessageType(NAS_MESSAGE_TYPE_SECURITY_MODE_REJECT)
    }
    
    pub const fn gmm_status() -> Self {
        MessageType(NAS_MESSAGE_TYPE_5GMM_STATUS)
    }
    
    pub const fn notification() -> Self {
        MessageType(NAS_MESSAGE_TYPE_NOTIFICATION)
    }
    
    pub const fn notification_response() -> Self {
        MessageType(NAS_MESSAGE_TYPE_NOTIFICATION_RESPONSE)
    }
    
    pub const fn ul_nas_transport() -> Self {
        MessageType(NAS_MESSAGE_TYPE_UL_NAS_TRANSPORT)
    }
    
    pub const fn dl_nas_transport() -> Self {
        MessageType(NAS_MESSAGE_TYPE_DL_NAS_TRANSPORT)
    }
    
    pub const fn pdu_session_establishment_request() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_ESTABLISHMENT_REQUEST)
    }
    
    pub const fn pdu_session_establishment_accept() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_ESTABLISHMENT_ACCEPT)
    }
    
    pub const fn pdu_session_establishment_reject() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_ESTABLISHMENT_REJECT)
    }
    
    pub const fn pdu_session_authentication_command() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_AUTHENTICATION_COMMAND)
    }
    
    pub const fn pdu_session_authentication_complete() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_AUTHENTICATION_COMPLETE)
    }
    
    pub const fn pdu_session_authentication_result() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_AUTHENTICATION_RESULT)
    }
    
    pub const fn pdu_session_modification_request() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_REQUEST)
    }
    
    pub const fn pdu_session_modification_reject() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_REJECT)
    }
    
    pub const fn pdu_session_modification_command() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_COMMAND)
    }
    
    pub const fn pdu_session_modification_complete() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_COMPLETE)
    }
    
    pub const fn pdu_session_modification_command_reject() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_COMMAND_REJECT)
    }
    
    pub const fn pdu_session_release_request() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_RELEASE_REQUEST)
    }
    
    pub const fn pdu_session_release_reject() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_RELEASE_REJECT)
    }
    
    pub const fn pdu_session_release_command() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_RELEASE_COMMAND)
    }
    
    pub const fn pdu_session_release_complete() -> Self {
        MessageType(NAS_MESSAGE_TYPE_PDU_SESSION_RELEASE_COMPLETE)
    }
    
    pub const fn gsm_status() -> Self {
        MessageType(NAS_MESSAGE_TYPE_5GSM_STATUS)
    }
}


// ******************************************************************
// FiveGsRegistrationType
// ******************************************************************

// Auto-generated
// #[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
// pub struct FiveGsRegistrationType(u8);

bitfield! {
    #[derive(TlvEncode, TlvDecode, Into, From, Clone)]
    pub struct FiveGsRegistrationType(u8);
    impl Debug;
    u8;
    pub get_raw_registration_type, set_raw_registration_type: 2, 0;
    pub get_follow_on_request, set_follow_on_request: 3;
}

pub enum RegistrationType {
    InitialRegistration = 0b001,
    MobilityRegistrationUpdating = 0b010,
    PeriodicRegistrationUpdating = 0b011,
    EmergencyRegistration = 0b100,
    SnpnOnboardingRegistration = 0b101,
    DisasterRoamingMobilityRegistrationUpdating = 0b110,
    DisasterRoamingInitialRegistration = 0b111,
}

impl FiveGsRegistrationType {
    pub fn new(reg_type: RegistrationType) -> Self {
        Self(reg_type as u8)
    }

    pub fn get_registration_type(&self) -> RegistrationType {
        match self.get_raw_registration_type() {
            0b001 => RegistrationType::InitialRegistration,
            0b010 => RegistrationType::MobilityRegistrationUpdating,
            0b011 => RegistrationType::PeriodicRegistrationUpdating,
            0b100 => RegistrationType::EmergencyRegistration,
            0b101 => RegistrationType::SnpnOnboardingRegistration,
            0b110 => RegistrationType::DisasterRoamingMobilityRegistrationUpdating,
            0b111 => RegistrationType::DisasterRoamingInitialRegistration,
            _ => RegistrationType::InitialRegistration,
        }
    }

    pub fn set_registration_type_enum(&mut self, reg_type: RegistrationType) {
        self.set_raw_registration_type(reg_type as u8);
    }
}

// ******************************************************************
// KeySetIdentifier
// ******************************************************************

// Auto-generated
// #[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
// pub struct KeySetIdentifier(u8);

bitfield! {
    #[derive(TlvEncode, TlvDecode, Into, From, Clone)]
    pub struct KeySetIdentifier(u8);
    impl Debug;
    u8;
    pub get_raw_nas_key_set_identifier, set_raw_nas_key_set_identifier: 2, 0;
    pub get_raw_security_context_type, set_raw_security_context_type: 3;
}

pub enum SecurityContextType {
    NativeSecurityContext = 0,
    MappedSecurityContext = 1,
}

pub enum NasKeySetIdentifierValue {
    KeyAvailable(u8),
    NoKeyAvailable,
}

impl KeySetIdentifier {
    pub fn new(
        security_context_type: SecurityContextType,
        nas_key_set_identifier_value: NasKeySetIdentifierValue,
    ) -> KeySetIdentifier {
        let mut result = Self(0);
        Self::set_security_context_type(&mut result, security_context_type);
        Self::set_key_set_identifier(&mut result, nas_key_set_identifier_value);
        result
    }

    pub fn get_security_context_type(&self) -> SecurityContextType {
        match self.get_raw_security_context_type() {
            false => SecurityContextType::NativeSecurityContext,
            true => SecurityContextType::MappedSecurityContext,
        }
    }

    pub fn set_security_context_type(&mut self, context_type: SecurityContextType) {
        match context_type {
            SecurityContextType::NativeSecurityContext => self.set_raw_security_context_type(false),
            SecurityContextType::MappedSecurityContext => self.set_raw_security_context_type(true),
        }
    }

    pub fn get_key_set_identifier(&self) -> NasKeySetIdentifierValue {
        let value = self.get_raw_nas_key_set_identifier();
        if value == 0b111 {
            NasKeySetIdentifierValue::NoKeyAvailable
        } else {
            NasKeySetIdentifierValue::KeyAvailable(value)
        }
    }

    pub fn set_key_set_identifier(&mut self, identifier: NasKeySetIdentifierValue) {
        match identifier {
            NasKeySetIdentifierValue::KeyAvailable(value) => {
                if value <= 0b110 {
                    self.set_raw_nas_key_set_identifier(value);
                }
            }
            NasKeySetIdentifierValue::NoKeyAvailable => {
                self.set_raw_nas_key_set_identifier(0b111);
            }
        }
    }
}

// ******************************************************************
// FiveGsMobileIdentity
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FiveGsMobileIdentity(pub Vec<u8>);

impl FiveGsMobileIdentity {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_mobile_identity_type(&self) -> IdentityType {
        FiveGsIdentityType(self.0[0] & 0b00000111).get_identity_type()
    }

    pub fn get_mobile_identity(&self) -> MobileIdentity {
        match self.get_mobile_identity_type() {
            IdentityType::NoIdentity => MobileIdentity::NoIdentity(NoIdentity(self.0.as_ref())),
            IdentityType::Suci => MobileIdentity::Suci(Suci(self.0.as_ref())),
            IdentityType::FiveGGuti => MobileIdentity::FiveGGuti(FiveGGuti(self.0.as_ref())),
            IdentityType::Imei => MobileIdentity::Imei(ImeiOrImeiSv(self.0.as_ref())),
            IdentityType::FiveGSTmsi => MobileIdentity::FiveGSTmsi(FiveGTmsi(self.0.as_ref())),
            IdentityType::Imeisv => MobileIdentity::Imeisv(ImeiOrImeiSv(self.0.as_ref())),
            IdentityType::MacAddress => MobileIdentity::MacAddress(MacAddress(self.0.as_ref())),
            IdentityType::Eui64 => MobileIdentity::Eui64(Eui64(self.0.as_ref())),
        }
    }

    pub fn set_mobile_identity(&mut self, identity: MobileIdentity) {
        match identity {
            MobileIdentity::NoIdentity(no_identity) => self.0 = no_identity.0.to_owned(),
            MobileIdentity::Suci(suci) => self.0 = suci.0.to_owned(),
            MobileIdentity::FiveGGuti(guti) => self.0 = guti.0.to_owned(),
            MobileIdentity::Imei(imei) => self.0 = imei.0.to_owned(),
            MobileIdentity::FiveGSTmsi(five_gs_tmsi) => self.0 = five_gs_tmsi.0.to_owned(),
            MobileIdentity::Imeisv(imeisv) => self.0 = imeisv.0.to_owned(),
            MobileIdentity::MacAddress(mac_addr) => self.0 = mac_addr.0.to_owned(),
            MobileIdentity::Eui64(eui64) => self.0 = eui64.0.to_owned(),
        }
    }
}

pub enum MobileIdentity<'a> {
    NoIdentity(NoIdentity<&'a [u8]>),
    Suci(Suci<'a>),
    FiveGGuti(FiveGGuti<&'a [u8]>),
    Imei(ImeiOrImeiSv<'a>),
    FiveGSTmsi(FiveGTmsi<&'a [u8]>),
    Imeisv(ImeiOrImeiSv<'a>),
    MacAddress(MacAddress<'a>),
    Eui64(Eui64<'a>),
}

// ******************************************************************
// FiveGGUTI, Guti, Tmsi
// ******************************************************************

// Manually-generated
bitfield! {
    #[derive(Clone)]
    pub struct FiveGGuti(MSB0 [u8]);
    impl Debug;
    u8;
    pub from into FiveGsIdentityType, get_identity_type, set_identity_type: 2, 0;
    pub get_mcc_digit_1, set_mcc_digit_1: 11, 8;
    pub get_mcc_digit_2, set_mcc_digit_2: 15, 12;
    pub get_mcc_digit_3, set_mcc_digit_3: 19, 16;
    pub get_mnc_digit_3, set_mnc_digit_3: 23, 20;
    pub get_mnc_digit_1, set_mnc_digit_1: 27, 24;
    pub get_mnc_digit_2, set_mnc_digit_2: 31, 28;
    pub get_amf_region_id, set_amf_region_id: 39, 32;
    pub get_amf_set_id, set_amf_set_id: 47, 40;
    pub get_amf_pointer, set_amf_pointer: 53, 48;
    pub get_amf_set_id_contd, set_amf_set_id_contd: 55, 54;
    pub u32, get_5g_tmsi, set_5g_tmsi: 87, 56;
}


impl<T: AsRef<[u8]>> FiveGGuti<T> {
    pub fn get_plmnid_string(&self) -> String {
        let mcc_digit_1 = self.get_mcc_digit_1();
        let mcc_digit_2 = self.get_mcc_digit_2();
        let mcc_digit_3 = self.get_mcc_digit_3();
        let mnc_digit_1 = self.get_mnc_digit_1();
        let mnc_digit_2 = self.get_mnc_digit_2();
        let mnc_digit_3 = self.get_mnc_digit_3();
        
        let bytes = [
            (mcc_digit_1 << 4) | mcc_digit_2,
            (mcc_digit_3 << 4) | mnc_digit_1,
            (mnc_digit_2 << 4) | mnc_digit_3,
        ];
        
        let plmn_id = hex::encode(&bytes);
        
        if plmn_id.chars().nth(5) == Some('f') {
            plmn_id[..5].to_string()
        } else {
            plmn_id
        }
    }

    pub fn get_amfid_string(&self) -> String {
        let data = self.0.as_ref();
        hex::encode(&data[4..7])
    }

    pub fn get_tmsi_string(&self) -> String {
        let data = self.0.as_ref();
        hex::encode(&data[7..11])
    }

    pub fn get_guti_string(&self) -> String {
        let plmn_id = self.get_plmnid_string();
        let amf_id = self.get_amfid_string();
        let tmsi = self.get_tmsi_string();
        
        format!("{}:{}:{}", plmn_id, amf_id, tmsi)
    }
}

// ******************************************************************
// IMEI
// ******************************************************************

// Manually-generated
#[derive(Debug, Clone)]
pub struct ImeiOrImeiSv<'a>(&'a [u8]);

impl ImeiOrImeiSv<'_> {
    pub fn get_identity_type(&self) -> FiveGsIdentityType {
        FiveGsIdentityType(&self.0[0] & 0b00000111)
    }

    pub fn is_even_digit(&self) -> bool {
        (self.0[0] & 0b00001000) == 0
    }

    pub fn get_imei_imeisv_data(&self) -> String {
        let mut imei_or_imeisv = String::new();
        let first_digit = (self.0[0] >> 4) & 0x0F;
        imei_or_imeisv.push(char::from_digit(first_digit as u32, 10).unwrap_or('?'));
        for &byte in &self.0[1..] {
            let high_nibble = (byte >> 4) & 0x0F;
            imei_or_imeisv.push(char::from_digit(high_nibble as u32, 10).unwrap_or('?'));

            let low_nibble = byte & 0x0F;
            imei_or_imeisv.push(char::from_digit(low_nibble as u32, 10).unwrap_or('?'));
        }

        if !self.is_even_digit() {
            imei_or_imeisv.pop();
        }

        imei_or_imeisv
    }
}

impl ToString for ImeiOrImeiSv<'_> {
    fn to_string(&self) -> String {
        self.0.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }
}

// ******************************************************************
// FiveGsTmsi
// ******************************************************************

// Manually-generated
bitfield! {
    #[derive(Clone)]
    pub struct FiveGTmsi(MSB0 [u8]);
    impl Debug;
    u8;
    pub from into FiveGsIdentityType, get_identity_type, set_identity_type: 2, 0;
    pub get_amf_set_id, set_amf_set_id: 15, 8;
    pub get_amf_pointer, set_amf_pointer: 21, 16;
    pub get_amf_set_id_contd, set_amf_set_id_contd: 23, 22;
    pub u32, get_5g_tmsi, set_5g_tmsi: 55, 24;
}

// ******************************************************************
// MacAddress
// ******************************************************************

// Manually-generated
#[derive(Debug, Clone)]
pub struct MacAddress<'a>(&'a [u8]);

impl MacAddress<'_> {
    pub fn get_identity_type(&self) -> FiveGsIdentityType {
        FiveGsIdentityType(&self.0[0] & 0b00000111)
    }

    pub fn can_be_used_as_equip_identifier(&self) -> bool {
        (self.0[0] & 0b00001000) == 0
    }

    pub fn get_mac_addr_data(&self) -> &[u8] {
        &self.0[2..8]
    }
}

impl ToString for MacAddress<'_> {
    fn to_string(&self) -> String {
        let mac_data = self.get_mac_addr_data();
        format!(
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            mac_data[0], mac_data[1], mac_data[2],
            mac_data[3], mac_data[4], mac_data[5]
        )
    }
}

// ******************************************************************
// Eui64
// ******************************************************************

// Manually-generated
#[derive(Debug, Clone)]
pub struct Eui64<'a>(&'a [u8]);

impl Eui64<'_> {
    pub fn get_identity_type(&self) -> FiveGsIdentityType {
        FiveGsIdentityType(&self.0[0] & 0b00000111)
    }

    pub fn get_eui64_data(&self) -> &[u8] {
        &self.0[2..10]
    }
}

// ******************************************************************
// Suci
// ******************************************************************

// Manually-generated
#[derive(Debug, Clone)]
pub struct Suci<'a>(pub &'a [u8]);

impl Suci<'_> {
    pub fn get_identity_type(&self) -> FiveGsIdentityType {
        FiveGsIdentityType(self.0[0] & 0b00000111)
    }

    pub fn get_supi_format(&self) -> SupiFormat {
        let format_bits = self.0[0] & 0b01110000;
        match format_bits {
            0b000 => SupiFormat::Imsi,
            0b001 => SupiFormat::NetworkSpecificIdentifier,
            0b010 => SupiFormat::Gci,
            0b011 => SupiFormat::Gli,
            _ => SupiFormat::Imsi,
        }
    }

    pub fn get_suci_data(&self) -> SuciData {
        match self.get_supi_format() {
            SupiFormat::Imsi => {
                return SuciData::Imsi(Imsi(self.0));
            }
            SupiFormat::NetworkSpecificIdentifier => {
                return SuciData::Nsi(NsiOrGciOrGli(self.0));
            },
            SupiFormat::Gci => {
                return SuciData::Gci(NsiOrGciOrGli(self.0));
            },
            SupiFormat::Gli => {
                return SuciData::Gli(NsiOrGciOrGli(self.0));
            },
        }
    }

}

impl ToString for Suci<'_> {
    fn to_string(&self) -> String {
        self.0.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
    }
}

#[derive(Debug, Clone)]
pub enum SupiFormat {
    Imsi = 0b000,
    NetworkSpecificIdentifier = 0b001,
    Gci = 0b010,
    Gli = 0b011,
}

impl From<u8> for SupiFormat {
    fn from(value: u8) -> Self {
        match value {
            0b000 => SupiFormat::Imsi,
            0b001 => SupiFormat::NetworkSpecificIdentifier,
            0b010 => SupiFormat::Gci,
            0b011 => SupiFormat::Gli,
            _ => SupiFormat::Imsi,
        }
    }
}

impl From<SupiFormat> for u8 {
    fn from(format: SupiFormat) -> Self {
        format as u8
    }
}

#[derive(Debug, Clone)]
pub enum SuciData<'a> {
    Imsi(Imsi<'a>),
    Nsi(NsiOrGciOrGli<'a>),
    Gci(NsiOrGciOrGli<'a>),
    Gli(NsiOrGciOrGli<'a>),
}

#[derive(Debug, Clone)]
pub struct Imsi<'a>(&'a [u8]);

impl Imsi<'_> {
    pub fn get_imsi_header(&self) -> ImsiHeader<&[u8]> {
        ImsiHeader(&self.0[0..8])
    }

    pub fn get_msin(&self) -> String {
        let mut msin = String::new();

        for &byte in &self.0[8..] {
            let high_nibble = (byte >> 4) & 0x0F;
            if high_nibble != 0x0F {
                msin.push(char::from_digit(high_nibble as u32, 16).unwrap_or('?'));
            }

            let low_nibble = byte & 0x0F;
            if low_nibble != 0x0F {
                msin.push(char::from_digit(low_nibble as u32, 16).unwrap_or('?'));
            }
        }

        msin
    }
}

bitfield! {
    #[derive(Clone)]
    pub struct ImsiHeader(MSB0 [u8]);
    impl Debug;
    u8;
    pub from into FiveGsIdentityType, get_identity_type, set_identity_type: 2, 0;
    pub from into SupiFormat, get_supi_format, set_supi_format: 6, 4;
    pub get_mcc_digit_1, set_mcc_digit_1: 11, 8;
    pub get_mcc_digit_2, set_mcc_digit_2: 15, 12;
    pub get_mcc_digit_3, set_mcc_digit_3: 19, 16;
    pub get_mnc_digit_3, set_mnc_digit_3: 23, 20;
    pub get_mnc_digit_1, set_mnc_digit_1: 27, 24;
    pub get_mnc_digit_2, set_mnc_digit_2: 31, 28;
    pub get_routing_indicator_digit_1, set_routing_indicator_digit_1: 35, 32;
    pub get_routing_indicator_digit_2, set_routing_indicator_digit_2: 39, 36;
    pub get_routing_indicator_digit_3, set_routing_indicator_digit_3: 43, 40;
    pub get_routing_indicator_digit_4, set_routing_indicator_digit_4: 47, 44;
    pub get_protection_scheme_id, set_protection_scheme_id: 51, 48;
    pub get_home_network_pki_value, set_home_network_pki_value: 63, 56;
}

#[derive(Debug, Clone)]
pub struct NsiOrGciOrGli<'a>(&'a [u8]);

impl NsiOrGciOrGli<'_> {
    pub fn get_nsi_gci_gli_header(&self) -> NsiOrGciOrGliHeader {
        NsiOrGciOrGliHeader(self.0[0])
    }

    pub fn get_suci_nai(&self) -> String {
        let mut suci_nai = String::new();

        for &byte in &self.0[8..] {
            let high_nibble = (byte >> 4) & 0x0F;
            if high_nibble != 0x0F {
                suci_nai.push(char::from_digit(high_nibble as u32, 16).unwrap_or('?'));
            }

            let low_nibble = byte & 0x0F;
            if low_nibble != 0x0F {
                suci_nai.push(char::from_digit(low_nibble as u32, 16).unwrap_or('?'));
            }
        }

        suci_nai
    }
}

bitfield! {
    #[derive(Clone)]
    pub struct NsiOrGciOrGliHeader(u8);
    impl Debug;
    u8;
    pub from into FiveGsIdentityType, get_identity_type, set_identity_type: 2, 0;
    pub from into SupiFormat, get_supi_format, set_supi_format: 6, 4;
}


// ******************************************************************
// NoIdentity
// ******************************************************************

// Manually-generated
bitfield! {
    #[derive(Clone)]
    pub struct NoIdentity(MSB0 [u8]);
    impl Debug;
    u8;
    pub from into FiveGsIdentityType, get_identity_type, set_identity_type: 2, 0;
}
// ******************************************************************
// FiveGmmCapability
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FiveGmmCapability(Vec<u8>);

// ******************************************************************
// UeSecurityCapability
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeSecurityCapability(Vec<u8>);

impl UeSecurityCapability {
    pub fn new(ea_ia: (EA, IA), eea_eia: Option<(EEA, EIA)>, length: usize) -> Self {
        // Todo: find a better solution for this.
        let mut result = vec![0; length];
        result[0] = ea_ia.0 .0;
        result[1] = ea_ia.1 .0;
        if let Some((eea_val, eia_val)) = eea_eia {
            result[2] = eea_val.0;
            result[3] = eia_val.0;
        }
        Self(result)
    }

    pub fn get_ea_ia(&self) -> (EA, IA) {
        // safety: index out of bound is handled prior to this call in Tlv Decode
        (EA(self.0[0]), IA(self.0[1]))
    }

    pub fn get_eea_eia(&self) -> Option<(EEA, EIA)> {
        if self.0.len() >= 3 {
            return Some((EEA(self.0[2]), EIA(self.0[3])));
        }
        None
    }
}

impl ToString for UeSecurityCapability {
    fn to_string(&self) -> String {
        self.0.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
    }
}

bitfield! {
    #[derive(Clone)]
    pub struct EA(u8);
    impl Debug;
    impl new;
    u8;
    pub get_5g_ea0_supported, set_5g_ea0_supported: 7;
    pub get_128_5g_ea1_supported, set_128_5g_ea1_supported: 6;
    pub get_128_5g_ea2_supported, set_128_5g_ea2_supported: 5;
    pub get_128_5g_ea3_supported, set_128_5g_ea3_supported: 4;
    pub get_5g_ea4_supported, set_5g_ea4_supported: 3;
    pub get_5g_ea5_supported, set_5g_ea5_supported: 2;
    pub get_5g_ea6_supported, set_5g_ea6_supported: 1;
    pub get_5g_ea7_supported, set_5g_ea7_supported: 0;
}

bitfield! {
    #[derive(Clone)]
    pub struct IA(u8);
    impl Debug;
    impl new;
    u8;
    pub get_5g_ia0_supported, set_5g_ia0_supported: 7;
    pub get_128_5g_ia1_supported, set_128_5g_ia1_supported: 6;
    pub get_128_5g_ia2_supported, set_128_5g_ia2_supported: 5;
    pub get_128_5g_ia3_supported, set_128_5g_ia3_supported: 4;
    pub get_5g_ia4_supported, set_5g_ia4_supported: 3;
    pub get_5g_ia5_supported, set_5g_ia5_supported: 2;
    pub get_5g_ia6_supported, set_5g_ia6_supported: 1;
    pub get_5g_ia7_supported, set_5g_ia7_supported: 0;
}

bitfield! {
    #[derive(Clone)]
    pub struct EEA(u8);
    impl Debug;
    impl new;
    u8;
    pub get_eea0_supported, set_eea0_supported: 7;
    pub get_128_eea1_supported, set_128_eea1_supported: 6;
    pub get_128_eea2_supported, set_128_eea2_supported: 5;
    pub get_128_eea3_supported, set_128_eea3_supported: 4;
    pub get_eea4_supported, set_eea4_supported: 3;
    pub get_eea5_supported, set_eea5_supported: 2;
    pub get_eea6_supported, set_eea6_supported: 1;
    pub get_eea7_supported, set_eea7_supported: 0;
}

bitfield! {
    #[derive(Clone)]
    pub struct EIA(u8);
    impl Debug;
    impl new;
    u8;
    pub get_eia0_supported, set_eia0_supported: 7;
    pub get_128_eia1_supported, set_128_eia1_supported: 6;
    pub get_128_eia2_supported, set_128_eia2_supported: 5;
    pub get_128_eia3_supported, set_128_eia3_supported: 4;
    pub get_eia4_supported, set_eia4_supported: 3;
    pub get_eia5_supported, set_eia5_supported: 2;
    pub get_eia6_supported, set_eia6_supported: 1;
    pub get_eia7_supported, set_eia7_supported: 0;
}

// ******************************************************************
// Nssai
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Nssai(Vec<u8>);

// ******************************************************************
// FiveGsTrackingAreaIdentity
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FiveGsTrackingAreaIdentity(Vec<u8>);

// ******************************************************************
// S1UeNetworkCapability
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct S1UeNetworkCapability(Vec<u8>);

// ******************************************************************
// UplinkDataStatus
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UplinkDataStatus(Vec<u8>);

// ******************************************************************
// PduSessionStatus
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionStatus(Vec<u8>);

// ******************************************************************
// MicoIndication
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MicoIndication(u8);

// ******************************************************************
// UeStatus
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeStatus(u8);

// ******************************************************************
// AllowedPduSessionStatus
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AllowedPduSessionStatus(Vec<u8>);

// ******************************************************************
// UeUsageSetting
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeUsageSetting(u8);

// ******************************************************************
// FiveGsDrxParameters
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FiveGsDrxParameters(u8);

// ******************************************************************
// EpsNasMessageContainer
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct EpsNasMessageContainer(Vec<u8>);

// ******************************************************************
// LadnIndication
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct LadnIndication(Vec<u8>);

// ******************************************************************
// PayloadContainerType
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PayloadContainerType(u8);

// ******************************************************************
// PayloadContainer
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PayloadContainer(Vec<u8>);

// ******************************************************************
// NetworkSlicingIndication
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct NetworkSlicingIndication(u8);

// ******************************************************************
// FiveGsUpdateType
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FiveGsUpdateType(u8);

// ******************************************************************
// MobileStationClassmark2
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MobileStationClassmark2(Vec<u8>);

// ******************************************************************
// SupportedCodecList
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SupportedCodecList(Vec<u8>);

// ******************************************************************
// MessageContainer
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MessageContainer(Vec<u8>);

// ******************************************************************
// EpsBearerContextStatus
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct EpsBearerContextStatus(Vec<u8>);

// ******************************************************************
// ExtendedDrxParameters
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ExtendedDrxParameters(Vec<u8>);

// ******************************************************************
// GprsTimer3
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct GprsTimer3(u8);

// ******************************************************************
// UeRadioCapabilityId
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeRadioCapabilityId(Vec<u8>);

// ******************************************************************
// MappedNssai
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MappedNssai(Vec<u8>);

// ******************************************************************
// AdditionalInformationRequested
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AdditionalInformationRequested(u8);

// ******************************************************************
// WusAssistanceInformation
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct WusAssistanceInformation(Vec<u8>);

// ******************************************************************
// N5gcIndication
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct N5gcIndication(u8);

// ******************************************************************
// NbN1ModeDrxParameters
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct NbN1ModeDrxParameters(u8);

// ******************************************************************
// UeRequestType
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeRequestType(u8);

// ******************************************************************
// PagingRestriction
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PagingRestriction(Vec<u8>);

// ******************************************************************
// ServiceLevelAaContainer
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ServiceLevelAaContainer(Vec<u8>);

// ******************************************************************
// Nid
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Nid(Vec<u8>);

// ******************************************************************
// PlmnIdentity
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PlmnIdentity(Vec<u8>);

// ******************************************************************
// PeipsAssistanceInformation
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PeipsAssistanceInformation(Vec<u8>);

// ******************************************************************
// FiveGsRegistrationResult
// ******************************************************************

// Auto-generated
// #[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
// pub struct FiveGsRegistrationResult(u8);

bitfield! {
    #[derive(TlvEncode, TlvDecode, Into, From, Clone)]
    pub struct FiveGsRegistrationResult(u8);
    impl Debug;
    u8;
    pub get_raw_registration_result_value, set_raw_registration_result_value: 2, 0;
    pub get_raw_sms_allowed, set_raw_sms_allowed: 3;
    pub get_raw_nssa_performed, set_raw_nssa_performed: 4;
    pub get_raw_emergency_registered, set_raw_emergency_registered: 5;
    pub get_raw_disaster_roaming_registration_value, set_raw_disaster_roaming_registration_value: 6;
}

pub enum RegistrationResultValue {
    ThreeGppAccess = 0b001,
    NonThreeGppAccess = 0b010,
    ThreeGppAndNonThreeGppAccess = 0b011,
    Reserved = 0b111,
}

impl FiveGsRegistrationResult {
    pub fn new(registration_result: RegistrationResultValue) -> Self {
        let mut result = Self(0);
        result.set_raw_registration_result_value(registration_result as u8);
        result
    }

    pub fn get_registration_result_value(&self) -> Option<RegistrationResultValue> {
        match self.get_raw_registration_result_value() {
            0b001 => Some(RegistrationResultValue::ThreeGppAccess),
            0b010 => Some(RegistrationResultValue::NonThreeGppAccess),
            0b011 => Some(RegistrationResultValue::ThreeGppAndNonThreeGppAccess),
            0b111 => Some(RegistrationResultValue::Reserved),
            _ => Some(RegistrationResultValue::ThreeGppAccess), 
        }
    }

    pub fn set_registration_result_value(&mut self, registration_result: RegistrationResultValue) {
        self.set_raw_registration_result_value(registration_result as u8);
    }

    pub fn is_sms_allowed(&self) -> bool {
        self.get_raw_sms_allowed()
    }

    pub fn set_sms_allowed(&mut self, allowed: bool) {
        self.set_raw_sms_allowed(allowed);
    }

    pub fn is_nssaa_to_be_performed(&self) -> bool {
        self.get_raw_nssa_performed()
    }

    pub fn set_nssaa_to_be_performed(&mut self, to_be_performed: bool) {
        self.set_raw_nssa_performed(to_be_performed);
    }

    pub fn is_emergency_registered(&self) -> bool {
        self.get_raw_emergency_registered()
    }

    pub fn set_emergency_registered(&mut self, registered: bool) {
        self.set_raw_emergency_registered(registered);
    }

    pub fn is_disaster_roaming_accepted(&self) -> bool {
        self.get_raw_disaster_roaming_registration_value()
    }

    pub fn set_disaster_roaming_accepted(&mut self, accepted: bool) {
        self.set_raw_disaster_roaming_registration_value(accepted);
    }
}

// ******************************************************************
// PlmnList
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PlmnList(Vec<u8>);

// ******************************************************************
// FiveGsTrackingAreaIdentityList
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FiveGsTrackingAreaIdentityList(Vec<u8>);

// ******************************************************************
// RejectedNssai
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct RejectedNssai(Vec<u8>);

// ******************************************************************
// FiveGsNetworkFeatureSupport
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FiveGsNetworkFeatureSupport(Vec<u8>);

// ******************************************************************
// PduSessionReactivationResult
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionReactivationResult(Vec<u8>);

// ******************************************************************
// PduSessionReactivationResultErrorCause
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionReactivationResultErrorCause(Vec<u8>);

// ******************************************************************
// LadnInformation
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct LadnInformation(Vec<u8>);

// ******************************************************************
// ServiceAreaList
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ServiceAreaList(Vec<u8>);

// ******************************************************************
// GprsTimer2
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct GprsTimer2(u8);

// ******************************************************************
// EmergencyNumberList
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct EmergencyNumberList(Vec<u8>);

// ******************************************************************
// ExtendedEmergencyNumberList
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ExtendedEmergencyNumberList(Vec<u8>);

// ******************************************************************
// SorTransparentContainer
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SorTransparentContainer(Vec<u8>);

// ******************************************************************
// EapMessage
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct EapMessage(Vec<u8>);

// ******************************************************************
// NssaiInclusionMode
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct NssaiInclusionMode(u8);

// ******************************************************************
// OperatorDefinedAccessCategoryDefinitions
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct OperatorDefinedAccessCategoryDefinitions(Vec<u8>);

// ******************************************************************
// Non3gppNwProvidedPolicies
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Non3gppNwProvidedPolicies(u8);

// ******************************************************************
// UeRadioCapabilityIdDeletionIndication
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeRadioCapabilityIdDeletionIndication(u8);

// ******************************************************************
// CipheringKeyData
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct CipheringKeyData(Vec<u8>);

// ******************************************************************
// CagInformationList
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct CagInformationList(Vec<u8>);

// ******************************************************************
// Truncated5gSTmsiConfiguration
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Truncated5gSTmsiConfiguration(u8);

// ******************************************************************
// ExtendedRejectedNssai
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ExtendedRejectedNssai(Vec<u8>);

// ******************************************************************
// FiveGsAdditionalRequestResult
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FiveGsAdditionalRequestResult(u8);

// ******************************************************************
// NssrgInformation
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct NssrgInformation(Vec<u8>);

// ******************************************************************
// RegistrationWaitRange
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct RegistrationWaitRange(Vec<u8>);

// ******************************************************************
// ListOfPlmnsToBeUsedInDisasterCondition
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ListOfPlmnsToBeUsedInDisasterCondition(Vec<u8>);

// ******************************************************************
// ExtendedCagInformationList
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ExtendedCagInformationList(Vec<u8>);

// ******************************************************************
// NsagInformation
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct NsagInformation(Vec<u8>);

// ******************************************************************
// FiveGmmCause
// ******************************************************************

// Auto-generated
// #[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
// pub struct FiveGmmCause(u8);

bitfield! {
    #[derive(TlvEncode, TlvDecode, Into, From, Clone)]
    pub struct FiveGmmCause(u8);
    impl Debug;
    u8;
    pub get_raw_cause_value, set_raw_cause_value: 7, 0;
}

pub enum GmmCauseValue {
    IllegalUe = 0x03,
    PeiNotAccepted = 0x05,
    IllegalMe = 0x06,
    FiveGsServicesNotAllowed = 0x07,
    UeIdentityCannotBeDerivedByNetwork = 0x09,
    ImplicitlyDeregistered = 0x0A,
    PlmnNotAllowed = 0x0B,
    TrackingAreaNotAllowed = 0x0C,
    RoamingNotAllowedInThisTrackingArea = 0x0D,
    NoSuitableCellsInTrackingArea = 0x0F,
    MacFailure = 0x14,
    SynchFailure = 0x15,
    Congestion = 0x16,
    UeSecurityCapabilitiesMismatch = 0x17,
    SecurityModeRejectedUnspecified = 0x18,
    Non5gAuthenticationUnacceptable = 0x1A,
    N1ModeNotAllowed = 0x1B,
    RestrictedServiceArea = 0x1C,
    RedirectionToEpcRequired = 0x1F,
    IabNodeOperationNotAuthorized = 0x24,
    LadnNotAvailable = 0x2B,
    NoNetworkSlicesAvailable = 0x3E,
    MaximumNumberOfPduSessionsReached = 0x41,
    InsufficientResourcesForSpecificSliceAndDnn = 0x43,
    InsufficientResourcesForSpecificSlice = 0x45,
    NgKsiAlreadyInUse = 0x47,
    Non3gppAccessTo5gcnNotAllowed = 0x48,
    ServingNetworkNotAuthorized = 0x49,
    TemporarilyNotAuthorizedForThisSnpn = 0x4A,
    PermanentlyNotAuthorizedForThisSnpn = 0x4B,
    NotAuthorizedForThisCagOrAuthorizedForCagCellsOnly = 0x4C,
    WirelineAccessAreaNotAllowed = 0x4D,
    PlmnNotAllowedToOperateAtPresentUeLocation = 0x4E,
    UasServicesNotAllowed = 0x4F,
    DisasterRoamingForDeterminedPlmnWithDisasterConditionNotAllowed = 0x50,
    PayloadWasNotForwarded = 0x5A,
    DnnNotSupportedOrNotSubscribedInSlice = 0x5B,
    InsufficientUserPlaneResourcesForPduSession = 0x5C,
    OnboardingServicesTerminated = 0x5D,
    SemanticallyIncorrectMessage = 0x5F,
    InvalidMandatoryInformation = 0x60,
    MessageTypeNonExistentOrNotImplemented = 0x61,
    MessageTypeNotCompatibleWithProtocolState = 0x62,
    InformationElementNonExistentOrNotImplemented = 0x63,
    ConditionalIeError = 0x64,
    MessageNotCompatibleWithProtocolState = 0x65,
    ProtocolErrorUnspecified = 0x6F,
}

impl FiveGmmCause {
    pub const fn illegal_ue() -> Self {
        Self(0x03)
    }

    pub const fn pei_not_accepted() -> Self {
        Self(0x05)
    }

    pub const fn illegal_me() -> Self {
        Self(0x06)
    }

    pub const fn five_gs_services_not_allowed() -> Self {
        Self(0x07)
    }

    pub const fn ue_identity_cannot_be_derived_by_network() -> Self {
        Self(0x09)
    }

    pub const fn implicitly_deregistered() -> Self {
        Self(0x0A)
    }

    pub const fn plmn_not_allowed() -> Self {
        Self(0x0B)
    }

    pub const fn tracking_area_not_allowed() -> Self {
        Self(0x0C)
    }

    pub const fn roaming_not_allowed_in_this_tracking_area() -> Self {
        Self(0x0D)
    }

    pub const fn no_suitable_cells_in_tracking_area() -> Self {
        Self(0x0F)
    }

    pub const fn mac_failure() -> Self {
        Self(0x14)
    }

    pub const fn synch_failure() -> Self {
        Self(0x15)
    }

    pub const fn congestion() -> Self {
        Self(0x16)
    }

    pub const fn ue_security_capabilities_mismatch() -> Self {
        Self(0x17)
    }

    pub const fn security_mode_rejected_unspecified() -> Self {
        Self(0x18)
    }

    pub const fn non_5g_authentication_unacceptable() -> Self {
        Self(0x1A)
    }

    pub const fn n1_mode_not_allowed() -> Self {
        Self(0x1B)
    }

    pub const fn restricted_service_area() -> Self {
        Self(0x1C)
    }

    pub const fn redirection_to_epc_required() -> Self {
        Self(0x1F)
    }

    pub const fn iab_node_operation_not_authorized() -> Self {
        Self(0x24)
    }

    pub const fn ladn_not_available() -> Self {
        Self(0x2B)
    }

    pub const fn no_network_slices_available() -> Self {
        Self(0x3E)
    }

    pub const fn maximum_number_of_pdu_sessions_reached() -> Self {
        Self(0x41)
    }

    pub const fn insufficient_resources_for_specific_slice_and_dnn() -> Self {
        Self(0x43)
    }

    pub const fn insufficient_resources_for_specific_slice() -> Self {
        Self(0x45)
    }

    pub const fn ng_ksi_already_in_use() -> Self {
        Self(0x47)
    }

    pub const fn non_3gpp_access_to_5gcn_not_allowed() -> Self {
        Self(0x48)
    }

    pub const fn serving_network_not_authorized() -> Self {
        Self(0x49)
    }

    pub const fn temporarily_not_authorized_for_this_snpn() -> Self {
        Self(0x4A)
    }

    pub const fn permanently_not_authorized_for_this_snpn() -> Self {
        Self(0x4B)
    }

    pub const fn not_authorized_for_this_cag_or_authorized_for_cag_cells_only() -> Self {
        Self(0x4C)
    }

    pub const fn wireline_access_area_not_allowed() -> Self {
        Self(0x4D)
    }

    pub const fn plmn_not_allowed_to_operate_at_present_ue_location() -> Self {
        Self(0x4E)
    }

    pub const fn uas_services_not_allowed() -> Self {
        Self(0x4F)
    }

    pub const fn disaster_roaming_for_determined_plmn_with_disaster_condition_not_allowed() -> Self {
        Self(0x50)
    }

    pub const fn payload_was_not_forwarded() -> Self {
        Self(0x5A)
    }

    pub const fn dnn_not_supported_or_not_subscribed_in_slice() -> Self {
        Self(0x5B)
    }

    pub const fn insufficient_user_plane_resources_for_pdu_session() -> Self {
        Self(0x5C)
    }

    pub const fn onboarding_services_terminated() -> Self {
        Self(0x5D)
    }

    pub const fn semantically_incorrect_message() -> Self {
        Self(0x5F)
    }

    pub const fn invalid_mandatory_information() -> Self {
        Self(0x60)
    }

    pub const fn message_type_non_existent_or_not_implemented() -> Self {
        Self(0x61)
    }

    pub const fn message_type_not_compatible_with_protocol_state() -> Self {
        Self(0x62)
    }

    pub const fn information_element_non_existent_or_not_implemented() -> Self {
        Self(0x63)
    }

    pub const fn conditional_ie_error() -> Self {
        Self(0x64)
    }

    pub const fn message_not_compatible_with_protocol_state() -> Self {
        Self(0x65)
    }

    pub const fn protocol_error_unspecified() -> Self {
        Self(0x6F)
    }
}

impl FiveGmmCause {
    pub fn new(cause_value: GmmCauseValue) -> Self {
        Self(cause_value as u8)
    }

    pub fn get_cause_value(&self) -> GmmCauseValue {
        match self.get_raw_cause_value() {
            0x03 => GmmCauseValue::IllegalUe,
            0x05 => GmmCauseValue::PeiNotAccepted,
            0x06 => GmmCauseValue::IllegalMe,
            0x07 => GmmCauseValue::FiveGsServicesNotAllowed,
            0x09 => GmmCauseValue::UeIdentityCannotBeDerivedByNetwork,
            0x0A => GmmCauseValue::ImplicitlyDeregistered,
            0x0B => GmmCauseValue::PlmnNotAllowed,
            0x0C => GmmCauseValue::TrackingAreaNotAllowed,
            0x0D => GmmCauseValue::RoamingNotAllowedInThisTrackingArea,
            0x0F => GmmCauseValue::NoSuitableCellsInTrackingArea,
            0x14 => GmmCauseValue::MacFailure,
            0x15 => GmmCauseValue::SynchFailure,
            0x16 => GmmCauseValue::Congestion,
            0x17 => GmmCauseValue::UeSecurityCapabilitiesMismatch,
            0x18 => GmmCauseValue::SecurityModeRejectedUnspecified,
            0x1A => GmmCauseValue::Non5gAuthenticationUnacceptable,
            0x1B => GmmCauseValue::N1ModeNotAllowed,
            0x1C => GmmCauseValue::RestrictedServiceArea,
            0x1F => GmmCauseValue::RedirectionToEpcRequired,
            0x24 => GmmCauseValue::IabNodeOperationNotAuthorized,
            0x2B => GmmCauseValue::LadnNotAvailable,
            0x3E => GmmCauseValue::NoNetworkSlicesAvailable,
            0x41 => GmmCauseValue::MaximumNumberOfPduSessionsReached,
            0x43 => GmmCauseValue::InsufficientResourcesForSpecificSliceAndDnn,
            0x45 => GmmCauseValue::InsufficientResourcesForSpecificSlice,
            0x47 => GmmCauseValue::NgKsiAlreadyInUse,
            0x48 => GmmCauseValue::Non3gppAccessTo5gcnNotAllowed,
            0x49 => GmmCauseValue::ServingNetworkNotAuthorized,
            0x4A => GmmCauseValue::TemporarilyNotAuthorizedForThisSnpn,
            0x4B => GmmCauseValue::PermanentlyNotAuthorizedForThisSnpn,
            0x4C => GmmCauseValue::NotAuthorizedForThisCagOrAuthorizedForCagCellsOnly,
            0x4D => GmmCauseValue::WirelineAccessAreaNotAllowed,
            0x4E => GmmCauseValue::PlmnNotAllowedToOperateAtPresentUeLocation,
            0x4F => GmmCauseValue::UasServicesNotAllowed,
            0x50 => GmmCauseValue::DisasterRoamingForDeterminedPlmnWithDisasterConditionNotAllowed,
            0x5A => GmmCauseValue::PayloadWasNotForwarded,
            0x5B => GmmCauseValue::DnnNotSupportedOrNotSubscribedInSlice,
            0x5C => GmmCauseValue::InsufficientUserPlaneResourcesForPduSession,
            0x5D => GmmCauseValue::OnboardingServicesTerminated,
            0x5F => GmmCauseValue::SemanticallyIncorrectMessage,
            0x60 => GmmCauseValue::InvalidMandatoryInformation,
            0x61 => GmmCauseValue::MessageTypeNonExistentOrNotImplemented,
            0x62 => GmmCauseValue::MessageTypeNotCompatibleWithProtocolState,
            0x63 => GmmCauseValue::InformationElementNonExistentOrNotImplemented,
            0x64 => GmmCauseValue::ConditionalIeError,
            0x65 => GmmCauseValue::MessageNotCompatibleWithProtocolState,
            0x6F => GmmCauseValue::ProtocolErrorUnspecified,
            _ => GmmCauseValue::ProtocolErrorUnspecified,
        }
    }

    pub fn set_cause_value(&mut self, cause_value: GmmCauseValue) {
        self.set_raw_cause_value(cause_value as u8);
    }
}

// ******************************************************************
// DeRegistrationType
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct DeRegistrationType(u8);

// ******************************************************************
// ServiceType
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ServiceType(u8);

// ******************************************************************
// ConfigurationUpdateIndication
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ConfigurationUpdateIndication(u8);

// ******************************************************************
// NetworkName
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct NetworkName(Vec<u8>);

// ******************************************************************
// TimeZone
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct TimeZone(u8);

// ******************************************************************
// TimeZoneAndTime
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct TimeZoneAndTime(Vec<u8>);

// ******************************************************************
// DaylightSavingTime
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct DaylightSavingTime(u8);

// ******************************************************************
// SmsIndication
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SmsIndication(u8);

// ******************************************************************
// AdditionalConfigurationIndication
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AdditionalConfigurationIndication(u8);

// ******************************************************************
// PriorityIndicator
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PriorityIndicator(u8);

// ******************************************************************
// Abba
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Abba(pub Vec<u8>);

// ******************************************************************
// AuthenticationParameterRand
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AuthenticationParameterRand(Vec<u8>);

// ******************************************************************
// AuthenticationParameterAutn
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AuthenticationParameterAutn(Vec<u8>);

// ******************************************************************
// AuthenticationResponseParameter
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AuthenticationResponseParameter(Vec<u8>);

// ******************************************************************
// AuthenticationFailureParameter
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AuthenticationFailureParameter(Vec<u8>);

// ******************************************************************
// FiveGsIdentityType
// ******************************************************************

// Auto-generated
// #[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
// pub struct FiveGsIdentityType(u8);

bitfield! {
    #[derive(TlvEncode, TlvDecode, Into, From, Clone)]
    pub struct FiveGsIdentityType(u8);
    impl Debug;
    u8;
    pub get_raw_identity_type, set_raw_identity_type: 2, 0;
}

pub enum IdentityType {
    NoIdentity = 0b000,
    Suci = 0b001,
    FiveGGuti = 0b010,
    Imei = 0b011,
    FiveGSTmsi = 0b100,
    Imeisv = 0b101,
    MacAddress = 0b110,
    Eui64 = 0b111,
}

impl FiveGsIdentityType {
    pub fn new(identity_type: IdentityType) -> Self {
        Self(identity_type as u8)
    }

    pub fn get_identity_type(&self) -> IdentityType {
        match self.get_raw_identity_type() {
            0b000 => IdentityType::NoIdentity,
            0b001 => IdentityType::Suci,
            0b010 => IdentityType::FiveGGuti,
            0b011 => IdentityType::Imei,
            0b100 => IdentityType::FiveGSTmsi,
            0b101 => IdentityType::Imeisv,
            0b110 => IdentityType::MacAddress,
            0b111 => IdentityType::Eui64,
            _ => IdentityType::Suci,
        }
    }

    pub fn set_identity_type_enum(&mut self, identity_type: IdentityType) {
        self.set_raw_identity_type(identity_type as u8);
    }
}

// ******************************************************************
// SecurityAlgorithms
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SecurityAlgorithms(u8);

// ******************************************************************
// ImeisvRequest
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ImeisvRequest(u8);

// ******************************************************************
// EpsNasSecurityAlgorithms
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct EpsNasSecurityAlgorithms(u8);

// ******************************************************************
// Additional5gSecurityInformation
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Additional5gSecurityInformation(u8);

// ******************************************************************
// S1UeSecurityCapability
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct S1UeSecurityCapability(Vec<u8>);

// ******************************************************************
// AccessType
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AccessType(u8);

// ******************************************************************
// PduSessionIdentity2
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionIdentity2(u8);

// ******************************************************************
// RequestType
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct RequestType(u8);

// ******************************************************************
// SNssai
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SNssai(Vec<u8>);

// ******************************************************************
// Dnn
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Dnn(Vec<u8>);

// ******************************************************************
// AdditionalInformation
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AdditionalInformation(Vec<u8>);

// ******************************************************************
// MaPduSessionInformation
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MaPduSessionInformation(u8);

// ******************************************************************
// ReleaseAssistanceIndication
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ReleaseAssistanceIndication(u8);

// ******************************************************************
// PduSessionIdentity
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionIdentity(u8);

// ******************************************************************
// ProcedureTransactionIdentity
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ProcedureTransactionIdentity(u8);

// ******************************************************************
// IntegrityProtectionMaximumDataRate
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct IntegrityProtectionMaximumDataRate(Vec<u8>);

// ******************************************************************
// PduSessionType
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionType(u8);

// ******************************************************************
// SscMode
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SscMode(u8);

// ******************************************************************
// FiveGsmCapability
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FiveGsmCapability(Vec<u8>);

// ******************************************************************
// MaximumNumberOfSupportedPacketFilters
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MaximumNumberOfSupportedPacketFilters(Vec<u8>);

// ******************************************************************
// AlwaysOnPduSessionRequested
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AlwaysOnPduSessionRequested(u8);

// ******************************************************************
// SmPduDnRequestContainer
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SmPduDnRequestContainer(Vec<u8>);

// ******************************************************************
// ExtendedProtocolConfigurationOptions
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ExtendedProtocolConfigurationOptions(Vec<u8>);

// ******************************************************************
// IpHeaderCompressionConfiguration
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct IpHeaderCompressionConfiguration(Vec<u8>);

// ******************************************************************
// DsTtEthernetPortMacAddress
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct DsTtEthernetPortMacAddress(Vec<u8>);

// ******************************************************************
// UeDsTtResidenceTime
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeDsTtResidenceTime(Vec<u8>);

// ******************************************************************
// PortManagementInformationContainer
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PortManagementInformationContainer(Vec<u8>);

// ******************************************************************
// EthernetHeaderCompressionConfiguration
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct EthernetHeaderCompressionConfiguration(u8);

// ******************************************************************
// PduAddress
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduAddress(Vec<u8>);

// ******************************************************************
// RequestedMbsContainer
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct RequestedMbsContainer(Vec<u8>);

// ******************************************************************
// PduSessionPairId
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionPairId(u8);

// ******************************************************************
// Rsn
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Rsn(u8);

// ******************************************************************
// QosRules
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct QosRules(Vec<u8>);

// ******************************************************************
// SessionAmbr
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SessionAmbr(Vec<u8>);

// ******************************************************************
// FiveGsmCause
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FiveGsmCause(u8);

// ******************************************************************
// GprsTimer
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct GprsTimer(u8);

// ******************************************************************
// AlwaysOnPduSessionIndication
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AlwaysOnPduSessionIndication(u8);

// ******************************************************************
// MappedEpsBearerContexts
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MappedEpsBearerContexts(Vec<u8>);

// ******************************************************************
// QosFlowDescriptions
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct QosFlowDescriptions(Vec<u8>);

// ******************************************************************
// FiveGsmNetworkFeatureSupport
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FiveGsmNetworkFeatureSupport(Vec<u8>);

// ******************************************************************
// ServingPlmnRateControl
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ServingPlmnRateControl(Vec<u8>);

// ******************************************************************
// AtsssContainer
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AtsssContainer(Vec<u8>);

// ******************************************************************
// ControlPlaneOnlyIndication
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ControlPlaneOnlyIndication(u8);

// ******************************************************************
// ReceivedMbsContainer
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ReceivedMbsContainer(Vec<u8>);

// ******************************************************************
// AllowedSscMode
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AllowedSscMode(u8);

// ******************************************************************
// FiveGsmCongestionReAttemptIndicator
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FiveGsmCongestionReAttemptIndicator(u8);

// ******************************************************************
// ReAttemptIndicator
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ReAttemptIndicator(u8);

// ******************************************************************
// HeaderCompressionConfiguration
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct HeaderCompressionConfiguration(Vec<u8>);

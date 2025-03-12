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


use bitfield::bitfield;
use tlv::prelude::*;
use tlv_derive::{TlvDecode, TlvEncode};
use derive_more::{Into, From};


// ******************************************************************
// ExtendedProtocolDiscriminator
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ExtendedProtocolDiscriminator(u8);


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


// ******************************************************************
// MessageType
// ******************************************************************

// Auto-generated
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MessageType(u8);


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
    pub fn new(security_context_type: SecurityContextType, nas_key_set_identifier_value: NasKeySetIdentifierValue) -> KeySetIdentifier {
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
            },
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
pub struct FiveGsMobileIdentity(Vec<u8>);

impl FiveGsMobileIdentity {

    pub fn new() -> Self {
        todo!()
    }

    pub fn get_mobile_identity_type(&self) -> Option<IdentityType> {
        FiveGsIdentityType(self.0.first()? & 0b00000111).get_identity_type()
    }

    pub fn get_mobile_identity(self) -> Option<MobileIdentity> {
        match self.get_mobile_identity_type()? {
            IdentityType::Suci => todo!(),
            IdentityType::FiveGGuti => {
                Some(MobileIdentity::FiveGGuti(FiveGGuti(self.0)))
            },
            IdentityType::Imei => todo!(),
            IdentityType::FiveGSTmsi => todo!(),
            IdentityType::Imeisv => todo!(),
            IdentityType::MacAddress => todo!(),
            IdentityType::Eui64 => todo!(),
        }
    }

    pub fn set_mobile_identity(&mut self, identity: MobileIdentity) {
        match identity {
            MobileIdentity::Suci => todo!(),
            MobileIdentity::FiveGGuti(guti) => {
                self.0 = guti.0;
            },
            MobileIdentity::Imei => todo!(),
            MobileIdentity::FiveGSTmsi => todo!(),
            MobileIdentity::Imeisv => todo!(),
            MobileIdentity::MacAddress => todo!(),
            MobileIdentity::Eui64 => todo!(),
        }
    }
}

pub enum MobileIdentity {
    Suci,
    FiveGGuti(Guti),
    Imei,
    FiveGSTmsi,
    Imeisv,
    MacAddress,
    Eui64,
}

// ******************************************************************
// FiveGGUTI
// ******************************************************************

// Manually-generated
pub type Guti = FiveGGuti<Vec<u8>>;
pub type Tmsi = u32;

bitfield! {
    #[derive(Clone)]
    pub struct FiveGGuti(MSB0 [u8]);
    impl Debug;
    u8;
    pub from into FiveGsIdentityType, get_identity_type, set_identity_type: 2, 0;
    pub get_mcc_digit_1, set_mcc_digit_1: 11, 8;
    pub get_mcc_digit_2, set_mcc_digit_2: 15, 12;
    pub get_mcc_digit_3, set_mcc_digit_3: 19, 16;
    pub get_mnc_digit_1, set_mnc_digit_1: 23, 20;
    pub get_mnc_digit_2, set_mnc_digit_2: 27, 24;
    pub get_mnc_digit_3, set_mnc_digit_3: 31, 28;
    pub get_amf_region_id, set_amf_region_id: 39, 32;
    pub get_amf_set_id, set_amf_set_id: 47, 40;
    pub get_amf_pointer, set_amf_pointer: 53, 48;
    pub get_amf_set_id_contd, set_amf_set_id_contd: 55, 54;
    pub u32, get_5g_tmsi, set_5g_tmsi: 87, 56;
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
        result[0] = ea_ia.0.0;
        result[1] = ea_ia.1.0;
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
            return Some((EEA(self.0[2]), EIA(self.0[3])))
        }
        None
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
#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FiveGsRegistrationResult(u8);


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
pub struct Abba(Vec<u8>);


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

    pub fn get_identity_type(&self) -> Option<IdentityType> {
        match self.get_raw_identity_type() {
            0b001 => Some(IdentityType::Suci),
            0b010 => Some(IdentityType::FiveGGuti),
            0b011 => Some(IdentityType::Imei),
            0b100 => Some(IdentityType::FiveGSTmsi),
            0b101 => Some(IdentityType::Imeisv),
            0b110 => Some(IdentityType::MacAddress),
            0b111 => Some(IdentityType::Eui64),
            _ => None, // Handle invalid values
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

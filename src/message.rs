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
 * Created on: 2025-03-11 22:49:24.620096 by nr
 * from 24501-h90.docx
 ******************************************************************************/


use tlv::prelude::*;
use crate::types::*;
use tlv_derive::{TlvDecode, TlvEncode};


pub const NAS_EXTENDED_PROTOCOL_DISCRIMINATOR_5GSM: u8 = 0x2e;
pub const NAS_EXTENDED_PROTOCOL_DISCRIMINATOR_5GMM: u8 = 0x7e;

pub const NAS_MESSAGE_TYPE_REGISTRATION_REQUEST: u8 = 65;
pub const NAS_MESSAGE_TYPE_REGISTRATION_ACCEPT: u8 = 66;
pub const NAS_MESSAGE_TYPE_REGISTRATION_COMPLETE: u8 = 67;
pub const NAS_MESSAGE_TYPE_REGISTRATION_REJECT: u8 = 68;
pub const NAS_MESSAGE_TYPE_DEREGISTRATION_REQUEST_FROM_UE: u8 = 69;
pub const NAS_MESSAGE_TYPE_DEREGISTRATION_ACCEPT_FROM_UE: u8 = 70;
pub const NAS_MESSAGE_TYPE_DEREGISTRATION_REQUEST_TO_UE: u8 = 71;
pub const NAS_MESSAGE_TYPE_DEREGISTRATION_ACCEPT_TO_UE: u8 = 72;
pub const NAS_MESSAGE_TYPE_SERVICE_REQUEST: u8 = 76;
pub const NAS_MESSAGE_TYPE_SERVICE_REJECT: u8 = 77;
pub const NAS_MESSAGE_TYPE_SERVICE_ACCEPT: u8 = 78;
pub const NAS_MESSAGE_TYPE_CONFIGURATION_UPDATE_COMMAND: u8 = 84;
pub const NAS_MESSAGE_TYPE_CONFIGURATION_UPDATE_COMPLETE: u8 = 85;
pub const NAS_MESSAGE_TYPE_AUTHENTICATION_REQUEST: u8 = 86;
pub const NAS_MESSAGE_TYPE_AUTHENTICATION_RESPONSE: u8 = 87;
pub const NAS_MESSAGE_TYPE_AUTHENTICATION_REJECT: u8 = 88;
pub const NAS_MESSAGE_TYPE_AUTHENTICATION_FAILURE: u8 = 89;
pub const NAS_MESSAGE_TYPE_AUTHENTICATION_RESULT: u8 = 90;
pub const NAS_MESSAGE_TYPE_IDENTITY_REQUEST: u8 = 91;
pub const NAS_MESSAGE_TYPE_IDENTITY_RESPONSE: u8 = 92;
pub const NAS_MESSAGE_TYPE_SECURITY_MODE_COMMAND: u8 = 93;
pub const NAS_MESSAGE_TYPE_SECURITY_MODE_COMPLETE: u8 = 94;
pub const NAS_MESSAGE_TYPE_SECURITY_MODE_REJECT: u8 = 95;
pub const NAS_MESSAGE_TYPE_5GMM_STATUS: u8 = 100;
pub const NAS_MESSAGE_TYPE_NOTIFICATION: u8 = 101;
pub const NAS_MESSAGE_TYPE_NOTIFICATION_RESPONSE: u8 = 102;
pub const NAS_MESSAGE_TYPE_UL_NAS_TRANSPORT: u8 = 103;
pub const NAS_MESSAGE_TYPE_DL_NAS_TRANSPORT: u8 = 104;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_ESTABLISHMENT_REQUEST: u8 = 193;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_ESTABLISHMENT_ACCEPT: u8 = 194;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_ESTABLISHMENT_REJECT: u8 = 195;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_AUTHENTICATION_COMMAND: u8 = 197;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_AUTHENTICATION_COMPLETE: u8 = 198;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_AUTHENTICATION_RESULT: u8 = 199;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_REQUEST: u8 = 201;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_REJECT: u8 = 202;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_COMMAND: u8 = 203;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_COMPLETE: u8 = 204;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_MODIFICATION_COMMAND_REJECT: u8 = 205;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_RELEASE_REQUEST: u8 = 209;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_RELEASE_REJECT: u8 = 210;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_RELEASE_COMMAND: u8 = 211;
pub const NAS_MESSAGE_TYPE_PDU_SESSION_RELEASE_COMPLETE: u8 = 212;
pub const NAS_MESSAGE_TYPE_5GSM_STATUS: u8 = 214;


/*******************************************************
 * REGISTRATION REQUEST
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasRegistrationRequest {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_registration_request_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_5gs_registration_type: FiveGsRegistrationType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_ngksi: KeySetIdentifier,

    #[tlv_config(tag_bytes_format = 0, min_length = 4, length_bytes_format = 2, format = "LV-E")]
    pub nas_5gs_mobile_identity: FiveGsMobileIdentity,


    /* Optional fields */
    #[tlv_config(tag = 0xC, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_non_current_native_nas_key_set_identifier: Option<KeySetIdentifier>,

    #[tlv_config(tag = 0x10, tag_bytes_format = 1, min_length = 1, max_length = 13, length_bytes_format = 1, format = "TLV")]
    pub nas_5gmm_capability: Option<FiveGmmCapability>,

    #[tlv_config(tag = 0x2E, tag_bytes_format = 1, min_length = 2, max_length = 8, length_bytes_format = 1, format = "TLV")]
    pub nas_ue_security_capability: Option<UeSecurityCapability>,

    #[tlv_config(tag = 0x2F, tag_bytes_format = 1, min_length = 2, max_length = 72, length_bytes_format = 1, format = "TLV")]
    pub nas_requested_nssai: Option<Nssai>,

    #[tlv_config(tag = 0x52, tag_bytes_format = 1, length = 6, length_bytes_format = 0, format = "TV")]
    pub nas_last_visited_registered_tai: Option<FiveGsTrackingAreaIdentity>,

    #[tlv_config(tag = 0x17, tag_bytes_format = 1, min_length = 2, max_length = 13, length_bytes_format = 1, format = "TLV")]
    pub nas_s1_ue_network_capability: Option<S1UeNetworkCapability>,

    #[tlv_config(tag = 0x40, tag_bytes_format = 1, min_length = 2, max_length = 32, length_bytes_format = 1, format = "TLV")]
    pub nas_uplink_data_status: Option<UplinkDataStatus>,

    #[tlv_config(tag = 0x50, tag_bytes_format = 1, min_length = 2, max_length = 32, length_bytes_format = 1, format = "TLV")]
    pub nas_pdu_session_status: Option<PduSessionStatus>,

    #[tlv_config(tag = 0xB, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_mico_indication: Option<MicoIndication>,

    #[tlv_config(tag = 0x2B, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_ue_status: Option<UeStatus>,

    #[tlv_config(tag = 0x77, tag_bytes_format = 1, length = 11, length_bytes_format = 2, format = "TLV-E")]
    pub nas_additional_guti: Option<FiveGsMobileIdentity>,

    #[tlv_config(tag = 0x25, tag_bytes_format = 1, min_length = 2, max_length = 32, length_bytes_format = 1, format = "TLV")]
    pub nas_allowed_pdu_session_status: Option<AllowedPduSessionStatus>,

    #[tlv_config(tag = 0x18, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_ue_usage_setting: Option<UeUsageSetting>,

    #[tlv_config(tag = 0x51, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_requested_drx_parameters: Option<FiveGsDrxParameters>,

    #[tlv_config(tag = 0x70, tag_bytes_format = 1, min_length = 1, length_bytes_format = 2, format = "TLV-E")]
    pub nas_eps_nas_message_container: Option<EpsNasMessageContainer>,

    #[tlv_config(tag = 0x74, tag_bytes_format = 1, min_length = 0, max_length = 808, length_bytes_format = 2, format = "TLV-E")]
    pub nas_ladn_indication: Option<LadnIndication>,

    #[tlv_config(tag = 0x8, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_payload_container_type: Option<PayloadContainerType>,

    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_payload_container: Option<PayloadContainer>,

    #[tlv_config(tag = 0x9, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_network_slicing_indication: Option<NetworkSlicingIndication>,

    #[tlv_config(tag = 0x53, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_5gs_update_type: Option<FiveGsUpdateType>,

    #[tlv_config(tag = 0x41, tag_bytes_format = 1, length = 3, length_bytes_format = 1, format = "TLV")]
    pub nas_mobile_station_classmark_2: Option<MobileStationClassmark2>,

    #[tlv_config(tag = 0x42, tag_bytes_format = 1, min_length = 3, length_bytes_format = 1, format = "TLV")]
    pub nas_supported_codecs: Option<SupportedCodecList>,

    #[tlv_config(tag = 0x71, tag_bytes_format = 1, min_length = 1, length_bytes_format = 2, format = "TLV-E")]
    pub nas_nas_message_container: Option<MessageContainer>,

    #[tlv_config(tag = 0x60, tag_bytes_format = 1, length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_eps_bearer_context_status: Option<EpsBearerContextStatus>,

    #[tlv_config(tag = 0x6E, tag_bytes_format = 1, min_length = 1, max_length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_requested_extended_drx_parameters: Option<ExtendedDrxParameters>,

    #[tlv_config(tag = 0x6A, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_t3324_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x67, tag_bytes_format = 1, min_length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_ue_radio_capability_id: Option<UeRadioCapabilityId>,

    #[tlv_config(tag = 0x35, tag_bytes_format = 1, min_length = 1, max_length = 40, length_bytes_format = 1, format = "TLV")]
    pub nas_requested_mapped_nssai: Option<MappedNssai>,

    #[tlv_config(tag = 0x48, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_additional_information_requested: Option<AdditionalInformationRequested>,

    #[tlv_config(tag = 0x1A, tag_bytes_format = 1, min_length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_requested_wus_assistance_information: Option<WusAssistanceInformation>,

    #[tlv_config(tag = 0xA, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_n5gc_indication: Option<N5gcIndication>,

    #[tlv_config(tag = 0x30, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_requested_nb_n1_mode_drx_parameters: Option<NbN1ModeDrxParameters>,

    #[tlv_config(tag = 0x29, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_ue_request_type: Option<UeRequestType>,

    #[tlv_config(tag = 0x28, tag_bytes_format = 1, min_length = 1, max_length = 33, length_bytes_format = 1, format = "TLV")]
    pub nas_paging_restriction: Option<PagingRestriction>,

    #[tlv_config(tag = 0x72, tag_bytes_format = 1, min_length = 3, length_bytes_format = 2, format = "TLV-E")]
    pub nas_service_level_aa_container: Option<ServiceLevelAaContainer>,

    #[tlv_config(tag = 0x32, tag_bytes_format = 1, length = 6, length_bytes_format = 1, format = "TLV")]
    pub nas_nid: Option<Nid>,

    #[tlv_config(tag = 0x16, tag_bytes_format = 1, length = 3, length_bytes_format = 1, format = "TLV")]
    pub nas_ms_determined_plmn_with_disaster_condition: Option<PlmnIdentity>,

    #[tlv_config(tag = 0x2A, tag_bytes_format = 1, min_length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_requested_peips_assistance_information: Option<PeipsAssistanceInformation>,

    #[tlv_config(tag = 0x3B, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_requested_t3512_value: Option<GprsTimer3>,

} 


/*******************************************************
 * REGISTRATION ACCEPT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasRegistrationAccept {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_registration_accept_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 1, format = "LV")]
    pub nas_5gs_registration_result: FiveGsRegistrationResult,


    /* Optional fields */
    #[tlv_config(tag = 0x77, tag_bytes_format = 1, length = 11, length_bytes_format = 2, format = "TLV-E")]
    pub nas_5g_guti: Option<FiveGsMobileIdentity>,

    #[tlv_config(tag = 0x4A, tag_bytes_format = 1, min_length = 3, max_length = 45, length_bytes_format = 1, format = "TLV")]
    pub nas_equivalent_plmns: Option<PlmnList>,

    #[tlv_config(tag = 0x54, tag_bytes_format = 1, min_length = 7, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_tai_list: Option<FiveGsTrackingAreaIdentityList>,

    #[tlv_config(tag = 0x15, tag_bytes_format = 1, min_length = 2, max_length = 72, length_bytes_format = 1, format = "TLV")]
    pub nas_allowed_nssai: Option<Nssai>,

    #[tlv_config(tag = 0x11, tag_bytes_format = 1, min_length = 2, max_length = 40, length_bytes_format = 1, format = "TLV")]
    pub nas_rejected_nssai: Option<RejectedNssai>,

    #[tlv_config(tag = 0x31, tag_bytes_format = 1, min_length = 2, max_length = 144, length_bytes_format = 1, format = "TLV")]
    pub nas_configured_nssai: Option<Nssai>,

    #[tlv_config(tag = 0x21, tag_bytes_format = 1, min_length = 1, max_length = 3, length_bytes_format = 1, format = "TLV")]
    pub nas_5gs_network_feature_support: Option<FiveGsNetworkFeatureSupport>,

    #[tlv_config(tag = 0x50, tag_bytes_format = 1, min_length = 2, max_length = 32, length_bytes_format = 1, format = "TLV")]
    pub nas_pdu_session_status: Option<PduSessionStatus>,

    #[tlv_config(tag = 0x26, tag_bytes_format = 1, min_length = 2, max_length = 32, length_bytes_format = 1, format = "TLV")]
    pub nas_pdu_session_reactivation_result: Option<PduSessionReactivationResult>,

    #[tlv_config(tag = 0x72, tag_bytes_format = 1, min_length = 2, max_length = 512, length_bytes_format = 2, format = "TLV-E")]
    pub nas_pdu_session_reactivation_result_error_cause: Option<PduSessionReactivationResultErrorCause>,

    #[tlv_config(tag = 0x79, tag_bytes_format = 1, min_length = 9, max_length = 1712, length_bytes_format = 2, format = "TLV-E")]
    pub nas_ladn_information: Option<LadnInformation>,

    #[tlv_config(tag = 0xB, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_mico_indication: Option<MicoIndication>,

    #[tlv_config(tag = 0x9, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_network_slicing_indication: Option<NetworkSlicingIndication>,

    #[tlv_config(tag = 0x27, tag_bytes_format = 1, min_length = 4, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_service_area_list: Option<ServiceAreaList>,

    #[tlv_config(tag = 0x5E, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_t3512_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x5D, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_non_3gpp_de_registration_timer_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x16, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_t3502_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x34, tag_bytes_format = 1, min_length = 3, max_length = 48, length_bytes_format = 1, format = "TLV")]
    pub nas_emergency_number_list: Option<EmergencyNumberList>,

    #[tlv_config(tag = 0x7A, tag_bytes_format = 1, min_length = 4, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_emergency_number_list: Option<ExtendedEmergencyNumberList>,

    #[tlv_config(tag = 0x73, tag_bytes_format = 1, min_length = 17, length_bytes_format = 2, format = "TLV-E")]
    pub nas_sor_transparent_container: Option<SorTransparentContainer>,

    #[tlv_config(tag = 0x78, tag_bytes_format = 1, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "TLV-E")]
    pub nas_eap_message: Option<EapMessage>,

    #[tlv_config(tag = 0xA, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_nssai_inclusion_mode: Option<NssaiInclusionMode>,

    #[tlv_config(tag = 0x76, tag_bytes_format = 1, min_length = 0, max_length = 8320, length_bytes_format = 2, format = "TLV-E")]
    pub nas_operator_defined_access_category_definitions: Option<OperatorDefinedAccessCategoryDefinitions>,

    #[tlv_config(tag = 0x51, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_negotiated_drx_parameters: Option<FiveGsDrxParameters>,

    #[tlv_config(tag = 0xD, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_non_3gpp_nw_policies: Option<Non3gppNwProvidedPolicies>,

    #[tlv_config(tag = 0x60, tag_bytes_format = 1, length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_eps_bearer_context_status: Option<EpsBearerContextStatus>,

    #[tlv_config(tag = 0x6E, tag_bytes_format = 1, min_length = 1, max_length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_negotiated_extended_drx_parameters: Option<ExtendedDrxParameters>,

    #[tlv_config(tag = 0x6C, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_t3447_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x6B, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_t3448_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x6A, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_t3324_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x67, tag_bytes_format = 1, min_length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_ue_radio_capability_id: Option<UeRadioCapabilityId>,

    #[tlv_config(tag = 0xE, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_ue_radio_capability_id_deletion_indication: Option<UeRadioCapabilityIdDeletionIndication>,

    #[tlv_config(tag = 0x39, tag_bytes_format = 1, min_length = 2, max_length = 144, length_bytes_format = 1, format = "TLV")]
    pub nas_pending_nssai: Option<Nssai>,

    #[tlv_config(tag = 0x74, tag_bytes_format = 1, min_length = 31, length_bytes_format = 2, format = "TLV-E")]
    pub nas_ciphering_key_data: Option<CipheringKeyData>,

    #[tlv_config(tag = 0x75, tag_bytes_format = 1, min_length = 0, length_bytes_format = 2, format = "TLV-E")]
    pub nas_cag_information_list: Option<CagInformationList>,

    #[tlv_config(tag = 0x1B, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_truncated_5g_s_tmsi_configuration: Option<Truncated5gSTmsiConfiguration>,

    #[tlv_config(tag = 0x1C, tag_bytes_format = 1, min_length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_negotiated_wus_assistance_information: Option<WusAssistanceInformation>,

    #[tlv_config(tag = 0x29, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_negotiated_nb_n1_mode_drx_parameters: Option<NbN1ModeDrxParameters>,

    #[tlv_config(tag = 0x68, tag_bytes_format = 1, min_length = 3, max_length = 88, length_bytes_format = 1, format = "TLV")]
    pub nas_extended_rejected_nssai: Option<ExtendedRejectedNssai>,

    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 3, length_bytes_format = 2, format = "TLV-E")]
    pub nas_service_level_aa_container: Option<ServiceLevelAaContainer>,

    #[tlv_config(tag = 0x33, tag_bytes_format = 1, min_length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_negotiated_peips_assistance_information: Option<PeipsAssistanceInformation>,

    #[tlv_config(tag = 0x34, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_5gs_additional_request_result: Option<FiveGsAdditionalRequestResult>,

    #[tlv_config(tag = 0x70, tag_bytes_format = 1, min_length = 4, max_length = 4096, length_bytes_format = 2, format = "TLV-E")]
    pub nas_nssrg_information: Option<NssrgInformation>,

    #[tlv_config(tag = 0x14, tag_bytes_format = 1, length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_disaster_roaming_wait_range: Option<RegistrationWaitRange>,

    #[tlv_config(tag = 0x2C, tag_bytes_format = 1, length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_disaster_return_wait_range: Option<RegistrationWaitRange>,

    #[tlv_config(tag = 0x13, tag_bytes_format = 1, min_length = 0, length_bytes_format = 1, format = "TLV")]
    pub nas_list_of_plmns_to_be_used_in_disaster_condition: Option<ListOfPlmnsToBeUsedInDisasterCondition>,

    #[tlv_config(tag = 0x1D, tag_bytes_format = 1, min_length = 7, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_for_roaming: Option<FiveGsTrackingAreaIdentityList>,

    #[tlv_config(tag = 0x1E, tag_bytes_format = 1, min_length = 7, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_forregional_provision_of_service: Option<FiveGsTrackingAreaIdentityList>,

    #[tlv_config(tag = 0x71, tag_bytes_format = 1, min_length = 0, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_cag_information_list: Option<ExtendedCagInformationList>,

    #[tlv_config(tag = 0x7C, tag_bytes_format = 1, min_length = 6, max_length = 3140, length_bytes_format = 2, format = "TLV-E")]
    pub nas_nsag_information: Option<NsagInformation>,

} 


/*******************************************************
 * REGISTRATION COMPLETE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasRegistrationComplete {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_registration_complete_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x73, tag_bytes_format = 1, length = 17, length_bytes_format = 2, format = "TLV-E")]
    pub nas_sor_transparent_container: Option<SorTransparentContainer>,

} 


/*******************************************************
 * REGISTRATION REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasRegistrationReject {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_registration_reject_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_5gmm_cause: FiveGmmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x5F, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_t3346_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x16, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_t3502_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x78, tag_bytes_format = 1, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "TLV-E")]
    pub nas_eap_message: Option<EapMessage>,

    #[tlv_config(tag = 0x69, tag_bytes_format = 1, min_length = 2, max_length = 40, length_bytes_format = 1, format = "TLV")]
    pub nas_rejected_nssai: Option<RejectedNssai>,

    #[tlv_config(tag = 0x75, tag_bytes_format = 1, min_length = 0, length_bytes_format = 2, format = "TLV-E")]
    pub nas_cag_information_list: Option<CagInformationList>,

    #[tlv_config(tag = 0x68, tag_bytes_format = 1, min_length = 3, max_length = 88, length_bytes_format = 1, format = "TLV")]
    pub nas_extended_rejected_nssai: Option<ExtendedRejectedNssai>,

    #[tlv_config(tag = 0x2C, tag_bytes_format = 1, length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_disaster_return_wait_range: Option<RegistrationWaitRange>,

    #[tlv_config(tag = 0x71, tag_bytes_format = 1, min_length = 0, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_cag_information_list: Option<ExtendedCagInformationList>,

    #[tlv_config(tag = 0x3A, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_lower_bound_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x1D, tag_bytes_format = 1, min_length = 7, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_for_roaming: Option<FiveGsTrackingAreaIdentityList>,

    #[tlv_config(tag = 0x1E, tag_bytes_format = 1, min_length = 7, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_forregional_provision_of_service: Option<FiveGsTrackingAreaIdentityList>,

} 


/*******************************************************
 * DEREGISTRATION REQUEST FROM UE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasDeregistrationRequestFromUe {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_de_registration_request_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_de_registration_type: DeRegistrationType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_ngksi: KeySetIdentifier,

    #[tlv_config(tag_bytes_format = 0, min_length = 4, length_bytes_format = 2, format = "LV-E")]
    pub nas_5gs_mobile_identity: FiveGsMobileIdentity,

} 


/*******************************************************
 * DEREGISTRATION ACCEPT FROM UE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasDeregistrationAcceptFromUe {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_de_registration_accept_message_identity: MessageType,

} 


/*******************************************************
 * DEREGISTRATION REQUEST TO UE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasDeregistrationRequestToUe {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_de_registration_request_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_de_registration_type: DeRegistrationType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet_1: SpareHalfOctet,


    /* Optional fields */
    #[tlv_config(tag = 0x58, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_5gmm_cause: Option<FiveGmmCause>,

    #[tlv_config(tag = 0x5F, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_t3346_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x6D, tag_bytes_format = 1, min_length = 2, max_length = 40, length_bytes_format = 1, format = "TLV")]
    pub nas_rejected_nssai: Option<RejectedNssai>,

    #[tlv_config(tag = 0x75, tag_bytes_format = 1, min_length = 0, length_bytes_format = 2, format = "TLV-E")]
    pub nas_cag_information_list: Option<CagInformationList>,

    #[tlv_config(tag = 0x68, tag_bytes_format = 1, min_length = 3, max_length = 88, length_bytes_format = 1, format = "TLV")]
    pub nas_extended_rejected_nssai: Option<ExtendedRejectedNssai>,

    #[tlv_config(tag = 0x2C, tag_bytes_format = 1, length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_disaster_return_wait_range: Option<RegistrationWaitRange>,

    #[tlv_config(tag = 0x71, tag_bytes_format = 1, min_length = 0, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_cag_information_list: Option<ExtendedCagInformationList>,

    #[tlv_config(tag = 0x3A, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_lower_bound_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x1D, tag_bytes_format = 1, min_length = 7, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_for_roaming: Option<FiveGsTrackingAreaIdentityList>,

    #[tlv_config(tag = 0x1E, tag_bytes_format = 1, min_length = 7, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_forregional_provision_of_service: Option<FiveGsTrackingAreaIdentityList>,

} 


/*******************************************************
 * DEREGISTRATION ACCEPT TO UE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasDeregistrationAcceptToUe {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_de_registration_accept_message_identity: MessageType,

} 


/*******************************************************
 * SERVICE REQUEST
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasServiceRequest {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_service_request_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_ngksi: KeySetIdentifier,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_service_type: ServiceType,

    #[tlv_config(tag_bytes_format = 0, length = 7, length_bytes_format = 2, format = "LV-E")]
    pub nas_5g_s_tmsi: FiveGsMobileIdentity,


    /* Optional fields */
    #[tlv_config(tag = 0x40, tag_bytes_format = 1, min_length = 2, max_length = 32, length_bytes_format = 1, format = "TLV")]
    pub nas_uplink_data_status: Option<UplinkDataStatus>,

    #[tlv_config(tag = 0x50, tag_bytes_format = 1, min_length = 2, max_length = 32, length_bytes_format = 1, format = "TLV")]
    pub nas_pdu_session_status: Option<PduSessionStatus>,

    #[tlv_config(tag = 0x25, tag_bytes_format = 1, min_length = 2, max_length = 32, length_bytes_format = 1, format = "TLV")]
    pub nas_allowed_pdu_session_status: Option<AllowedPduSessionStatus>,

    #[tlv_config(tag = 0x71, tag_bytes_format = 1, min_length = 1, length_bytes_format = 2, format = "TLV-E")]
    pub nas_nas_message_container: Option<MessageContainer>,

    #[tlv_config(tag = 0x29, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_ue_request_type: Option<UeRequestType>,

    #[tlv_config(tag = 0x28, tag_bytes_format = 1, min_length = 1, max_length = 33, length_bytes_format = 1, format = "TLV")]
    pub nas_paging_restriction: Option<PagingRestriction>,

} 


/*******************************************************
 * SERVICE REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasServiceReject {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_service_reject_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_5gmm_cause: FiveGmmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x50, tag_bytes_format = 1, min_length = 2, max_length = 32, length_bytes_format = 1, format = "TLV")]
    pub nas_pdu_session_status: Option<PduSessionStatus>,

    #[tlv_config(tag = 0x5F, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_t3346_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x78, tag_bytes_format = 1, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "TLV-E")]
    pub nas_eap_message: Option<EapMessage>,

    #[tlv_config(tag = 0x6B, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_t3448_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x75, tag_bytes_format = 1, min_length = 0, length_bytes_format = 2, format = "TLV-E")]
    pub nas_cag_information_list: Option<CagInformationList>,

    #[tlv_config(tag = 0x2C, tag_bytes_format = 1, length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_disaster_return_wait_range: Option<RegistrationWaitRange>,

    #[tlv_config(tag = 0x71, tag_bytes_format = 1, min_length = 0, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_cag_information_list: Option<ExtendedCagInformationList>,

    #[tlv_config(tag = 0x3A, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_lower_bound_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x1D, tag_bytes_format = 1, min_length = 7, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_for_roaming: Option<FiveGsTrackingAreaIdentityList>,

    #[tlv_config(tag = 0x1E, tag_bytes_format = 1, min_length = 7, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_forregional_provision_of_service: Option<FiveGsTrackingAreaIdentityList>,

} 


/*******************************************************
 * SERVICE ACCEPT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasServiceAccept {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_service_accept_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x50, tag_bytes_format = 1, min_length = 2, max_length = 32, length_bytes_format = 1, format = "TLV")]
    pub nas_pdu_session_status: Option<PduSessionStatus>,

    #[tlv_config(tag = 0x26, tag_bytes_format = 1, min_length = 2, max_length = 32, length_bytes_format = 1, format = "TLV")]
    pub nas_pdu_session_reactivation_result: Option<PduSessionReactivationResult>,

    #[tlv_config(tag = 0x72, tag_bytes_format = 1, min_length = 2, max_length = 512, length_bytes_format = 2, format = "TLV-E")]
    pub nas_pdu_session_reactivation_result_error_cause: Option<PduSessionReactivationResultErrorCause>,

    #[tlv_config(tag = 0x78, tag_bytes_format = 1, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "TLV-E")]
    pub nas_eap_message: Option<EapMessage>,

    #[tlv_config(tag = 0x6B, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_t3448_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x34, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_5gs_additional_request_result: Option<FiveGsAdditionalRequestResult>,

    #[tlv_config(tag = 0x1D, tag_bytes_format = 1, min_length = 7, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_for_roaming: Option<FiveGsTrackingAreaIdentityList>,

    #[tlv_config(tag = 0x1E, tag_bytes_format = 1, min_length = 7, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_forregional_provision_of_service: Option<FiveGsTrackingAreaIdentityList>,

} 


/*******************************************************
 * CONFIGURATION UPDATE COMMAND
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasConfigurationUpdateCommand {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_configuration_update_command_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0xD, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_configuration_update_indication: Option<ConfigurationUpdateIndication>,

    #[tlv_config(tag = 0x77, tag_bytes_format = 1, length = 11, length_bytes_format = 2, format = "TLV-E")]
    pub nas_5g_guti: Option<FiveGsMobileIdentity>,

    #[tlv_config(tag = 0x54, tag_bytes_format = 1, min_length = 7, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_tai_list: Option<FiveGsTrackingAreaIdentityList>,

    #[tlv_config(tag = 0x15, tag_bytes_format = 1, min_length = 2, max_length = 72, length_bytes_format = 1, format = "TLV")]
    pub nas_allowed_nssai: Option<Nssai>,

    #[tlv_config(tag = 0x27, tag_bytes_format = 1, min_length = 4, max_length = 112, length_bytes_format = 1, format = "TLV")]
    pub nas_service_area_list: Option<ServiceAreaList>,

    #[tlv_config(tag = 0x43, tag_bytes_format = 1, min_length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_full_name_for_network: Option<NetworkName>,

    #[tlv_config(tag = 0x45, tag_bytes_format = 1, min_length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_short_name_for_network: Option<NetworkName>,

    #[tlv_config(tag = 0x46, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_local_time_zone: Option<TimeZone>,

    #[tlv_config(tag = 0x47, tag_bytes_format = 1, length = 7, length_bytes_format = 0, format = "TV")]
    pub nas_universal_time_and_local_time_zone: Option<TimeZoneAndTime>,

    #[tlv_config(tag = 0x49, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_network_daylight_saving_time: Option<DaylightSavingTime>,

    #[tlv_config(tag = 0x79, tag_bytes_format = 1, min_length = 0, max_length = 1712, length_bytes_format = 2, format = "TLV-E")]
    pub nas_ladn_information: Option<LadnInformation>,

    #[tlv_config(tag = 0xB, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_mico_indication: Option<MicoIndication>,

    #[tlv_config(tag = 0x9, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_network_slicing_indication: Option<NetworkSlicingIndication>,

    #[tlv_config(tag = 0x31, tag_bytes_format = 1, min_length = 2, max_length = 144, length_bytes_format = 1, format = "TLV")]
    pub nas_configured_nssai: Option<Nssai>,

    #[tlv_config(tag = 0x11, tag_bytes_format = 1, min_length = 2, max_length = 40, length_bytes_format = 1, format = "TLV")]
    pub nas_rejected_nssai: Option<RejectedNssai>,

    #[tlv_config(tag = 0x76, tag_bytes_format = 1, min_length = 0, max_length = 8320, length_bytes_format = 2, format = "TLV-E")]
    pub nas_operator_defined_access_category_definitions: Option<OperatorDefinedAccessCategoryDefinitions>,

    #[tlv_config(tag = 0xF, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_sms_indication: Option<SmsIndication>,

    #[tlv_config(tag = 0x6C, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_t3447_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x75, tag_bytes_format = 1, min_length = 0, length_bytes_format = 2, format = "TLV-E")]
    pub nas_cag_information_list: Option<CagInformationList>,

    #[tlv_config(tag = 0x67, tag_bytes_format = 1, min_length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_ue_radio_capability_id: Option<UeRadioCapabilityId>,

    #[tlv_config(tag = 0xA, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_ue_radio_capability_id_deletion_indication: Option<UeRadioCapabilityIdDeletionIndication>,

    #[tlv_config(tag = 0x44, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_5gs_registration_result: Option<FiveGsRegistrationResult>,

    #[tlv_config(tag = 0x1B, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_truncated_5g_s_tmsi_configuration: Option<Truncated5gSTmsiConfiguration>,

    #[tlv_config(tag = 0xC, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_additional_configuration_indication: Option<AdditionalConfigurationIndication>,

    #[tlv_config(tag = 0x68, tag_bytes_format = 1, min_length = 3, max_length = 88, length_bytes_format = 1, format = "TLV")]
    pub nas_extended_rejected_nssai: Option<ExtendedRejectedNssai>,

    #[tlv_config(tag = 0x72, tag_bytes_format = 1, min_length = 3, length_bytes_format = 2, format = "TLV-E")]
    pub nas_service_level_aa_container: Option<ServiceLevelAaContainer>,

    #[tlv_config(tag = 0x70, tag_bytes_format = 1, min_length = 4, max_length = 4096, length_bytes_format = 2, format = "TLV-E")]
    pub nas_nssrg_information: Option<NssrgInformation>,

    #[tlv_config(tag = 0x14, tag_bytes_format = 1, length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_disaster_roaming_wait_range: Option<RegistrationWaitRange>,

    #[tlv_config(tag = 0x2C, tag_bytes_format = 1, length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_disaster_return_wait_range: Option<RegistrationWaitRange>,

    #[tlv_config(tag = 0x13, tag_bytes_format = 1, min_length = 0, length_bytes_format = 1, format = "TLV")]
    pub nas_list_of_plmns_to_be_used_in_disaster_condition: Option<ListOfPlmnsToBeUsedInDisasterCondition>,

    #[tlv_config(tag = 0x71, tag_bytes_format = 1, min_length = 0, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_cag_information_list: Option<ExtendedCagInformationList>,

    #[tlv_config(tag = 0x1F, tag_bytes_format = 1, min_length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_updated_peips_assistance_information: Option<PeipsAssistanceInformation>,

    #[tlv_config(tag = 0x73, tag_bytes_format = 1, min_length = 6, max_length = 3140, length_bytes_format = 2, format = "TLV-E")]
    pub nas_nsag_information: Option<NsagInformation>,

    #[tlv_config(tag = 0xE, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_priority_indicator: Option<PriorityIndicator>,

} 


/*******************************************************
 * CONFIGURATION UPDATE COMPLETE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasConfigurationUpdateComplete {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_configuration_update_complete_message_identity: MessageType,

} 


/*******************************************************
 * AUTHENTICATION REQUEST
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasAuthenticationRequest {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_authentication_request_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_ngksi: KeySetIdentifier,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet_1: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, min_length = 2, length_bytes_format = 1, format = "LV")]
    pub nas_abba: Abba,


    /* Optional fields */
    #[tlv_config(tag = 0x21, tag_bytes_format = 1, length = 16, length_bytes_format = 0, format = "TV")]
    pub nas_authentication_parameter_rand: Option<AuthenticationParameterRand>,

    #[tlv_config(tag = 0x20, tag_bytes_format = 1, length = 16, length_bytes_format = 1, format = "TLV")]
    pub nas_authentication_parameter_autn: Option<AuthenticationParameterAutn>,

    #[tlv_config(tag = 0x78, tag_bytes_format = 1, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "TLV-E")]
    pub nas_eap_message: Option<EapMessage>,

} 


/*******************************************************
 * AUTHENTICATION RESPONSE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasAuthenticationResponse {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_authentication_response_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x2D, tag_bytes_format = 1, length = 16, length_bytes_format = 1, format = "TLV")]
    pub nas_authentication_response_parameter: Option<AuthenticationResponseParameter>,

    #[tlv_config(tag = 0x78, tag_bytes_format = 1, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "TLV-E")]
    pub nas_eap_message: Option<EapMessage>,

} 


/*******************************************************
 * AUTHENTICATION REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasAuthenticationReject {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_authentication_reject_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x78, tag_bytes_format = 1, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "TLV-E")]
    pub nas_eap_message: Option<EapMessage>,

} 


/*******************************************************
 * AUTHENTICATION FAILURE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasAuthenticationFailure {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_authentication_failure_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_5gmm_cause: FiveGmmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x30, tag_bytes_format = 1, length = 14, length_bytes_format = 1, format = "TLV")]
    pub nas_authentication_failure_parameter: Option<AuthenticationFailureParameter>,

} 


/*******************************************************
 * AUTHENTICATION RESULT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasAuthenticationResult {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_authentication_result_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_ngksi: KeySetIdentifier,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet_1: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "LV-E")]
    pub nas_eap_message: EapMessage,


    /* Optional fields */
    #[tlv_config(tag = 0x38, tag_bytes_format = 1, min_length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_abba: Option<Abba>,

} 


/*******************************************************
 * IDENTITY REQUEST
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasIdentityRequest {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_identity_request_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_identity_type: FiveGsIdentityType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet_1: SpareHalfOctet,

} 


/*******************************************************
 * IDENTITY RESPONSE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasIdentityResponse {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_identity_response_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, min_length = 1, length_bytes_format = 2, format = "LV-E")]
    pub nas_mobile_identity: FiveGsMobileIdentity,

} 


/*******************************************************
 * SECURITY MODE COMMAND
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasSecurityModeCommand {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_security_mode_command_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_selected_nas_security_algorithms: SecurityAlgorithms,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_ngksi: KeySetIdentifier,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet_1: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, min_length = 2, max_length = 8, length_bytes_format = 1, format = "LV")]
    pub nas_replayed_ue_security_capabilities: UeSecurityCapability,


    /* Optional fields */
    #[tlv_config(tag = 0xE, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_imeisv_request: Option<ImeisvRequest>,

    #[tlv_config(tag = 0x57, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_selected_eps_nas_security_algorithms: Option<EpsNasSecurityAlgorithms>,

    #[tlv_config(tag = 0x36, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_additional_5g_security_information: Option<Additional5gSecurityInformation>,

    #[tlv_config(tag = 0x78, tag_bytes_format = 1, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "TLV-E")]
    pub nas_eap_message: Option<EapMessage>,

    #[tlv_config(tag = 0x38, tag_bytes_format = 1, min_length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_abba: Option<Abba>,

    #[tlv_config(tag = 0x19, tag_bytes_format = 1, min_length = 2, max_length = 5, length_bytes_format = 1, format = "TLV")]
    pub nas_replayed_s1_ue_security_capabilities: Option<S1UeSecurityCapability>,

} 


/*******************************************************
 * SECURITY MODE COMPLETE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasSecurityModeComplete {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_security_mode_complete_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x77, tag_bytes_format = 1, length = 9, length_bytes_format = 2, format = "TLV-E")]
    pub nas_imeisv: Option<FiveGsMobileIdentity>,

    #[tlv_config(tag = 0x71, tag_bytes_format = 1, min_length = 1, length_bytes_format = 2, format = "TLV-E")]
    pub nas_nas_message_container: Option<MessageContainer>,

    #[tlv_config(tag = 0x78, tag_bytes_format = 1, min_length = 4, length_bytes_format = 2, format = "TLV-E")]
    pub nas_non_imeisv_pei: Option<FiveGsMobileIdentity>,

} 


/*******************************************************
 * SECURITY MODE REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasSecurityModeReject {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_security_mode_reject_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_5gmm_cause: FiveGmmCause,

} 


/*******************************************************
 * 5GMM STATUS
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct Nas5gmmStatus {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_5gmm_status_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_5gmm_cause: FiveGmmCause,

} 


/*******************************************************
 * NOTIFICATION
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasNotification {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_notification_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_access_type: AccessType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet_1: SpareHalfOctet,

} 


/*******************************************************
 * NOTIFICATION RESPONSE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasNotificationResponse {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_notification_response_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x50, tag_bytes_format = 1, min_length = 2, max_length = 32, length_bytes_format = 1, format = "TLV")]
    pub nas_pdu_session_status: Option<PduSessionStatus>,

} 


/*******************************************************
 * UL NAS TRANSPORT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasUlNasTransport {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_ul_nas_transport_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_payload_container_type: PayloadContainerType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet_1: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "LV-E")]
    pub nas_payload_container: PayloadContainer,


    /* Optional fields */
    #[tlv_config(tag = 0x12, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_pdu_session_id: Option<PduSessionIdentity2>,

    #[tlv_config(tag = 0x59, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_old_pdu_session_id: Option<PduSessionIdentity2>,

    #[tlv_config(tag = 0x8, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_request_type: Option<RequestType>,

    #[tlv_config(tag = 0x22, tag_bytes_format = 1, min_length = 1, max_length = 8, length_bytes_format = 1, format = "TLV")]
    pub nas_s_nssai: Option<SNssai>,

    #[tlv_config(tag = 0x25, tag_bytes_format = 1, min_length = 1, max_length = 100, length_bytes_format = 1, format = "TLV")]
    pub nas_dnn: Option<Dnn>,

    #[tlv_config(tag = 0x24, tag_bytes_format = 1, min_length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_additional_information: Option<AdditionalInformation>,

    #[tlv_config(tag = 0xA, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_ma_pdu_session_information: Option<MaPduSessionInformation>,

    #[tlv_config(tag = 0xF, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_release_assistance_indication: Option<ReleaseAssistanceIndication>,

} 


/*******************************************************
 * DL NAS TRANSPORT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasDlNasTransport {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_dl_nas_transport_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_payload_container_type: PayloadContainerType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_spare_half_octet_1: SpareHalfOctet,

    #[tlv_config(tag_bytes_format = 0, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "LV-E")]
    pub nas_payload_container: PayloadContainer,


    /* Optional fields */
    #[tlv_config(tag = 0x12, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_pdu_session_id: Option<PduSessionIdentity2>,

    #[tlv_config(tag = 0x24, tag_bytes_format = 1, min_length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_additional_information: Option<AdditionalInformation>,

    #[tlv_config(tag = 0x58, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_5gmm_cause: Option<FiveGmmCause>,

    #[tlv_config(tag = 0x37, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_back_off_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x3A, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_lower_bound_timer_value: Option<GprsTimer3>,

} 


/*******************************************************
 * PDU SESSION ESTABLISHMENT REQUEST
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionEstablishmentRequest {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_establishment_request_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 2, length_bytes_format = 0, format = "V")]
    pub nas_integrity_protection_maximum_data_rate: IntegrityProtectionMaximumDataRate,


    /* Optional fields */
    #[tlv_config(tag = 0x9, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_pdu_session_type: Option<PduSessionType>,

    #[tlv_config(tag = 0xA, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_ssc_mode: Option<SscMode>,

    #[tlv_config(tag = 0x28, tag_bytes_format = 1, min_length = 1, max_length = 13, length_bytes_format = 1, format = "TLV")]
    pub nas_5gsm_capability: Option<FiveGsmCapability>,

    #[tlv_config(tag = 0x55, tag_bytes_format = 1, length = 2, length_bytes_format = 0, format = "TV")]
    pub nas_maximum_number_of_supported_packet_filters: Option<MaximumNumberOfSupportedPacketFilters>,

    #[tlv_config(tag = 0xB, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_always_on_pdu_session_requested: Option<AlwaysOnPduSessionRequested>,

    #[tlv_config(tag = 0x39, tag_bytes_format = 1, min_length = 1, max_length = 253, length_bytes_format = 1, format = "TLV")]
    pub nas_sm_pdu_dn_request_container: Option<SmPduDnRequestContainer>,

    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

    #[tlv_config(tag = 0x66, tag_bytes_format = 1, min_length = 3, max_length = 255, length_bytes_format = 1, format = "TLV")]
    pub nas_ip_header_compression_configuration: Option<IpHeaderCompressionConfiguration>,

    #[tlv_config(tag = 0x6E, tag_bytes_format = 1, length = 6, length_bytes_format = 1, format = "TLV")]
    pub nas_ds_tt_ethernet_port_mac_address: Option<DsTtEthernetPortMacAddress>,

    #[tlv_config(tag = 0x6F, tag_bytes_format = 1, length = 8, length_bytes_format = 1, format = "TLV")]
    pub nas_ue_ds_tt_residence_time: Option<UeDsTtResidenceTime>,

    #[tlv_config(tag = 0x74, tag_bytes_format = 1, min_length = 5, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_port_management_information_container: Option<PortManagementInformationContainer>,

    #[tlv_config(tag = 0x1F, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_ethernet_header_compression_configuration: Option<EthernetHeaderCompressionConfiguration>,

    #[tlv_config(tag = 0x29, tag_bytes_format = 1, length = 9, length_bytes_format = 1, format = "TLV")]
    pub nas_suggested_interface_identifier: Option<PduAddress>,

    #[tlv_config(tag = 0x72, tag_bytes_format = 1, min_length = 3, length_bytes_format = 2, format = "TLV-E")]
    pub nas_service_level_aa_container: Option<ServiceLevelAaContainer>,

    #[tlv_config(tag = 0x70, tag_bytes_format = 1, min_length = 5, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_requested_mbs_container: Option<RequestedMbsContainer>,

    #[tlv_config(tag = 0x34, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_pdu_session_pair_id: Option<PduSessionPairId>,

    #[tlv_config(tag = 0x35, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_rsn: Option<Rsn>,

} 


/*******************************************************
 * PDU SESSION ESTABLISHMENT ACCEPT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionEstablishmentAccept {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_establishment_accept_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_selected_pdu_session_type: PduSessionType,

    #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
    pub nas_selected_ssc_mode: SscMode,

    #[tlv_config(tag_bytes_format = 0, min_length = 4, max_length = 65536, length_bytes_format = 2, format = "LV-E")]
    pub nas_authorized_qos_rules: QosRules,

    #[tlv_config(tag_bytes_format = 0, length = 6, length_bytes_format = 1, format = "LV")]
    pub nas_session_ambr: SessionAmbr,


    /* Optional fields */
    #[tlv_config(tag = 0x59, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_5gsm_cause: Option<FiveGsmCause>,

    #[tlv_config(tag = 0x29, tag_bytes_format = 1, min_length = 5, max_length = 29, length_bytes_format = 1, format = "TLV")]
    pub nas_pdu_address: Option<PduAddress>,

    #[tlv_config(tag = 0x56, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_rq_timer_value: Option<GprsTimer>,

    #[tlv_config(tag = 0x22, tag_bytes_format = 1, min_length = 1, max_length = 8, length_bytes_format = 1, format = "TLV")]
    pub nas_s_nssai: Option<SNssai>,

    #[tlv_config(tag = 0x8, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_always_on_pdu_session_indication: Option<AlwaysOnPduSessionIndication>,

    #[tlv_config(tag = 0x75, tag_bytes_format = 1, min_length = 4, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_mapped_eps_bearer_contexts: Option<MappedEpsBearerContexts>,

    #[tlv_config(tag = 0x78, tag_bytes_format = 1, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "TLV-E")]
    pub nas_eap_message: Option<EapMessage>,

    #[tlv_config(tag = 0x79, tag_bytes_format = 1, min_length = 3, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_authorized_qos_flow_descriptions: Option<QosFlowDescriptions>,

    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

    #[tlv_config(tag = 0x25, tag_bytes_format = 1, min_length = 1, max_length = 100, length_bytes_format = 1, format = "TLV")]
    pub nas_dnn: Option<Dnn>,

    #[tlv_config(tag = 0x17, tag_bytes_format = 1, min_length = 1, max_length = 13, length_bytes_format = 1, format = "TLV")]
    pub nas_5gsm_network_feature_support: Option<FiveGsmNetworkFeatureSupport>,

    #[tlv_config(tag = 0x18, tag_bytes_format = 1, length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_serving_plmn_rate_control: Option<ServingPlmnRateControl>,

    #[tlv_config(tag = 0x77, tag_bytes_format = 1, min_length = 0, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_atsss_container: Option<AtsssContainer>,

    #[tlv_config(tag = 0xC, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_control_plane_only_indication: Option<ControlPlaneOnlyIndication>,

    #[tlv_config(tag = 0x66, tag_bytes_format = 1, min_length = 3, max_length = 255, length_bytes_format = 1, format = "TLV")]
    pub nas_ip_header_compression_configuration: Option<IpHeaderCompressionConfiguration>,

    #[tlv_config(tag = 0x1F, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_ethernet_header_compression_configuration: Option<EthernetHeaderCompressionConfiguration>,

    #[tlv_config(tag = 0x72, tag_bytes_format = 1, min_length = 3, length_bytes_format = 2, format = "TLV-E")]
    pub nas_service_level_aa_container: Option<ServiceLevelAaContainer>,

    #[tlv_config(tag = 0x71, tag_bytes_format = 1, min_length = 6, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_received_mbs_container: Option<ReceivedMbsContainer>,

} 


/*******************************************************
 * PDU SESSION ESTABLISHMENT REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionEstablishmentReject {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_establishment_reject_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_5gsm_cause: FiveGsmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x37, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_back_off_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0xF, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_allowed_ssc_mode: Option<AllowedSscMode>,

    #[tlv_config(tag = 0x78, tag_bytes_format = 1, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "TLV-E")]
    pub nas_eap_message: Option<EapMessage>,

    #[tlv_config(tag = 0x61, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_5gsm_congestion_re_attempt_indicator: Option<FiveGsmCongestionReAttemptIndicator>,

    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

    #[tlv_config(tag = 0x1D, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_re_attempt_indicator: Option<ReAttemptIndicator>,

    #[tlv_config(tag = 0x72, tag_bytes_format = 1, min_length = 3, length_bytes_format = 2, format = "TLV-E")]
    pub nas_service_level_aa_container: Option<ServiceLevelAaContainer>,

} 


/*******************************************************
 * PDU SESSION AUTHENTICATION COMMAND
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionAuthenticationCommand {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_authentication_command_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "LV-E")]
    pub nas_eap_message: EapMessage,


    /* Optional fields */
    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

} 


/*******************************************************
 * PDU SESSION AUTHENTICATION COMPLETE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionAuthenticationComplete {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_authentication_complete_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "LV-E")]
    pub nas_eap_message: EapMessage,


    /* Optional fields */
    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

} 


/*******************************************************
 * PDU SESSION AUTHENTICATION RESULT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionAuthenticationResult {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_authentication_result_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x78, tag_bytes_format = 1, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "TLV-E")]
    pub nas_eap_message: Option<EapMessage>,

    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

} 


/*******************************************************
 * PDU SESSION MODIFICATION REQUEST
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionModificationRequest {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_modification_request_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x28, tag_bytes_format = 1, min_length = 1, max_length = 13, length_bytes_format = 1, format = "TLV")]
    pub nas_5gsm_capability: Option<FiveGsmCapability>,

    #[tlv_config(tag = 0x59, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_5gsm_cause: Option<FiveGsmCause>,

    #[tlv_config(tag = 0x55, tag_bytes_format = 1, length = 2, length_bytes_format = 0, format = "TV")]
    pub nas_maximum_number_of_supported_packet_filters: Option<MaximumNumberOfSupportedPacketFilters>,

    #[tlv_config(tag = 0xB, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_always_on_pdu_session_requested: Option<AlwaysOnPduSessionRequested>,

    #[tlv_config(tag = 0x13, tag_bytes_format = 1, length = 2, length_bytes_format = 0, format = "TV")]
    pub nas_integrity_protection_maximum_data_rate: Option<IntegrityProtectionMaximumDataRate>,

    #[tlv_config(tag = 0x7A, tag_bytes_format = 1, min_length = 4, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_requested_qos_rules: Option<QosRules>,

    #[tlv_config(tag = 0x79, tag_bytes_format = 1, min_length = 3, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_requested_qos_flow_descriptions: Option<QosFlowDescriptions>,

    #[tlv_config(tag = 0x75, tag_bytes_format = 1, min_length = 4, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_mapped_eps_bearer_contexts: Option<MappedEpsBearerContexts>,

    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

    #[tlv_config(tag = 0x74, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_port_management_information_container: Option<PortManagementInformationContainer>,

    #[tlv_config(tag = 0x66, tag_bytes_format = 1, min_length = 3, max_length = 255, length_bytes_format = 1, format = "TLV")]
    pub nas_ip_header_compression_configuration: Option<HeaderCompressionConfiguration>,

    #[tlv_config(tag = 0x1F, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_ethernet_header_compression_configuration: Option<EthernetHeaderCompressionConfiguration>,

    #[tlv_config(tag = 0x70, tag_bytes_format = 1, min_length = 5, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_requested_mbs_container: Option<RequestedMbsContainer>,

    #[tlv_config(tag = 0x72, tag_bytes_format = 1, min_length = 3, length_bytes_format = 2, format = "TLV-E")]
    pub nas_service_level_aa_container: Option<ServiceLevelAaContainer>,

} 


/*******************************************************
 * PDU SESSION MODIFICATION REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionModificationReject {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_modification_reject_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_5gsm_cause: FiveGsmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x37, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_back_off_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x61, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_5gsm_congestion_re_attempt_indicator: Option<FiveGsmCongestionReAttemptIndicator>,

    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

    #[tlv_config(tag = 0x1D, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_re_attempt_indicator: Option<ReAttemptIndicator>,

} 


/*******************************************************
 * PDU SESSION MODIFICATION COMMAND
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionModificationCommand {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_modification_command_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x59, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_5gsm_cause: Option<FiveGsmCause>,

    #[tlv_config(tag = 0x2A, tag_bytes_format = 1, length = 6, length_bytes_format = 1, format = "TLV")]
    pub nas_session_ambr: Option<SessionAmbr>,

    #[tlv_config(tag = 0x56, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_rq_timer_value: Option<GprsTimer>,

    #[tlv_config(tag = 0x8, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_always_on_pdu_session_indication: Option<AlwaysOnPduSessionIndication>,

    #[tlv_config(tag = 0x7A, tag_bytes_format = 1, min_length = 4, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_authorized_qos_rules: Option<QosRules>,

    #[tlv_config(tag = 0x75, tag_bytes_format = 1, min_length = 4, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_mapped_eps_bearer_contexts: Option<MappedEpsBearerContexts>,

    #[tlv_config(tag = 0x79, tag_bytes_format = 1, min_length = 3, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_authorized_qos_flow_descriptions: Option<QosFlowDescriptions>,

    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

    #[tlv_config(tag = 0x77, tag_bytes_format = 1, min_length = 0, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_atsss_container: Option<AtsssContainer>,

    #[tlv_config(tag = 0x66, tag_bytes_format = 1, min_length = 3, max_length = 255, length_bytes_format = 1, format = "TLV")]
    pub nas_ip_header_compression_configuration: Option<IpHeaderCompressionConfiguration>,

    #[tlv_config(tag = 0x74, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_port_management_information_container: Option<PortManagementInformationContainer>,

    #[tlv_config(tag = 0x1E, tag_bytes_format = 1, length = 2, length_bytes_format = 1, format = "TLV")]
    pub nas_serving_plmn_rate_control: Option<ServingPlmnRateControl>,

    #[tlv_config(tag = 0x1F, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_ethernet_header_compression_configuration: Option<EthernetHeaderCompressionConfiguration>,

    #[tlv_config(tag = 0x71, tag_bytes_format = 1, min_length = 6, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_received_mbs_container: Option<ReceivedMbsContainer>,

    #[tlv_config(tag = 0x72, tag_bytes_format = 1, min_length = 3, length_bytes_format = 2, format = "TLV-E")]
    pub nas_service_level_aa_container: Option<ServiceLevelAaContainer>,

} 


/*******************************************************
 * PDU SESSION MODIFICATION COMPLETE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionModificationComplete {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_modification_complete_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

    #[tlv_config(tag = 0x74, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_port_management_information_container: Option<PortManagementInformationContainer>,

} 


/*******************************************************
 * PDU SESSION MODIFICATION COMMAND REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionModificationCommandReject {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_modification_command_reject_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_5gsm_cause: FiveGsmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

} 


/*******************************************************
 * PDU SESSION RELEASE REQUEST
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionReleaseRequest {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_release_request_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x59, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_5gsm_cause: Option<FiveGsmCause>,

    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

} 


/*******************************************************
 * PDU SESSION RELEASE REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionReleaseReject {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_release_reject_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_5gsm_cause: FiveGsmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

} 


/*******************************************************
 * PDU SESSION RELEASE COMMAND
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionReleaseCommand {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_release_command_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_5gsm_cause: FiveGsmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x37, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_back_off_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x78, tag_bytes_format = 1, min_length = 4, max_length = 1500, length_bytes_format = 2, format = "TLV-E")]
    pub nas_eap_message: Option<EapMessage>,

    #[tlv_config(tag = 0x61, tag_bytes_format = 1, length = 1, length_bytes_format = 1, format = "TLV")]
    pub nas_5gsm_congestion_re_attempt_indicator: Option<FiveGsmCongestionReAttemptIndicator>,

    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

    #[tlv_config(tag = 0xD, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
    pub nas_access_type: Option<AccessType>,

    #[tlv_config(tag = 0x72, tag_bytes_format = 1, min_length = 3, length_bytes_format = 2, format = "TLV-E")]
    pub nas_service_level_aa_container: Option<ServiceLevelAaContainer>,

} 


/*******************************************************
 * PDU SESSION RELEASE COMPLETE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct NasPduSessionReleaseComplete {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_release_complete_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x59, tag_bytes_format = 1, length = 1, length_bytes_format = 0, format = "TV")]
    pub nas_5gsm_cause: Option<FiveGsmCause>,

    #[tlv_config(tag = 0x7B, tag_bytes_format = 1, min_length = 1, max_length = 65535, length_bytes_format = 2, format = "TLV-E")]
    pub nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions>,

} 


/*******************************************************
 * 5GSM STATUS
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode, Clone)]
pub struct Nas5gsmStatus {
    /* Mandatory fields */
    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_5gsm_status_message_identity: MessageType,

    #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
    pub nas_5gsm_cause: FiveGsmCause,

} 


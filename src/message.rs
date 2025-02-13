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
 * Created on: 2025-01-04 18:13:00.264223 by nr
 * from
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

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasRegistrationRequest {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_registration_request_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gs_registration_type: _5gsRegistrationType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_ngksi: KeySetIdentifier,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 2, format = "LV-E")]
    nas_5gs_mobile_identity: _5gsMobileIdentity<Vec<u8>>,


    /* Optional fields */
    #[tlv_config(tag = 0xC-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_non_current_native_nas_key_set_identifier: Option<KeySetIdentifier<Vec<u8>>>,

    #[tlv_config(tag = 0x10, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_5gmm_capability: Option<_5gmmCapability<Vec<u8>>>,

    #[tlv_config(tag = 0x2E, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_ue_security_capability: Option<UeSecurityCapability<Vec<u8>>>,

    #[tlv_config(tag = 0x2F, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_requested_nssai: Option<Nssai<Vec<u8>>>,

    #[tlv_config(tag = 0x52, tag_byte_format = 1, length = 6, length_byte_format = 0, format = "TV")]
    nas_last_visited_registered_tai: Option<_5gsTrackingAreaIdentity<Vec<u8>>>,

    #[tlv_config(tag = 0x17, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_s1_ue_network_capability: Option<S1UeNetworkCapability<Vec<u8>>>,

    #[tlv_config(tag = 0x40, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_uplink_data_status: Option<UplinkDataStatus<Vec<u8>>>,

    #[tlv_config(tag = 0x50, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_pdu_session_status: Option<PduSessionStatus<Vec<u8>>>,

    #[tlv_config(tag = 0xB-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_mico_indication: Option<MicoIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x2B, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_ue_status: Option<UeStatus>,

    #[tlv_config(tag = 0x77, tag_byte_format = 1, length = 11, length_byte_format = 2, format = "TLV-E")]
    nas_additional_guti: Option<_5gsMobileIdentity<Vec<u8>>>,

    #[tlv_config(tag = 0x25, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_allowed_pdu_session_status: Option<AllowedPduSessionStatus<Vec<u8>>>,

    #[tlv_config(tag = 0x18, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_ue_usage_setting: Option<UeUsageSetting>,

    #[tlv_config(tag = 0x51, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_requested_drx_parameters: Option<_5gsDrxParameters>,

    #[tlv_config(tag = 0x70, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_eps_nas_message_container: Option<EpsNasMessageContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x74, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_ladn_indication: Option<LadnIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x8-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_payload_container_type: Option<PayloadContainerType<Vec<u8>>>,

    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_payload_container: Option<PayloadContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x9-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_network_slicing_indication: Option<NetworkSlicingIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x53, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_5gs_update_type: Option<_5gsUpdateType>,

    #[tlv_config(tag = 0x41, tag_byte_format = 1, length = 3, length_byte_format = 1, format = "TLV")]
    nas_mobile_station_classmark_2: Option<MobileStationClassmark2<Vec<u8>>>,

    #[tlv_config(tag = 0x42, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_supported_codecs: Option<SupportedCodecList<Vec<u8>>>,

    #[tlv_config(tag = 0x71, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_nas_message_container: Option<MessageContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x60, tag_byte_format = 1, length = 2, length_byte_format = 1, format = "TLV")]
    nas_eps_bearer_context_status: Option<EpsBearerContextStatus<Vec<u8>>>,

    #[tlv_config(tag = 0x6E, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_requested_extended_drx_parameters: Option<ExtendedDrxParameters<Vec<u8>>>,

    #[tlv_config(tag = 0x6A, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_t3324_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x67, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_ue_radio_capability_id: Option<UeRadioCapabilityId<Vec<u8>>>,

    #[tlv_config(tag = 0x35, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_requested_mapped_nssai: Option<MappedNssai<Vec<u8>>>,

    #[tlv_config(tag = 0x48, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_additional_information_requested: Option<AdditionalInformationRequested>,

    #[tlv_config(tag = 0x1A, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_requested_wus_assistance_information: Option<WusAssistanceInformation<Vec<u8>>>,

    #[tlv_config(tag = 0xA-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_n5gc_indication: Option<N5gcIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x30, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_requested_nb_n1_mode_drx_parameters: Option<NbN1ModeDrxParameters>,

    #[tlv_config(tag = 0x29, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_ue_request_type: Option<UeRequestType>,

    #[tlv_config(tag = 0x28, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_paging_restriction: Option<PagingRestriction<Vec<u8>>>,

    #[tlv_config(tag = 0x72, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_service_level_aa_container: Option<ServiceLevelAaContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x32, tag_byte_format = 1, length = 6, length_byte_format = 1, format = "TLV")]
    nas_nid: Option<Nid<Vec<u8>>>,

    #[tlv_config(tag = 0x16, tag_byte_format = 1, length = 3, length_byte_format = 1, format = "TLV")]
    nas_ms_determined_plmn_with_disaster_condition: Option<PlmnIdentity<Vec<u8>>>,

    #[tlv_config(tag = 0x2A, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_requested_peips_assistance_information: Option<PeipsAssistanceInformation<Vec<u8>>>,

    #[tlv_config(tag = 0x3B, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_requested_t3512_value: Option<GprsTimer3>,

}


/*******************************************************
 * REGISTRATION ACCEPT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasRegistrationAccept {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_registration_accept_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length = 1, length_byte_format = 1, format = "LV")]
    nas_5gs_registration_result: _5gsRegistrationResult,


    /* Optional fields */
    #[tlv_config(tag = 0x77, tag_byte_format = 1, length = 11, length_byte_format = 2, format = "TLV-E")]
    nas_5g_guti: Option<_5gsMobileIdentity<Vec<u8>>>,

    #[tlv_config(tag = 0x4A, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_equivalent_plmns: Option<PlmnList<Vec<u8>>>,

    #[tlv_config(tag = 0x54, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_tai_list: Option<_5gsTrackingAreaIdentityList<Vec<u8>>>,

    #[tlv_config(tag = 0x15, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_allowed_nssai: Option<Nssai<Vec<u8>>>,

    #[tlv_config(tag = 0x11, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_rejected_nssai: Option<RejectedNssai<Vec<u8>>>,

    #[tlv_config(tag = 0x31, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_configured_nssai: Option<Nssai<Vec<u8>>>,

    #[tlv_config(tag = 0x21, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_5gs_network_feature_support: Option<_5gsNetworkFeatureSupport<Vec<u8>>>,

    #[tlv_config(tag = 0x50, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_pdu_session_status: Option<PduSessionStatus<Vec<u8>>>,

    #[tlv_config(tag = 0x26, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_pdu_session_reactivation_result: Option<PduSessionReactivationResult<Vec<u8>>>,

    #[tlv_config(tag = 0x72, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_pdu_session_reactivation_result_error_cause: Option<PduSessionReactivationResultErrorCause<Vec<u8>>>,

    #[tlv_config(tag = 0x79, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_ladn_information: Option<LadnInformation<Vec<u8>>>,

    #[tlv_config(tag = 0xB-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_mico_indication: Option<MicoIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x9-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_network_slicing_indication: Option<NetworkSlicingIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x27, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_service_area_list: Option<ServiceAreaList<Vec<u8>>>,

    #[tlv_config(tag = 0x5E, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_t3512_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x5D, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_non_3gpp_de_registration_timer_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x16, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_t3502_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x34, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_emergency_number_list: Option<EmergencyNumberList<Vec<u8>>>,

    #[tlv_config(tag = 0x7A, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_emergency_number_list: Option<ExtendedEmergencyNumberList<Vec<u8>>>,

    #[tlv_config(tag = 0x73, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_sor_transparent_container: Option<SorTransparentContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x78, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_eap_message: Option<EapMessage<Vec<u8>>>,

    #[tlv_config(tag = 0xA-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_nssai_inclusion_mode: Option<NssaiInclusionMode<Vec<u8>>>,

    #[tlv_config(tag = 0x76, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_operator_defined_access_category_definitions: Option<OperatorDefinedAccessCategoryDefinitions<Vec<u8>>>,

    #[tlv_config(tag = 0x51, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_negotiated_drx_parameters: Option<_5gsDrxParameters>,

    #[tlv_config(tag = 0xD-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_non_3gpp_nw_policies: Option<Non3gppNwProvidedPolicies<Vec<u8>>>,

    #[tlv_config(tag = 0x60, tag_byte_format = 1, length = 2, length_byte_format = 1, format = "TLV")]
    nas_eps_bearer_context_status: Option<EpsBearerContextStatus<Vec<u8>>>,

    #[tlv_config(tag = 0x6E, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_negotiated_extended_drx_parameters: Option<ExtendedDrxParameters<Vec<u8>>>,

    #[tlv_config(tag = 0x6C, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_t3447_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x6B, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_t3448_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x6A, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_t3324_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x67, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_ue_radio_capability_id: Option<UeRadioCapabilityId<Vec<u8>>>,

    #[tlv_config(tag = 0xE-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_ue_radio_capability_id_deletion_indication: Option<UeRadioCapabilityIdDeletionIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x39, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_pending_nssai: Option<Nssai<Vec<u8>>>,

    #[tlv_config(tag = 0x74, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_ciphering_key_data: Option<CipheringKeyData<Vec<u8>>>,

    #[tlv_config(tag = 0x75, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_cag_information_list: Option<CagInformationList<Vec<u8>>>,

    #[tlv_config(tag = 0x1B, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_truncated_5g_s_tmsi_configuration: Option<Truncated5gSTmsiConfiguration>,

    #[tlv_config(tag = 0x1C, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_negotiated_wus_assistance_information: Option<WusAssistanceInformation<Vec<u8>>>,

    #[tlv_config(tag = 0x29, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_negotiated_nb_n1_mode_drx_parameters: Option<NbN1ModeDrxParameters>,

    #[tlv_config(tag = 0x68, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_extended_rejected_nssai: Option<ExtendedRejectedNssai<Vec<u8>>>,

    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_service_level_aa_container: Option<ServiceLevelAaContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x33, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_negotiated_peips_assistance_information: Option<PeipsAssistanceInformation<Vec<u8>>>,

    #[tlv_config(tag = 0x34, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_5gs_additional_request_result: Option<_5gsAdditionalRequestResult>,

    #[tlv_config(tag = 0x70, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_nssrg_information: Option<NssrgInformation<Vec<u8>>>,

    #[tlv_config(tag = 0x14, tag_byte_format = 1, length = 2, length_byte_format = 1, format = "TLV")]
    nas_disaster_roaming_wait_range: Option<RegistrationWaitRange<Vec<u8>>>,

    #[tlv_config(tag = 0x2C, tag_byte_format = 1, length = 2, length_byte_format = 1, format = "TLV")]
    nas_disaster_return_wait_range: Option<RegistrationWaitRange<Vec<u8>>>,

    #[tlv_config(tag = 0x13, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_list_of_plmns_to_be_used_in_disaster_condition: Option<ListOfPlmnsToBeUsedInDisasterCondition<Vec<u8>>>,

    #[tlv_config(tag = 0x1D, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_for_roaming: Option<_5gsTrackingAreaIdentityList<Vec<u8>>>,

    #[tlv_config(tag = 0x1E, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_forregional_provision_of_service: Option<_5gsTrackingAreaIdentityList<Vec<u8>>>,

    #[tlv_config(tag = 0x71, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_cag_information_list: Option<ExtendedCagInformationList<Vec<u8>>>,

    #[tlv_config(tag = 0x7C, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_nsag_information: Option<NsagInformation<Vec<u8>>>,

}


/*******************************************************
 * REGISTRATION COMPLETE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasRegistrationComplete {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_registration_complete_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x73, tag_byte_format = 1, length = 17, length_byte_format = 2, format = "TLV-E")]
    nas_sor_transparent_container: Option<SorTransparentContainer<Vec<u8>>>,

}


/*******************************************************
 * REGISTRATION REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasRegistrationReject {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_registration_reject_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gmm_cause: _5gmmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x5F, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_t3346_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x16, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_t3502_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x78, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_eap_message: Option<EapMessage<Vec<u8>>>,

    #[tlv_config(tag = 0x69, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_rejected_nssai: Option<RejectedNssai<Vec<u8>>>,

    #[tlv_config(tag = 0x75, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_cag_information_list: Option<CagInformationList<Vec<u8>>>,

    #[tlv_config(tag = 0x68, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_extended_rejected_nssai: Option<ExtendedRejectedNssai<Vec<u8>>>,

    #[tlv_config(tag = 0x2C, tag_byte_format = 1, length = 2, length_byte_format = 1, format = "TLV")]
    nas_disaster_return_wait_range: Option<RegistrationWaitRange<Vec<u8>>>,

    #[tlv_config(tag = 0x71, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_cag_information_list: Option<ExtendedCagInformationList<Vec<u8>>>,

    #[tlv_config(tag = 0x3A, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_lower_bound_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x1D, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_for_roaming: Option<_5gsTrackingAreaIdentityList<Vec<u8>>>,

    #[tlv_config(tag = 0x1E, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_forregional_provision_of_service: Option<_5gsTrackingAreaIdentityList<Vec<u8>>>,

}


/*******************************************************
 * DEREGISTRATION REQUEST FROM UE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasDeregistrationRequestFromUe {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_de_registration_request_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_de_registration_type: DeRegistrationType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_ngksi: KeySetIdentifier,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 2, format = "LV-E")]
    nas_5gs_mobile_identity: _5gsMobileIdentity<Vec<u8>>,

}


/*******************************************************
 * DEREGISTRATION ACCEPT FROM UE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasDeregistrationAcceptFromUe {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_de_registration_accept_message_identity: MessageType,

}


/*******************************************************
 * DEREGISTRATION REQUEST TO UE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasDeregistrationRequestToUe {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_de_registration_request_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_de_registration_type: DeRegistrationType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet_1: SpareHalfOctet,


    /* Optional fields */
    #[tlv_config(tag = 0x58, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_5gmm_cause: Option<_5gmmCause>,

    #[tlv_config(tag = 0x5F, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_t3346_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x6D, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_rejected_nssai: Option<RejectedNssai<Vec<u8>>>,

    #[tlv_config(tag = 0x75, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_cag_information_list: Option<CagInformationList<Vec<u8>>>,

    #[tlv_config(tag = 0x68, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_extended_rejected_nssai: Option<ExtendedRejectedNssai<Vec<u8>>>,

    #[tlv_config(tag = 0x2C, tag_byte_format = 1, length = 2, length_byte_format = 1, format = "TLV")]
    nas_disaster_return_wait_range: Option<RegistrationWaitRange<Vec<u8>>>,

    #[tlv_config(tag = 0x71, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_cag_information_list: Option<ExtendedCagInformationList<Vec<u8>>>,

    #[tlv_config(tag = 0x3A, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_lower_bound_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x1D, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_for_roaming: Option<_5gsTrackingAreaIdentityList<Vec<u8>>>,

    #[tlv_config(tag = 0x1E, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_forregional_provision_of_service: Option<_5gsTrackingAreaIdentityList<Vec<u8>>>,

}


/*******************************************************
 * DEREGISTRATION ACCEPT TO UE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasDeregistrationAcceptToUe {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_de_registration_accept_message_identity: MessageType,

}


/*******************************************************
 * SERVICE REQUEST
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasServiceRequest {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_service_request_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_ngksi: KeySetIdentifier,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_service_type: ServiceType,

    #[tlv_config(tag_byte_format = 0, length = 7, length_byte_format = 2, format = "LV-E")]
    nas_5g_s_tmsi: _5gsMobileIdentity<Vec<u8>>,


    /* Optional fields */
    #[tlv_config(tag = 0x40, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_uplink_data_status: Option<UplinkDataStatus<Vec<u8>>>,

    #[tlv_config(tag = 0x50, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_pdu_session_status: Option<PduSessionStatus<Vec<u8>>>,

    #[tlv_config(tag = 0x25, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_allowed_pdu_session_status: Option<AllowedPduSessionStatus<Vec<u8>>>,

    #[tlv_config(tag = 0x71, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_nas_message_container: Option<MessageContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x29, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_ue_request_type: Option<UeRequestType>,

    #[tlv_config(tag = 0x28, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_paging_restriction: Option<PagingRestriction<Vec<u8>>>,

}


/*******************************************************
 * SERVICE REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasServiceReject {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_service_reject_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gmm_cause: _5gmmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x50, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_pdu_session_status: Option<PduSessionStatus<Vec<u8>>>,

    #[tlv_config(tag = 0x5F, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_t3346_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x78, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_eap_message: Option<EapMessage<Vec<u8>>>,

    #[tlv_config(tag = 0x6B, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_t3448_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x75, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_cag_information_list: Option<CagInformationList<Vec<u8>>>,

    #[tlv_config(tag = 0x2C, tag_byte_format = 1, length = 2, length_byte_format = 1, format = "TLV")]
    nas_disaster_return_wait_range: Option<RegistrationWaitRange<Vec<u8>>>,

    #[tlv_config(tag = 0x71, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_cag_information_list: Option<ExtendedCagInformationList<Vec<u8>>>,

    #[tlv_config(tag = 0x3A, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_lower_bound_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x1D, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_for_roaming: Option<_5gsTrackingAreaIdentityList<Vec<u8>>>,

    #[tlv_config(tag = 0x1E, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_forregional_provision_of_service: Option<_5gsTrackingAreaIdentityList<Vec<u8>>>,

}


/*******************************************************
 * SERVICE ACCEPT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasServiceAccept {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_service_accept_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x50, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_pdu_session_status: Option<PduSessionStatus<Vec<u8>>>,

    #[tlv_config(tag = 0x26, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_pdu_session_reactivation_result: Option<PduSessionReactivationResult<Vec<u8>>>,

    #[tlv_config(tag = 0x72, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_pdu_session_reactivation_result_error_cause: Option<PduSessionReactivationResultErrorCause<Vec<u8>>>,

    #[tlv_config(tag = 0x78, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_eap_message: Option<EapMessage<Vec<u8>>>,

    #[tlv_config(tag = 0x6B, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_t3448_value: Option<GprsTimer2>,

    #[tlv_config(tag = 0x34, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_5gs_additional_request_result: Option<_5gsAdditionalRequestResult>,

    #[tlv_config(tag = 0x1D, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_for_roaming: Option<_5gsTrackingAreaIdentityList<Vec<u8>>>,

    #[tlv_config(tag = 0x1E, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_forbidden_tai_for_the_list_of_5gs_forbidden_tracking_areas_forregional_provision_of_service: Option<_5gsTrackingAreaIdentityList<Vec<u8>>>,

}


/*******************************************************
 * CONFIGURATION UPDATE COMMAND
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasConfigurationUpdateCommand {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_configuration_update_command_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0xD-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_configuration_update_indication: Option<ConfigurationUpdateIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x77, tag_byte_format = 1, length = 11, length_byte_format = 2, format = "TLV-E")]
    nas_5g_guti: Option<_5gsMobileIdentity<Vec<u8>>>,

    #[tlv_config(tag = 0x54, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_tai_list: Option<_5gsTrackingAreaIdentityList<Vec<u8>>>,

    #[tlv_config(tag = 0x15, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_allowed_nssai: Option<Nssai<Vec<u8>>>,

    #[tlv_config(tag = 0x27, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_service_area_list: Option<ServiceAreaList<Vec<u8>>>,

    #[tlv_config(tag = 0x43, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_full_name_for_network: Option<NetworkName<Vec<u8>>>,

    #[tlv_config(tag = 0x45, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_short_name_for_network: Option<NetworkName<Vec<u8>>>,

    #[tlv_config(tag = 0x46, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_local_time_zone: Option<TimeZone>,

    #[tlv_config(tag = 0x47, tag_byte_format = 1, length = 7, length_byte_format = 0, format = "TV")]
    nas_universal_time_and_local_time_zone: Option<TimeZoneAndTime<Vec<u8>>>,

    #[tlv_config(tag = 0x49, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_network_daylight_saving_time: Option<DaylightSavingTime>,

    #[tlv_config(tag = 0x79, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_ladn_information: Option<LadnInformation<Vec<u8>>>,

    #[tlv_config(tag = 0xB-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_mico_indication: Option<MicoIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x9-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_network_slicing_indication: Option<NetworkSlicingIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x31, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_configured_nssai: Option<Nssai<Vec<u8>>>,

    #[tlv_config(tag = 0x11, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_rejected_nssai: Option<RejectedNssai<Vec<u8>>>,

    #[tlv_config(tag = 0x76, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_operator_defined_access_category_definitions: Option<OperatorDefinedAccessCategoryDefinitions<Vec<u8>>>,

    #[tlv_config(tag = 0xF-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_sms_indication: Option<SmsIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x6C, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_t3447_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x75, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_cag_information_list: Option<CagInformationList<Vec<u8>>>,

    #[tlv_config(tag = 0x67, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_ue_radio_capability_id: Option<UeRadioCapabilityId<Vec<u8>>>,

    #[tlv_config(tag = 0xA-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_ue_radio_capability_id_deletion_indication: Option<UeRadioCapabilityIdDeletionIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x44, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_5gs_registration_result: Option<_5gsRegistrationResult>,

    #[tlv_config(tag = 0x1B, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_truncated_5g_s_tmsi_configuration: Option<Truncated5gSTmsiConfiguration>,

    #[tlv_config(tag = 0xC-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_additional_configuration_indication: Option<AdditionalConfigurationIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x68, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_extended_rejected_nssai: Option<ExtendedRejectedNssai<Vec<u8>>>,

    #[tlv_config(tag = 0x72, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_service_level_aa_container: Option<ServiceLevelAaContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x70, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_nssrg_information: Option<NssrgInformation<Vec<u8>>>,

    #[tlv_config(tag = 0x14, tag_byte_format = 1, length = 2, length_byte_format = 1, format = "TLV")]
    nas_disaster_roaming_wait_range: Option<RegistrationWaitRange<Vec<u8>>>,

    #[tlv_config(tag = 0x2C, tag_byte_format = 1, length = 2, length_byte_format = 1, format = "TLV")]
    nas_disaster_return_wait_range: Option<RegistrationWaitRange<Vec<u8>>>,

    #[tlv_config(tag = 0x13, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_list_of_plmns_to_be_used_in_disaster_condition: Option<ListOfPlmnsToBeUsedInDisasterCondition<Vec<u8>>>,

    #[tlv_config(tag = 0x71, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_cag_information_list: Option<ExtendedCagInformationList<Vec<u8>>>,

    #[tlv_config(tag = 0x1F, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_updated_peips_assistance_information: Option<PeipsAssistanceInformation<Vec<u8>>>,

    #[tlv_config(tag = 0x73, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_nsag_information: Option<NsagInformation<Vec<u8>>>,

    #[tlv_config(tag = 0xE-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_priority_indicator: Option<PriorityIndicator<Vec<u8>>>,

}


/*******************************************************
 * CONFIGURATION UPDATE COMPLETE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasConfigurationUpdateComplete {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_configuration_update_complete_message_identity: MessageType,

}


/*******************************************************
 * AUTHENTICATION REQUEST
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasAuthenticationRequest {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_authentication_request_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_ngksi: KeySetIdentifier,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet_1: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 1, format = "LV")]
    nas_abba: Abba<Vec<u8>>,


    /* Optional fields */
    #[tlv_config(tag = 0x21, tag_byte_format = 1, length = 16, length_byte_format = 0, format = "TV")]
    nas_authentication_parameter_rand: Option<AuthenticationParameterRand<Vec<u8>>>,

    #[tlv_config(tag = 0x20, tag_byte_format = 1, length = 16, length_byte_format = 1, format = "TLV")]
    nas_authentication_parameter_autn: Option<AuthenticationParameterAutn<Vec<u8>>>,

    #[tlv_config(tag = 0x78, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_eap_message: Option<EapMessage<Vec<u8>>>,

}


/*******************************************************
 * AUTHENTICATION RESPONSE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasAuthenticationResponse {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_authentication_response_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x2D, tag_byte_format = 1, length = 16, length_byte_format = 1, format = "TLV")]
    nas_authentication_response_parameter: Option<AuthenticationResponseParameter<Vec<u8>>>,

    #[tlv_config(tag = 0x78, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_eap_message: Option<EapMessage<Vec<u8>>>,

}


/*******************************************************
 * AUTHENTICATION REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasAuthenticationReject {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_authentication_reject_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x78, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_eap_message: Option<EapMessage<Vec<u8>>>,

}


/*******************************************************
 * AUTHENTICATION FAILURE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasAuthenticationFailure {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_authentication_failure_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gmm_cause: _5gmmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x30, tag_byte_format = 1, length = 14, length_byte_format = 1, format = "TLV")]
    nas_authentication_failure_parameter: Option<AuthenticationFailureParameter<Vec<u8>>>,

}


/*******************************************************
 * AUTHENTICATION RESULT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasAuthenticationResult {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_authentication_result_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_ngksi: KeySetIdentifier,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet_1: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 2, format = "LV-E")]
    nas_eap_message: EapMessage<Vec<u8>>,


    /* Optional fields */
    #[tlv_config(tag = 0x38, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_abba: Option<Abba<Vec<u8>>>,

}


/*******************************************************
 * IDENTITY REQUEST
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasIdentityRequest {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_identity_request_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_identity_type: _5gsIdentityType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet_1: SpareHalfOctet,

}


/*******************************************************
 * IDENTITY RESPONSE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasIdentityResponse {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_identity_response_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 2, format = "LV-E")]
    nas_mobile_identity: _5gsMobileIdentity<Vec<u8>>,

}


/*******************************************************
 * SECURITY MODE COMMAND
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasSecurityModeCommand {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_mode_command_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_selected_nas_security_algorithms: SecurityAlgorithms,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_ngksi: KeySetIdentifier,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet_1: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 1, format = "LV")]
    nas_replayed_ue_security_capabilities: UeSecurityCapability<Vec<u8>>,


    /* Optional fields */
    #[tlv_config(tag = 0xE-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_imeisv_request: Option<ImeisvRequest<Vec<u8>>>,

    #[tlv_config(tag = 0x57, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_selected_eps_nas_security_algorithms: Option<EpsNasSecurityAlgorithms>,

    #[tlv_config(tag = 0x36, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_additional_5g_security_information: Option<Additional5gSecurityInformation>,

    #[tlv_config(tag = 0x78, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_eap_message: Option<EapMessage<Vec<u8>>>,

    #[tlv_config(tag = 0x38, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_abba: Option<Abba<Vec<u8>>>,

    #[tlv_config(tag = 0x19, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_replayed_s1_ue_security_capabilities: Option<S1UeSecurityCapability<Vec<u8>>>,

}


/*******************************************************
 * SECURITY MODE COMPLETE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasSecurityModeComplete {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_mode_complete_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x77, tag_byte_format = 1, length = 9, length_byte_format = 2, format = "TLV-E")]
    nas_imeisv: Option<_5gsMobileIdentity<Vec<u8>>>,

    #[tlv_config(tag = 0x71, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_nas_message_container: Option<MessageContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x78, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_non_imeisv_pei: Option<_5gsMobileIdentity<Vec<u8>>>,

}


/*******************************************************
 * SECURITY MODE REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasSecurityModeReject {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_mode_reject_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gmm_cause: _5gmmCause,

}


/*******************************************************
 * 5GMM STATUS
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct Nas5gmmStatus {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gmm_status_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gmm_cause: _5gmmCause,

}


/*******************************************************
 * NOTIFICATION
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasNotification {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_notification_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_access_type: AccessType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet_1: SpareHalfOctet,

}


/*******************************************************
 * NOTIFICATION RESPONSE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasNotificationResponse {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_notification_response_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x50, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_pdu_session_status: Option<PduSessionStatus<Vec<u8>>>,

}


/*******************************************************
 * UL NAS TRANSPORT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasUlNasTransport {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_ul_nas_transport_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_payload_container_type: PayloadContainerType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet_1: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 2, format = "LV-E")]
    nas_payload_container: PayloadContainer<Vec<u8>>,


    /* Optional fields */
    #[tlv_config(tag = 0x12, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_pdu_session_id: Option<PduSessionIdentity2>,

    #[tlv_config(tag = 0x59, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_old_pdu_session_id: Option<PduSessionIdentity2>,

    #[tlv_config(tag = 0x8-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_request_type: Option<RequestType<Vec<u8>>>,

    #[tlv_config(tag = 0x22, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_s_nssai: Option<SNssai<Vec<u8>>>,

    #[tlv_config(tag = 0x25, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_dnn: Option<Dnn<Vec<u8>>>,

    #[tlv_config(tag = 0x24, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_additional_information: Option<AdditionalInformation<Vec<u8>>>,

    #[tlv_config(tag = 0xA-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_ma_pdu_session_information: Option<MaPduSessionInformation<Vec<u8>>>,

    #[tlv_config(tag = 0xF-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_release_assistance_indication: Option<ReleaseAssistanceIndication<Vec<u8>>>,

}


/*******************************************************
 * DL NAS TRANSPORT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasDlNasTransport {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_security_header_type: SecurityHeaderType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_dl_nas_transport_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_payload_container_type: PayloadContainerType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_spare_half_octet_1: SpareHalfOctet,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 2, format = "LV-E")]
    nas_payload_container: PayloadContainer<Vec<u8>>,


    /* Optional fields */
    #[tlv_config(tag = 0x12, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_pdu_session_id: Option<PduSessionIdentity2>,

    #[tlv_config(tag = 0x24, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_additional_information: Option<AdditionalInformation<Vec<u8>>>,

    #[tlv_config(tag = 0x58, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_5gmm_cause: Option<_5gmmCause>,

    #[tlv_config(tag = 0x37, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_back_off_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x3A, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_lower_bound_timer_value: Option<GprsTimer3>,

}


/*******************************************************
 * PDU SESSION ESTABLISHMENT REQUEST
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionEstablishmentRequest {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_establishment_request_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_integrity_protection_maximum_data_rate: IntegrityProtectionMaximumDataRate<Vec<u8>>,


    /* Optional fields */
    #[tlv_config(tag = 0x9-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_pdu_session_type: Option<PduSessionType<Vec<u8>>>,

    #[tlv_config(tag = 0xA-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_ssc_mode: Option<SscMode<Vec<u8>>>,

    #[tlv_config(tag = 0x28, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_5gsm_capability: Option<_5gsmCapability<Vec<u8>>>,

    #[tlv_config(tag = 0x55, tag_byte_format = 1, length = 2, length_byte_format = 0, format = "TV")]
    nas_maximum_number_of_supported_packet_filters: Option<MaximumNumberOfSupportedPacketFilters<Vec<u8>>>,

    #[tlv_config(tag = 0xB-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_always_on_pdu_session_requested: Option<AlwaysOnPduSessionRequested<Vec<u8>>>,

    #[tlv_config(tag = 0x39, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_sm_pdu_dn_request_container: Option<SmPduDnRequestContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

    #[tlv_config(tag = 0x66, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_ip_header_compression_configuration: Option<IpHeaderCompressionConfiguration<Vec<u8>>>,

    #[tlv_config(tag = 0x6E, tag_byte_format = 1, length = 6, length_byte_format = 1, format = "TLV")]
    nas_ds_tt_ethernet_port_mac_address: Option<DsTtEthernetPortMacAddress<Vec<u8>>>,

    #[tlv_config(tag = 0x6F, tag_byte_format = 1, length = 8, length_byte_format = 1, format = "TLV")]
    nas_ue_ds_tt_residence_time: Option<UeDsTtResidenceTime<Vec<u8>>>,

    #[tlv_config(tag = 0x74, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_port_management_information_container: Option<PortManagementInformationContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x1F, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_ethernet_header_compression_configuration: Option<EthernetHeaderCompressionConfiguration>,

    #[tlv_config(tag = 0x29, tag_byte_format = 1, length = 9, length_byte_format = 1, format = "TLV")]
    nas_suggested_interface_identifier: Option<PduAddress<Vec<u8>>>,

    #[tlv_config(tag = 0x72, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_service_level_aa_container: Option<ServiceLevelAaContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x70, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_requested_mbs_container: Option<RequestedMbsContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x34, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_pdu_session_pair_id: Option<PduSessionPairId>,

    #[tlv_config(tag = 0x35, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_rsn: Option<Rsn>,

}


/*******************************************************
 * PDU SESSION ESTABLISHMENT ACCEPT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionEstablishmentAccept {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_establishment_accept_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_selected_pdu_session_type: PduSessionType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_selected_ssc_mode: SscMode,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 2, format = "LV-E")]
    nas_authorized_qos_rules: QosRules<Vec<u8>>,

    #[tlv_config(tag_byte_format = 0, length = 6, length_byte_format = 1, format = "LV")]
    nas_session_ambr: SessionAmbr<Vec<u8>>,


    /* Optional fields */
    #[tlv_config(tag = 0x59, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_5gsm_cause: Option<_5gsmCause>,

    #[tlv_config(tag = 0x29, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_pdu_address: Option<PduAddress<Vec<u8>>>,

    #[tlv_config(tag = 0x56, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_rq_timer_value: Option<GprsTimer>,

    #[tlv_config(tag = 0x22, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_s_nssai: Option<SNssai<Vec<u8>>>,

    #[tlv_config(tag = 0x8-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_always_on_pdu_session_indication: Option<AlwaysOnPduSessionIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x75, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_mapped_eps_bearer_contexts: Option<MappedEpsBearerContexts<Vec<u8>>>,

    #[tlv_config(tag = 0x78, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_eap_message: Option<EapMessage<Vec<u8>>>,

    #[tlv_config(tag = 0x79, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_authorized_qos_flow_descriptions: Option<QosFlowDescriptions<Vec<u8>>>,

    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

    #[tlv_config(tag = 0x25, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_dnn: Option<Dnn<Vec<u8>>>,

    #[tlv_config(tag = 0x17, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_5gsm_network_feature_support: Option<_5gsmNetworkFeatureSupport<Vec<u8>>>,

    #[tlv_config(tag = 0x18, tag_byte_format = 1, length = 2, length_byte_format = 1, format = "TLV")]
    nas_serving_plmn_rate_control: Option<ServingPlmnRateControl<Vec<u8>>>,

    #[tlv_config(tag = 0x77, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_atsss_container: Option<AtsssContainer<Vec<u8>>>,

    #[tlv_config(tag = 0xC-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_control_plane_only_indication: Option<ControlPlaneOnlyIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x66, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_ip_header_compression_configuration: Option<IpHeaderCompressionConfiguration<Vec<u8>>>,

    #[tlv_config(tag = 0x1F, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_ethernet_header_compression_configuration: Option<EthernetHeaderCompressionConfiguration>,

    #[tlv_config(tag = 0x72, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_service_level_aa_container: Option<ServiceLevelAaContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x71, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_received_mbs_container: Option<ReceivedMbsContainer<Vec<u8>>>,

}


/*******************************************************
 * PDU SESSION ESTABLISHMENT REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionEstablishmentReject {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_establishment_reject_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gsm_cause: _5gsmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x37, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_back_off_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0xF-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_allowed_ssc_mode: Option<AllowedSscMode<Vec<u8>>>,

    #[tlv_config(tag = 0x78, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_eap_message: Option<EapMessage<Vec<u8>>>,

    #[tlv_config(tag = 0x61, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_5gsm_congestion_re_attempt_indicator: Option<_5gsmCongestionReAttemptIndicator>,

    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

    #[tlv_config(tag = 0x1D, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_re_attempt_indicator: Option<ReAttemptIndicator>,

    #[tlv_config(tag = 0x72, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_service_level_aa_container: Option<ServiceLevelAaContainer<Vec<u8>>>,

}


/*******************************************************
 * PDU SESSION AUTHENTICATION COMMAND
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionAuthenticationCommand {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_authentication_command_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 2, format = "LV-E")]
    nas_eap_message: EapMessage<Vec<u8>>,


    /* Optional fields */
    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

}


/*******************************************************
 * PDU SESSION AUTHENTICATION COMPLETE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionAuthenticationComplete {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_authentication_complete_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 2, format = "LV-E")]
    nas_eap_message: EapMessage<Vec<u8>>,


    /* Optional fields */
    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

}


/*******************************************************
 * PDU SESSION AUTHENTICATION RESULT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionAuthenticationResult {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_authentication_result_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x78, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_eap_message: Option<EapMessage<Vec<u8>>>,

    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

}


/*******************************************************
 * PDU SESSION MODIFICATION REQUEST
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionModificationRequest {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_modification_request_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x28, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_5gsm_capability: Option<_5gsmCapability<Vec<u8>>>,

    #[tlv_config(tag = 0x59, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_5gsm_cause: Option<_5gsmCause>,

    #[tlv_config(tag = 0x55, tag_byte_format = 1, length = 2, length_byte_format = 0, format = "TV")]
    nas_maximum_number_of_supported_packet_filters: Option<MaximumNumberOfSupportedPacketFilters<Vec<u8>>>,

    #[tlv_config(tag = 0xB-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_always_on_pdu_session_requested: Option<AlwaysOnPduSessionRequested<Vec<u8>>>,

    #[tlv_config(tag = 0x13, tag_byte_format = 1, length = 2, length_byte_format = 0, format = "TV")]
    nas_integrity_protection_maximum_data_rate: Option<IntegrityProtectionMaximumDataRate<Vec<u8>>>,

    #[tlv_config(tag = 0x7A, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_requested_qos_rules: Option<QosRules<Vec<u8>>>,

    #[tlv_config(tag = 0x79, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_requested_qos_flow_descriptions: Option<QosFlowDescriptions<Vec<u8>>>,

    #[tlv_config(tag = 0x75, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_mapped_eps_bearer_contexts: Option<MappedEpsBearerContexts<Vec<u8>>>,

    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

    #[tlv_config(tag = 0x74, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_port_management_information_container: Option<PortManagementInformationContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x66, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_ip_header_compression_configuration: Option<HeaderCompressionConfiguration<Vec<u8>>>,

    #[tlv_config(tag = 0x1F, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_ethernet_header_compression_configuration: Option<EthernetHeaderCompressionConfiguration>,

    #[tlv_config(tag = 0x70, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_requested_mbs_container: Option<RequestedMbsContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x72, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_service_level_aa_container: Option<ServiceLevelAaContainer<Vec<u8>>>,

}


/*******************************************************
 * PDU SESSION MODIFICATION REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionModificationReject {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_modification_reject_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gsm_cause: _5gsmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x37, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_back_off_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x61, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_5gsm_congestion_re_attempt_indicator: Option<_5gsmCongestionReAttemptIndicator>,

    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

    #[tlv_config(tag = 0x1D, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_re_attempt_indicator: Option<ReAttemptIndicator>,

}


/*******************************************************
 * PDU SESSION MODIFICATION COMMAND
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionModificationCommand {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_modification_command_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x59, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_5gsm_cause: Option<_5gsmCause>,

    #[tlv_config(tag = 0x2A, tag_byte_format = 1, length = 6, length_byte_format = 1, format = "TLV")]
    nas_session_ambr: Option<SessionAmbr<Vec<u8>>>,

    #[tlv_config(tag = 0x56, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_rq_timer_value: Option<GprsTimer>,

    #[tlv_config(tag = 0x8-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_always_on_pdu_session_indication: Option<AlwaysOnPduSessionIndication<Vec<u8>>>,

    #[tlv_config(tag = 0x7A, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_authorized_qos_rules: Option<QosRules<Vec<u8>>>,

    #[tlv_config(tag = 0x75, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_mapped_eps_bearer_contexts: Option<MappedEpsBearerContexts<Vec<u8>>>,

    #[tlv_config(tag = 0x79, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_authorized_qos_flow_descriptions: Option<QosFlowDescriptions<Vec<u8>>>,

    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

    #[tlv_config(tag = 0x77, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_atsss_container: Option<AtsssContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x66, tag_byte_format = 1, length_byte_format = 1, format = "TLV")]
    nas_ip_header_compression_configuration: Option<IpHeaderCompressionConfiguration<Vec<u8>>>,

    #[tlv_config(tag = 0x74, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_port_management_information_container: Option<PortManagementInformationContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x1E, tag_byte_format = 1, length = 2, length_byte_format = 1, format = "TLV")]
    nas_serving_plmn_rate_control: Option<ServingPlmnRateControl<Vec<u8>>>,

    #[tlv_config(tag = 0x1F, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_ethernet_header_compression_configuration: Option<EthernetHeaderCompressionConfiguration>,

    #[tlv_config(tag = 0x71, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_received_mbs_container: Option<ReceivedMbsContainer<Vec<u8>>>,

    #[tlv_config(tag = 0x72, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_service_level_aa_container: Option<ServiceLevelAaContainer<Vec<u8>>>,

}


/*******************************************************
 * PDU SESSION MODIFICATION COMPLETE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionModificationComplete {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_modification_complete_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

    #[tlv_config(tag = 0x74, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_port_management_information_container: Option<PortManagementInformationContainer<Vec<u8>>>,

}


/*******************************************************
 * PDU SESSION MODIFICATION COMMAND REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionModificationCommandReject {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_modification_command_reject_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gsm_cause: _5gsmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

}


/*******************************************************
 * PDU SESSION RELEASE REQUEST
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionReleaseRequest {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_release_request_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x59, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_5gsm_cause: Option<_5gsmCause>,

    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

}


/*******************************************************
 * PDU SESSION RELEASE REJECT
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionReleaseReject {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_release_reject_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gsm_cause: _5gsmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

}


/*******************************************************
 * PDU SESSION RELEASE COMMAND
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionReleaseCommand {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_release_command_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gsm_cause: _5gsmCause,


    /* Optional fields */
    #[tlv_config(tag = 0x37, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_back_off_timer_value: Option<GprsTimer3>,

    #[tlv_config(tag = 0x78, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_eap_message: Option<EapMessage<Vec<u8>>>,

    #[tlv_config(tag = 0x61, tag_byte_format = 1, length = 1, length_byte_format = 1, format = "TLV")]
    nas_5gsm_congestion_re_attempt_indicator: Option<_5gsmCongestionReAttemptIndicator>,

    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

    #[tlv_config(tag = 0xD-, tag_byte_format = 1, length = 0, length_byte_format = 0, format = "TV")]
    nas_access_type: Option<AccessType<Vec<u8>>>,

    #[tlv_config(tag = 0x72, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_service_level_aa_container: Option<ServiceLevelAaContainer<Vec<u8>>>,

}


/*******************************************************
 * PDU SESSION RELEASE COMPLETE
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct NasPduSessionReleaseComplete {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_release_complete_message_identity: MessageType,


    /* Optional fields */
    #[tlv_config(tag = 0x59, tag_byte_format = 1, length = 1, length_byte_format = 0, format = "TV")]
    nas_5gsm_cause: Option<_5gsmCause>,

    #[tlv_config(tag = 0x7B, tag_byte_format = 1, length_byte_format = 2, format = "TLV-E")]
    nas_extended_protocol_configuration_options: Option<ExtendedProtocolConfigurationOptions<Vec<u8>>>,

}


/*******************************************************
 * 5GSM STATUS
 ******************************************************/

#[derive(Debug, TlvEncode, TlvDecode)]
pub struct Nas5gsmStatus {
    /* Mandatory fields */
    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pdu_session_id: PduSessionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_pti: ProcedureTransactionIdentity,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gsm_status_message_identity: MessageType,

    #[tlv_config(tag_byte_format = 0, length_byte_format = 0, format = "V")]
    nas_5gsm_cause: _5gsmCause,

}


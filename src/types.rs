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
 * Created on: 2025-03-08 22:04:56.581152 by nr
 * from 24501-h90.docx
 ******************************************************************************/


use bitfield::bitfield;
use tlv::prelude::*;
use tlv_derive::{TlvDecode, TlvEncode};
use derive_more::{Into, From};


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ExtendedProtocolDiscriminator(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SecurityHeaderType(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SpareHalfOctet(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MessageType(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsRegistrationType(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct KeySetIdentifier(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsMobileIdentity(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegmmCapability(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeSecurityCapability(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Nssai(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsTrackingAreaIdentity(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct S1UeNetworkCapability(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UplinkDataStatus(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionStatus(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MicoIndication(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeStatus(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AllowedPduSessionStatus(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeUsageSetting(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsDrxParameters(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct EpsNasMessageContainer(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct LadnIndication(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PayloadContainerType(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PayloadContainer(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct NetworkSlicingIndication(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsUpdateType(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MobileStationClassmark2(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SupportedCodecList(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MessageContainer(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct EpsBearerContextStatus(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ExtendedDrxParameters(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct GprsTimer3(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeRadioCapabilityId(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MappedNssai(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AdditionalInformationRequested(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct WusAssistanceInformation(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct N5gcIndication(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct NbN1ModeDrxParameters(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeRequestType(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PagingRestriction(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ServiceLevelAaContainer(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Nid(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PlmnIdentity(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PeipsAssistanceInformation(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsRegistrationResult(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PlmnList(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsTrackingAreaIdentityList(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct RejectedNssai(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsNetworkFeatureSupport(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionReactivationResult(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionReactivationResultErrorCause(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct LadnInformation(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ServiceAreaList(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct GprsTimer2(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct EmergencyNumberList(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ExtendedEmergencyNumberList(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SorTransparentContainer(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct EapMessage(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct NssaiInclusionMode(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct OperatorDefinedAccessCategoryDefinitions(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Non3gppNwProvidedPolicies(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeRadioCapabilityIdDeletionIndication(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct CipheringKeyData(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct CagInformationList(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Truncated5gSTmsiConfiguration(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ExtendedRejectedNssai(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsAdditionalRequestResult(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct NssrgInformation(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct RegistrationWaitRange(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ListOfPlmnsToBeUsedInDisasterCondition(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ExtendedCagInformationList(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct NsagInformation(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegmmCause(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct DeRegistrationType(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ServiceType(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ConfigurationUpdateIndication(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct NetworkName(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct TimeZone(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct TimeZoneAndTime(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct DaylightSavingTime(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SmsIndication(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AdditionalConfigurationIndication(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PriorityIndicator(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Abba(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AuthenticationParameterRand(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AuthenticationParameterAutn(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AuthenticationResponseParameter(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AuthenticationFailureParameter(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsIdentityType(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SecurityAlgorithms(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ImeisvRequest(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct EpsNasSecurityAlgorithms(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Additional5gSecurityInformation(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct S1UeSecurityCapability(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AccessType(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionIdentity2(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct RequestType(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SNssai(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Dnn(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AdditionalInformation(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MaPduSessionInformation(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ReleaseAssistanceIndication(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionIdentity(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ProcedureTransactionIdentity(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct IntegrityProtectionMaximumDataRate(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionType(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SscMode(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsmCapability(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MaximumNumberOfSupportedPacketFilters(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AlwaysOnPduSessionRequested(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SmPduDnRequestContainer(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ExtendedProtocolConfigurationOptions(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct IpHeaderCompressionConfiguration(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct DsTtEthernetPortMacAddress(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct UeDsTtResidenceTime(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PortManagementInformationContainer(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct EthernetHeaderCompressionConfiguration(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduAddress(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct RequestedMbsContainer(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct PduSessionPairId(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct Rsn(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct QosRules(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct SessionAmbr(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsmCause(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct GprsTimer(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AlwaysOnPduSessionIndication(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct MappedEpsBearerContexts(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct QosFlowDescriptions(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsmNetworkFeatureSupport(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ServingPlmnRateControl(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AtsssContainer(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ControlPlaneOnlyIndication(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ReceivedMbsContainer(Vec<u8>);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct AllowedSscMode(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct FivegsmCongestionReAttemptIndicator(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct ReAttemptIndicator(u8);


#[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
pub struct HeaderCompressionConfiguration(Vec<u8>);

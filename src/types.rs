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
 * Created on: 2025-01-04 03:46:59.123898 by nr
 * from
 ******************************************************************************/
use bitfield::bitfield;
use derive_deref::Deref;


bitfield! {
   #[derive(Deref)]
   pub struct ExtendedProtocolDiscriminator(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct SecurityHeaderType(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct SpareHalfOctet(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct MessageType(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsRegistrationType(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct KeySetIdentifier(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsMobileIdentity(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gmmCapability(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct UeSecurityCapability(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct Nssai(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsTrackingAreaIdentity(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct S1UeNetworkCapability(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct UplinkDataStatus(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PduSessionStatus(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct MicoIndication(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct UeStatus(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct AllowedPduSessionStatus(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct UeUsageSetting(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsDrxParameters(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct EpsNasMessageContainer(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct LadnIndication(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PayloadContainerType(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PayloadContainer(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct NetworkSlicingIndication(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsUpdateType(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct MobileStationClassmark2(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct SupportedCodecList(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct MessageContainer(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct EpsBearerContextStatus(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ExtendedDrxParameters(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct GprsTimer3(u8);
   impl Debug;
   u8;
   pub get_binary_coded_timer, set_binary_coded_timer: 4, 0;
   // todo add enum
   get_value_unit, set_value_unit: 7, 5;

}

bitfield! {
   #[derive(Deref)]
   pub struct UeRadioCapabilityId(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct MappedNssai(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct AdditionalInformationRequested(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct WusAssistanceInformation(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct N5gcIndication(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct NbN1ModeDrxParameters(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct UeRequestType(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PagingRestriction(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ServiceLevelAaContainer(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct Nid(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PlmnIdentity(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PeipsAssistanceInformation(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsRegistrationResult(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PlmnList(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsTrackingAreaIdentityList(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct RejectedNssai(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsNetworkFeatureSupport(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PduSessionReactivationResult(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PduSessionReactivationResultErrorCause(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct LadnInformation(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ServiceAreaList(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct GprsTimer2(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct EmergencyNumberList(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ExtendedEmergencyNumberList(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct SorTransparentContainer(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct EapMessage(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct NssaiInclusionMode(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct OperatorDefinedAccessCategoryDefinitions(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct Non3gppNwProvidedPolicies(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct UeRadioCapabilityIdDeletionIndication(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct CipheringKeyData(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct CagInformationList(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct Truncated5gSTmsiConfiguration(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ExtendedRejectedNssai(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsAdditionalRequestResult(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct NssrgInformation(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct RegistrationWaitRange(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ListOfPlmnsToBeUsedInDisasterCondition(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ExtendedCagInformationList(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct NsagInformation(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gmmCause(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct DeRegistrationType(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ServiceType(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ConfigurationUpdateIndication(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct NetworkName(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct TimeZone(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct TimeZoneAndTime(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct DaylightSavingTime(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct SmsIndication(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct AdditionalConfigurationIndication(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PriorityIndicator(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct Abba(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct AuthenticationParameterRand(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct AuthenticationParameterAutn(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct AuthenticationResponseParameter(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct AuthenticationFailureParameter(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsIdentityType(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct SecurityAlgorithms(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ImeisvRequest(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct EpsNasSecurityAlgorithms(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct Additional5gSecurityInformation(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct S1UeSecurityCapability(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct AccessType(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PduSessionIdentity2(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct RequestType(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct SNssai(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct Dnn(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct AdditionalInformation(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct MaPduSessionInformation(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ReleaseAssistanceIndication(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PduSessionIdentity(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ProcedureTransactionIdentity(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct IntegrityProtectionMaximumDataRate(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PduSessionType(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct SscMode(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsmCapability(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct MaximumNumberOfSupportedPacketFilters(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct AlwaysOnPduSessionRequested(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct SmPduDnRequestContainer(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ExtendedProtocolConfigurationOptions(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct IpHeaderCompressionConfiguration(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct DsTtEthernetPortMacAddress(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct UeDsTtResidenceTime(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PortManagementInformationContainer(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct EthernetHeaderCompressionConfiguration(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PduAddress(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct RequestedMbsContainer(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct PduSessionPairId(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct Rsn(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct QosRules(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct SessionAmbr(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsmCause(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct GprsTimer(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct AlwaysOnPduSessionIndication(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct MappedEpsBearerContexts(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct QosFlowDescriptions(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsmNetworkFeatureSupport(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ServingPlmnRateControl(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct AtsssContainer(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ControlPlaneOnlyIndication(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ReceivedMbsContainer(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct AllowedSscMode(MSB0 [u8]);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct _5gsmCongestionReAttemptIndicator(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct ReAttemptIndicator(u8);
   impl Debug;
   u8;
}

bitfield! {
   #[derive(Deref)]
   pub struct HeaderCompressionConfiguration(MSB0 [u8]);
   impl Debug;
   u8;
}


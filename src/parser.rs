use tlv::{prelude::TlvDecode, Bytes};

use crate::{error::NasParsingError, message::*};
pub enum GmmMessage{
    RegistrationRequest(NasRegistrationRequest),
	RegistrationAccept(NasRegistrationAccept),
	RegistrationComplete(NasRegistrationComplete),
	RegistrationReject(NasRegistrationReject),

	AuthenticationRequest(NasAuthenticationRequest),
	AuthenticationResponse(NasAuthenticationResponse),
	AuthenticationResult(NasAuthenticationResult),
	AuthenticationFailure(NasAuthenticationFailure),
	AuthenticationReject(NasAuthenticationReject),

	SecurityModeCommand(NasSecurityModeCommand),
	SecurityModeComplete(NasSecurityModeComplete),
	SecurityModeReject(NasSecurityModeReject),

	ServiceRequest(NasServiceRequest),
	ServiceAccept(NasServiceAccept),
	ServiceReject(NasServiceReject),

	DeRegistrationRequestFromUe(NasDeregistrationRequestFromUe),
	DeRegistrationAcceptFromUe(NasDeregistrationAcceptFromUe),
	DeRegistrationRequestToUe(NasDeregistrationRequestToUe),
	DeRegistrationAcceptToUe(NasDeregistrationAcceptToUe),

	ConfigurationUpdateCommand(NasConfigurationUpdateCommand),
	ConfigurationUpdateComplete(NasConfigurationUpdateComplete),

	IdentityRequest(NasIdentityRequest),
	IdentityResponse(NasIdentityResponse),

	UlNasTransport(NasUlNasTransport),
	DlNasTransport(NasDlNasTransport),

	GmmStatus(Nas5gmmStatus),
}

impl TryFrom<&mut Bytes> for GmmMessage{
    type Error = NasParsingError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, Self::Error> {
        if bytes.len() < 3 {
            return Err(NasParsingError::InvalidNasPdu);
        }

        match bytes[2] {

            NAS_MESSAGE_TYPE_REGISTRATION_REQUEST => {
                Ok(GmmMessage::RegistrationRequest(NasRegistrationRequest::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_REGISTRATION_ACCEPT => {
                Ok(GmmMessage::RegistrationAccept(NasRegistrationAccept::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_REGISTRATION_COMPLETE => {
                Ok(GmmMessage::RegistrationComplete(NasRegistrationComplete::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_REGISTRATION_REJECT => {
                Ok(GmmMessage::RegistrationReject(NasRegistrationReject::decode(bytes.len(), bytes)?))
            }

            NAS_MESSAGE_TYPE_AUTHENTICATION_REQUEST => {
                Ok(GmmMessage::AuthenticationRequest(NasAuthenticationRequest::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_AUTHENTICATION_RESPONSE => {
                Ok(GmmMessage::AuthenticationResponse(NasAuthenticationResponse::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_AUTHENTICATION_RESULT => {
                Ok(GmmMessage::AuthenticationResult(NasAuthenticationResult::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_AUTHENTICATION_FAILURE => {
                Ok(GmmMessage::AuthenticationFailure(NasAuthenticationFailure::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_AUTHENTICATION_REJECT => {
                Ok(GmmMessage::AuthenticationReject(NasAuthenticationReject::decode(bytes.len(), bytes)?))
            }

            NAS_MESSAGE_TYPE_SECURITY_MODE_COMMAND => {
                Ok(GmmMessage::SecurityModeCommand(NasSecurityModeCommand::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_SECURITY_MODE_COMPLETE => {
                Ok(GmmMessage::SecurityModeComplete(NasSecurityModeComplete::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_SECURITY_MODE_REJECT => {
                Ok(GmmMessage::SecurityModeReject(NasSecurityModeReject::decode(bytes.len(), bytes)?))
            }

            NAS_MESSAGE_TYPE_SERVICE_REQUEST => {
                Ok(GmmMessage::ServiceRequest(NasServiceRequest::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_SERVICE_ACCEPT => {
                Ok(GmmMessage::ServiceAccept(NasServiceAccept::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_SERVICE_REJECT => {
                Ok(GmmMessage::ServiceReject(NasServiceReject::decode(bytes.len(), bytes)?))
            }

            NAS_MESSAGE_TYPE_DEREGISTRATION_REQUEST_FROM_UE => {
                Ok(GmmMessage::DeRegistrationRequestFromUe(NasDeregistrationRequestFromUe::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_DEREGISTRATION_ACCEPT_FROM_UE => {
                Ok(GmmMessage::DeRegistrationAcceptFromUe(NasDeregistrationAcceptFromUe::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_DEREGISTRATION_REQUEST_TO_UE => {
                Ok(GmmMessage::DeRegistrationRequestToUe(NasDeregistrationRequestToUe::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_DEREGISTRATION_ACCEPT_TO_UE => {
                Ok(GmmMessage::DeRegistrationAcceptToUe(NasDeregistrationAcceptToUe::decode(bytes.len(), bytes)?))
            }
            
            NAS_MESSAGE_TYPE_CONFIGURATION_UPDATE_COMMAND => {
                Ok(GmmMessage::ConfigurationUpdateCommand(NasConfigurationUpdateCommand::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_CONFIGURATION_UPDATE_COMPLETE => {
                Ok(GmmMessage::ConfigurationUpdateComplete(NasConfigurationUpdateComplete::decode(bytes.len(), bytes)?))
            }

            NAS_MESSAGE_TYPE_IDENTITY_REQUEST => {
                Ok(GmmMessage::IdentityRequest(NasIdentityRequest::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_IDENTITY_RESPONSE => {
                Ok(GmmMessage::IdentityResponse(NasIdentityResponse::decode(bytes.len(), bytes)?))
            }

            NAS_MESSAGE_TYPE_UL_NAS_TRANSPORT => {
                Ok(GmmMessage::UlNasTransport(NasUlNasTransport::decode(bytes.len(), bytes)?))
            }
            NAS_MESSAGE_TYPE_DL_NAS_TRANSPORT => {
                Ok(GmmMessage::DlNasTransport(NasDlNasTransport::decode(bytes.len(), bytes)?))
            }
           
            NAS_MESSAGE_TYPE_5GMM_STATUS => {
                Ok(GmmMessage::GmmStatus(Nas5gmmStatus::decode(bytes.len(), bytes)?))
            }
            _ => Err(NasParsingError::UnknownNasPdu),
        }
    }
}


pub enum GsmMessage{
    PduSessionEstablishmentRequest(NasPduSessionEstablishmentRequest),
    PduSessionEstablishmentAccept(NasPduSessionEstablishmentAccept),
    PduSessionEstablishmentReject(NasPduSessionEstablishmentReject),
    
    PduSessionAuthenticationCommand(NasPduSessionAuthenticationCommand),
    PduSessionAuthenticationComplete(NasPduSessionAuthenticationComplete),
    PduSessionAuthenticationResult(NasPduSessionAuthenticationResult),
    
    PduSessionModificationRequest(NasPduSessionModificationRequest),
    PduSessionModificationReject(NasPduSessionModificationReject),
    PduSessionModificationCommand(NasPduSessionModificationCommand),
    PduSessionModificationComplete(NasPduSessionModificationComplete),
    PduSessionModificationCommandReject(NasPduSessionModificationCommandReject),
    
    PduSessionReleaseRequest(NasPduSessionReleaseRequest),
    PduSessionReleaseReject(NasPduSessionReleaseReject),
    PduSessionReleaseCommand(NasPduSessionReleaseCommand),
    PduSessionReleaseComplete(NasPduSessionReleaseComplete),
    
    GsmStatus(Nas5gsmStatus),
}

impl TryFrom<&mut Bytes> for GsmMessage{
    type Error = NasParsingError;

    fn try_from(bytes: &mut Bytes) -> Result<Self, Self::Error> {
        if bytes.len() < 3 {
            return Err(NasParsingError::InvalidNasPdu);
        }

        match bytes[2] {
            _ => {
                todo!()
            }
        }
    }
}